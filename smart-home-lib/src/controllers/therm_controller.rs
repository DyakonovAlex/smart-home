//! UDP контроллер для умного термометра

use crate::devices::SmartTherm;
use crate::protocol::{ThermData, now_ms};
use crate::traits::Reporter;
use crate::units::Celsius;
use std::collections::HashMap;
use std::fmt;
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tokio::sync::watch;

/// Ошибки контроллера
#[derive(Debug, Clone)]
pub enum ThermError {
    /// Нет свежих данных
    NoFreshData,
    /// Ошибка сети
    NetworkError(String),
    /// Ошибка блокировки
    LockError,
}

impl std::fmt::Display for ThermError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFreshData => write!(f, "Нет свежих данных"),
            Self::NetworkError(msg) => write!(f, "Сетевая ошибка: {}", msg),
            Self::LockError => write!(f, "Ошибка блокировки"),
        }
    }
}

impl std::error::Error for ThermError {}

/// Тип callback функции для уведомлений об изменениях
type TemperatureCallback = Box<dyn Fn(Result<Celsius, ThermError>) + Send + 'static>;

/// Контроллер умного термометра (UDP)
pub struct ThermController {
    /// Внутренний термометр
    therm: Arc<RwLock<SmartTherm>>,
    /// Адрес для прослушивания UDP
    listen_addr: String,
    /// Максимальный возраст данных
    max_age: Duration,
    /// Время последнего обновления (0 = нет данных, >0 = timestamp в мс)
    last_update: Arc<AtomicU64>,
    /// Флаг работы фонового потока
    running: Arc<AtomicBool>,
    /// Handle фонового потока
    thread_handle: Option<JoinHandle<()>>,
    /// Канал для уведомлений о новых данных (async)
    temp_sender: watch::Sender<Option<Result<Celsius, ThermError>>>,
    temp_receiver: watch::Receiver<Option<Result<Celsius, ThermError>>>,
    /// Список callback'ов для уведомлений об изменениях
    callbacks: Arc<Mutex<HashMap<usize, TemperatureCallback>>>,
    /// Счетчик для SubscriptionHandle
    next_callback_id: Arc<AtomicUsize>,
}

impl ThermController {
    /// Создает новый контроллер
    pub fn new(initial_temp: f64, listen_addr: &str, max_age: Duration) -> Self {
        let (temp_sender, temp_receiver) = watch::channel(None);

        Self {
            therm: Arc::new(RwLock::new(SmartTherm::new(initial_temp))),
            listen_addr: listen_addr.to_string(),
            max_age,
            last_update: Arc::new(AtomicU64::new(0)),
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
            temp_sender,
            temp_receiver,
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            next_callback_id: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Запускает автоматическое обновление в фоне
    pub fn start(&mut self) {
        if self.running.load(Ordering::Relaxed) {
            return; // Уже запущен
        }

        self.running.store(true, Ordering::Relaxed);

        let therm = Arc::clone(&self.therm);
        let last_update = Arc::clone(&self.last_update);
        let running = Arc::clone(&self.running);
        let listen_addr = self.listen_addr.clone();
        let max_age = self.max_age;
        let temp_sender = self.temp_sender.clone();
        let callbacks = Arc::clone(&self.callbacks);

        let handle = thread::spawn(move || {
            // Создаем UDP сокет для получения данных
            let socket = match UdpSocket::bind(&listen_addr) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("❌ Не удалось привязать UDP сокет {}: {}", listen_addr, e);
                    return;
                }
            };

            let mut buf = [0; 1024];

            while running.load(Ordering::Relaxed) {
                // Неблокирующее чтение
                socket.set_nonblocking(true).ok();

                match socket.recv_from(&mut buf) {
                    Ok((size, _)) => {
                        if let Ok(data_str) = std::str::from_utf8(&buf[..size]) {
                            if let Ok(therm_data) = serde_json::from_str::<ThermData>(data_str) {
                                let new_temp = Celsius::new(therm_data.temperature);

                                last_update.store(now_ms(), Ordering::Relaxed);

                                // Обновляем термометр
                                if let Ok(mut therm) = therm.write() {
                                    therm.set_temperature(therm_data.temperature);
                                }

                                // Уведомляем о новых данных
                                let result = Ok(new_temp);
                                let _ = temp_sender.send(Some(result.clone()));

                                // Уведомляем всех подписчиков (callback)
                                if let Ok(callbacks) = callbacks.lock() {
                                    for (_id, callback) in callbacks.iter() {
                                        callback(result.clone());
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // Нет данных, спим немного
                        thread::sleep(Duration::from_millis(10));

                        // Проверяем возраст данных
                        let last_timestamp = last_update.load(Ordering::Relaxed);
                        if last_timestamp != 0
                            && (now_ms() - last_timestamp) > max_age.as_millis() as u64
                        {
                            // Данные устарели - уведомляем
                            let error_result = Err(ThermError::NoFreshData);
                            let _ = temp_sender.send(Some(error_result.clone()));

                            if let Ok(callbacks) = callbacks.lock() {
                                for (_id, callback) in callbacks.iter() {
                                    callback(error_result.clone());
                                }
                            }
                        }
                    }
                }
            }
        });

        self.thread_handle = Some(handle);
    }

    /// Получает текущую температуру
    pub fn temperature(&self) -> Result<Celsius, ThermError> {
        // Проверяем возраст данных
        let last_timestamp = self.last_update.load(Ordering::Relaxed);

        if last_timestamp == 0 {
            // Нет данных
            return Err(ThermError::NoFreshData);
        }

        if (now_ms() - last_timestamp) > self.max_age.as_millis() as u64 {
            // Данные устарели
            return Err(ThermError::NoFreshData);
        }

        // Получаем температуру
        self.therm
            .read()
            .map(|therm| therm.temperature())
            .map_err(|_| ThermError::LockError)
    }

    /// Получает копию внутреннего термометра
    pub fn device(&self) -> SmartTherm {
        self.therm
            .read()
            .map(|therm| therm.clone())
            .unwrap_or_else(|_| SmartTherm::new(0.0))
    }

    /// Останавливает автоматическое обновление
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }

    /// Ждет новых данных (async)
    pub async fn wait_for_new_data(&self) -> Result<Celsius, ThermError> {
        let mut receiver = self.temp_receiver.clone();

        // Ждем изменений в канале
        match receiver.changed().await {
            Ok(_) => match receiver.borrow().clone() {
                Some(result) => result,
                None => Err(ThermError::NoFreshData),
            },
            Err(_) => Err(ThermError::NetworkError("Channel closed".to_string())),
        }
    }

    /// Подписка на изменения температуры (callback)
    pub fn on_temperature_change<F>(&self, callback: F) -> SubscriptionHandle
    where
        F: Fn(Result<Celsius, ThermError>) + Send + 'static,
    {
        let callback_id = self.next_callback_id.fetch_add(1, Ordering::Relaxed);

        if let Ok(mut callbacks) = self.callbacks.lock() {
            callbacks.insert(callback_id, Box::new(callback));
        }

        SubscriptionHandle {
            callback_id,
            callbacks: Arc::clone(&self.callbacks),
        }
    }
}

impl Drop for ThermController {
    fn drop(&mut self) {
        self.stop();
    }
}

impl Reporter for ThermController {
    fn report(&self) -> String {
        self.device().report()
    }
}

impl fmt::Display for ThermController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
    }
}

/// Handle подписки
pub struct SubscriptionHandle {
    callback_id: usize,
    callbacks: Arc<Mutex<HashMap<usize, TemperatureCallback>>>,
}

impl SubscriptionHandle {
    /// Отписывается от уведомлений
    pub fn unsubscribe(self) {
        if let Ok(mut callbacks) = self.callbacks.lock() {
            callbacks.remove(&self.callback_id);
            println!("📵 Отписка от уведомлений (ID: {})", self.callback_id);
        }
    }
}

impl Drop for SubscriptionHandle {
    fn drop(&mut self) {
        if let Ok(mut callbacks) = self.callbacks.lock() {
            callbacks.remove(&self.callback_id);
            println!(
                "📵 Автоматическая отписка от уведомлений (ID: {})",
                self.callback_id
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::UdpSocket;
    use std::thread;
    use std::time::Duration;

    fn find_free_port() -> u16 {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind to find free port");
        socket
            .local_addr()
            .expect("Failed to get local addr")
            .port()
    }

    #[test]
    fn controller_creation() {
        let port = find_free_port();
        let addr = format!("127.0.0.1:{}", port);
        let max_age = Duration::from_secs(5);

        let controller = ThermController::new(22.5, &addr, max_age);

        assert_eq!(controller.listen_addr, addr);
        assert_eq!(controller.max_age, max_age);
        assert_eq!(controller.last_update.load(Ordering::Relaxed), 0);
        assert!(!controller.running.load(Ordering::Relaxed));

        let device = controller.device();
        assert_eq!(device.temperature(), Celsius::new(22.5));
    }

    #[test]
    fn temperature_no_data_initially() {
        let port = find_free_port();
        let addr = format!("127.0.0.1:{}", port);
        let controller = ThermController::new(20.0, &addr, Duration::from_secs(1));

        let result = controller.temperature();
        assert!(matches!(result, Err(ThermError::NoFreshData)));
    }

    #[test]
    fn temperature_with_fresh_data() {
        let port = find_free_port();
        let addr = format!("127.0.0.1:{}", port);
        let controller = ThermController::new(20.0, &addr, Duration::from_secs(10));

        // Симулируем получение данных напрямую
        controller.last_update.store(now_ms(), Ordering::Relaxed);

        if let Ok(mut therm) = controller.therm.write() {
            therm.set_temperature(25.5);
        }

        let result = controller.temperature();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Celsius::new(25.5));
    }

    #[test]
    fn temperature_with_stale_data() {
        let port = find_free_port();
        let addr = format!("127.0.0.1:{}", port);
        let controller = ThermController::new(20.0, &addr, Duration::from_millis(100));

        let old_timestamp = now_ms() - 200;
        controller
            .last_update
            .store(old_timestamp, Ordering::Relaxed);

        let result = controller.temperature();
        assert!(matches!(result, Err(ThermError::NoFreshData)));
    }

    #[test]
    fn controller_start_stop_basic() {
        let port = find_free_port();
        let addr = format!("127.0.0.1:{}", port);
        let mut controller = ThermController::new(20.0, &addr, Duration::from_secs(5));

        // Изначально не запущен
        assert!(!controller.running.load(Ordering::Relaxed));

        // Запускаем
        controller.start();
        assert!(controller.running.load(Ordering::Relaxed));

        // Останавливаем БЫСТРО
        thread::sleep(Duration::from_millis(10)); // минимальная задержка
        controller.stop();
        assert!(!controller.running.load(Ordering::Relaxed));
    }

    #[test]
    fn subscription_basic() {
        let port = find_free_port();
        let addr = format!("127.0.0.1:{}", port);
        let controller = ThermController::new(20.0, &addr, Duration::from_secs(5));

        let _subscription = controller.on_temperature_change(|_| {});

        let callbacks_len = controller.callbacks.lock().map(|cb| cb.len()).unwrap_or(0);
        assert_eq!(callbacks_len, 1);
    }

    #[test]
    fn subscription_unsubscribe() {
        let port = find_free_port();
        let addr = format!("127.0.0.1:{}", port);
        let controller = ThermController::new(20.0, &addr, Duration::from_secs(5));

        let subscription = controller.on_temperature_change(|_| {});

        let callbacks_len = controller.callbacks.lock().map(|cb| cb.len()).unwrap_or(0);
        assert_eq!(callbacks_len, 1);

        subscription.unsubscribe();

        let callbacks_len = controller.callbacks.lock().map(|cb| cb.len()).unwrap_or(0);
        assert_eq!(callbacks_len, 0);
    }
}

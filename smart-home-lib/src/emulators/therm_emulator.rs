//! Простой эмулятор термометра

use super::scenario::EmulationScenario;
use crate::protocol::ThermData;
use rand::Rng;
use serde_json;
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Простой эмулятор термометра
pub struct ThermEmulator {
    initial_temp: f64,
    device_id: Option<String>,
    scenario: EmulationScenario,
    interval: Duration,
    target_addr: Option<String>,
    running: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
}

impl ThermEmulator {
    /// Создает новый эмулятор
    pub fn new(initial_temp: f64) -> Self {
        Self {
            initial_temp,
            device_id: None,
            scenario: EmulationScenario::Normal,
            interval: Duration::from_secs(1),
            target_addr: None,
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    /// Builder: устанавливает ID устройства
    pub fn with_device_id(mut self, device_id: &str) -> Self {
        self.device_id = Some(device_id.to_string());
        self
    }

    /// Builder: устанавливает сценарий
    pub fn with_scenario(mut self, scenario: EmulationScenario) -> Self {
        self.scenario = scenario;
        self
    }

    /// Builder: устанавливает интервал обновления
    pub fn with_update_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Устанавливает адрес для отправки данных
    pub fn connect_to(&mut self, addr: &str) -> Result<(), std::io::Error> {
        self.target_addr = Some(addr.to_string());
        Ok(())
    }

    /// Запускает поток эмуляции
    pub fn start(&mut self) {
        if self.running.load(Ordering::Relaxed) {
            panic!("Emulator already running!");
        }

        self.running.store(true, Ordering::Relaxed);

        let running = Arc::clone(&self.running);
        let target_addr = self.target_addr.clone();
        let device_id = self.device_id.clone();
        let scenario = self.scenario;
        let interval = self.interval;
        let mut current_temp = self.initial_temp;

        let handle = thread::spawn(move || {
            // Создаем UDP сокет для отправки
            let socket = match UdpSocket::bind("0.0.0.0:0") {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("[ThermEmulator] UDP socket error: {}", e);
                    return;
                }
            };

            while running.load(Ordering::Relaxed) {
                // Обновляем температуру согласно сценарию
                current_temp = Self::update_temperature(current_temp, scenario);

                // Отправляем данные по UDP
                if let Some(ref addr) = target_addr {
                    let _ =
                        Self::send_temperature_data(&socket, addr, current_temp, device_id.clone());
                }

                thread::sleep(interval);
            }

            println!("[ThermEmulator] Server stopped");
        });

        self.thread_handle = Some(handle);
    }

    /// Останавливает поток эмуляции
    pub fn stop(&mut self) {
        println!("[ThermEmulator] Stopping...");
        self.running.store(false, Ordering::Relaxed);

        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }

        println!("[ThermEmulator] Stopped");
    }

    /// Обновляет температуру согласно сценарию
    fn update_temperature(current_temp: f64, scenario: EmulationScenario) -> f64 {
        let mut rng = rand::rng();

        match scenario {
            EmulationScenario::Normal => {
                // Небольшие случайные колебания ±0.5°C
                current_temp + rng.random_range(-0.5..=0.5)
            }
            EmulationScenario::Fire => {
                // Быстрый рост температуры +1-3°C за итерацию
                current_temp + rng.random_range(1.0..=3.0)
            }
            EmulationScenario::Freeze => {
                // Быстрое падение температуры -1-3°C за итерацию
                current_temp - rng.random_range(1.0..=3.0)
            }
            EmulationScenario::Fluctuate => {
                // Большие колебания ±2°C
                current_temp + rng.random_range(-2.0..=2.0)
            }
        }
    }

    /// Отправляет данные о температуре по UDP
    fn send_temperature_data(
        socket: &UdpSocket,
        addr: &str,
        temperature: f64,
        device_id: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = ThermData {
            temperature,
            device_id,
        };

        let json_data = serde_json::to_string(&data)?;
        socket.send_to(json_data.as_bytes(), addr)?;

        println!("[ThermEmulator] Send: {:.1}°C to {}", temperature, addr);
        Ok(())
    }
}

impl Drop for ThermEmulator {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn emulator_creation() {
        let emulator = ThermEmulator::new(22.5);
        assert_eq!(emulator.initial_temp, 22.5);
        assert_eq!(emulator.device_id, None);
        assert!(matches!(emulator.scenario, EmulationScenario::Normal));
        assert_eq!(emulator.interval, Duration::from_secs(1));
        assert_eq!(emulator.target_addr, None);
        assert!(!emulator.running.load(Ordering::Relaxed));
    }

    #[test]
    fn builder_pattern_device_id() {
        let emulator = ThermEmulator::new(20.0).with_device_id("kitchen_001");
        assert_eq!(emulator.device_id, Some("kitchen_001".to_string()));
        assert_eq!(emulator.initial_temp, 20.0);
    }

    #[test]
    fn builder_pattern_scenario() {
        let emulator = ThermEmulator::new(18.0).with_scenario(EmulationScenario::Fire);
        assert!(matches!(emulator.scenario, EmulationScenario::Fire));
    }

    #[test]
    fn builder_pattern_interval() {
        let custom_interval = Duration::from_millis(500);
        let emulator = ThermEmulator::new(25.0).with_update_interval(custom_interval);
        assert_eq!(emulator.interval, custom_interval);
    }

    #[test]
    fn builder_pattern_chaining() {
        let emulator = ThermEmulator::new(19.5)
            .with_device_id("living_room_002")
            .with_scenario(EmulationScenario::Freeze)
            .with_update_interval(Duration::from_millis(200));

        assert_eq!(emulator.initial_temp, 19.5);
        assert_eq!(emulator.device_id, Some("living_room_002".to_string()));
        assert!(matches!(emulator.scenario, EmulationScenario::Freeze));
        assert_eq!(emulator.interval, Duration::from_millis(200));
    }

    #[test]
    fn connect_to_sets_target_address() {
        let mut emulator = ThermEmulator::new(22.0);
        let result = emulator.connect_to("127.0.0.1:8080");
        assert!(result.is_ok());
        assert_eq!(emulator.target_addr, Some("127.0.0.1:8080".to_string()));
    }

    #[test]
    fn update_temperature_normal_scenario() {
        let initial_temp = 20.0;
        // Тестируем несколько итераций
        for _ in 0..10 {
            let new_temp =
                ThermEmulator::update_temperature(initial_temp, EmulationScenario::Normal);
            assert!(new_temp >= initial_temp - 0.5);
            assert!(new_temp <= initial_temp + 0.5);
        }
    }

    #[test]
    fn update_temperature_fire_scenario() {
        let initial_temp = 20.0;
        for _ in 0..10 {
            let new_temp = ThermEmulator::update_temperature(initial_temp, EmulationScenario::Fire);
            assert!(new_temp >= initial_temp + 1.0);
            assert!(new_temp <= initial_temp + 3.0);
        }
    }

    #[test]
    fn update_temperature_freeze_scenario() {
        let initial_temp = 20.0;
        for _ in 0..10 {
            let new_temp =
                ThermEmulator::update_temperature(initial_temp, EmulationScenario::Freeze);
            assert!(new_temp >= initial_temp - 3.0);
            assert!(new_temp <= initial_temp - 1.0);
        }
    }

    #[test]
    fn update_temperature_fluctuate_scenario() {
        let initial_temp = 20.0;
        for _ in 0..10 {
            let new_temp =
                ThermEmulator::update_temperature(initial_temp, EmulationScenario::Fluctuate);
            assert!(new_temp >= initial_temp - 2.0);
            assert!(new_temp <= initial_temp + 2.0);
        }
    }

    #[test]
    fn json_serialization() {
        // Тестируем только сериализацию, без сетевых операций
        let data = ThermData {
            temperature: 23.5,
            device_id: Some("test_device".to_string()),
        };

        let json = serde_json::to_string(&data).expect("Failed to serialize");
        assert!(json.contains("23.5"));
        assert!(json.contains("test_device"));

        let parsed: ThermData = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(parsed.temperature, 23.5);
        assert_eq!(parsed.device_id, Some("test_device".to_string()));
    }

    #[test]
    fn json_serialization_no_device_id() {
        let data = ThermData {
            temperature: -5.5,
            device_id: None,
        };

        let json = serde_json::to_string(&data).expect("Failed to serialize");
        let parsed: ThermData = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(parsed.temperature, -5.5);
        assert_eq!(parsed.device_id, None);
    }

    // Быстрый тест жизненного цикла без реальной сети
    #[test]
    fn emulator_state_management() {
        let mut emulator = ThermEmulator::new(20.0);

        // Изначально не запущен
        assert!(!emulator.running.load(Ordering::Relaxed));

        // Устанавливаем target (без реальной отправки)
        emulator
            .connect_to("127.0.0.1:65432")
            .expect("Failed to set target");
        assert_eq!(emulator.target_addr, Some("127.0.0.1:65432".to_string()));

        // Проверяем что running флаг можно изменить
        let running = Arc::clone(&emulator.running);
        running.store(true, Ordering::Relaxed);
        assert!(emulator.running.load(Ordering::Relaxed));

        running.store(false, Ordering::Relaxed);
        assert!(!emulator.running.load(Ordering::Relaxed));
    }

    // Интеграционный тест - помечаем как ignored для обычных запусков
    #[test]
    #[ignore = "integration test with networking"]
    fn integration_send_temperature_data() {
        use std::net::UdpSocket;

        let socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind test socket");
        let receiver = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind receiver");
        let receiver_addr = receiver
            .local_addr()
            .expect("Failed to get receiver address");

        let test_temp = 23.5;
        let test_device_id = Some("test_device".to_string());

        let result = ThermEmulator::send_temperature_data(
            &socket,
            &receiver_addr.to_string(),
            test_temp,
            test_device_id.clone(),
        );
        assert!(result.is_ok());

        receiver
            .set_read_timeout(Some(Duration::from_millis(100)))
            .ok();
        let mut buf = [0; 1024];

        if let Ok((size, _)) = receiver.recv_from(&mut buf) {
            let received_json = std::str::from_utf8(&buf[..size]).expect("Invalid UTF-8");
            let parsed: ThermData =
                serde_json::from_str(received_json).expect("Failed to parse JSON");

            assert_eq!(parsed.temperature, test_temp);
            assert_eq!(parsed.device_id, test_device_id);
        }
    }

    // Интеграционный тест жизненного цикла - тоже ignored
    #[test]
    #[ignore = "integration test with threading"]
    fn integration_emulator_lifecycle() {
        let mut emulator = ThermEmulator::new(20.0)
            .with_device_id("test_device")
            .with_update_interval(Duration::from_millis(50)); // Быстрый интервал

        emulator
            .connect_to("127.0.0.1:65432")
            .expect("Failed to set target");

        // Запускаем
        emulator.start();
        assert!(emulator.running.load(Ordering::Relaxed));

        // Ждем немного
        std::thread::sleep(Duration::from_millis(100));

        // Останавливаем
        emulator.stop();
        assert!(!emulator.running.load(Ordering::Relaxed));
    }

    #[test]
    #[ignore = "integration test with threading"]
    #[should_panic(expected = "Emulator already running!")]
    fn integration_emulator_double_start_panics() {
        let mut emulator = ThermEmulator::new(20.0);
        emulator
            .connect_to("127.0.0.1:65433")
            .expect("Failed to set target");

        emulator.start();
        emulator.start(); // Должно вызвать панику
    }
}

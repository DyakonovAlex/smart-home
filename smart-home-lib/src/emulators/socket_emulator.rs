//! Async эмулятор умной розетки для TCP тестирования

use crate::protocol::socket_protocol::{
    SocketCommand, SocketData, SocketResponse, receive_command, send_response,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;

/// Конфигурация эмулятора
#[derive(Debug, Clone)]
pub struct EmulatorConfig {
    /// Адрес для прослушивания TCP соединений
    pub bind_address: String,
    /// Номинальная мощность устройства (в ваттах)
    pub power_rating: f64,
    /// ID устройства для логирования
    pub device_id: String,
}

impl EmulatorConfig {
    /// Создает новую конфигурацию с заданной мощностью
    pub fn new(power_rating: f64) -> Self {
        Self {
            bind_address: "127.0.0.1:0".to_string(),
            power_rating,
            device_id: "socket_emulator".to_string(),
        }
    }

    /// Builder: Устанавливает адрес для прослушивания
    pub fn with_address(mut self, address: &str) -> Self {
        self.bind_address = address.to_string();
        self
    }

    /// Builder: Устанавливает ID устройства
    pub fn with_device_id(mut self, device_id: &str) -> Self {
        self.device_id = device_id.to_string();
        self
    }
}

/// Состояние эмулируемой розетки
#[derive(Debug, Clone)]
struct SocketState {
    active: bool,
    current_power: f64, // В ваттах
    device_id: Option<String>,
}

impl SocketState {
    /// Создает новое состояние розетки без ID
    fn new() -> Self {
        Self {
            active: false,
            current_power: 0.0,
            device_id: None,
        }
    }

    /// Builder: Устанавливает ID устройства
    pub fn with_device_id(mut self, device_id: String) -> Self {
        self.device_id = Some(device_id);
        self
    }

    fn turn_on(&mut self, power_rating: f64) {
        self.active = true;
        self.current_power = power_rating;

        let id = self.device_id.as_deref().unwrap_or("socket");
        println!("[{}] Socket turned ON - {}W", id, power_rating);
    }

    fn turn_off(&mut self) {
        self.active = false;
        self.current_power = 0.0;

        let id = self.device_id.as_deref().unwrap_or("socket");
        println!("[{}] Socket turned OFF", id);
    }

    fn to_data(&self) -> SocketData {
        SocketData {
            active: self.active,
            power: self.current_power,
            device_id: self.device_id.clone(),
        }
    }
}

/// Async эмулятор умной розетки
pub struct SocketEmulator {
    /// Общее состояние розетки для всех клиентов
    state: Arc<Mutex<SocketState>>,
    /// Конфигурация эмулятора
    config: EmulatorConfig,
    /// Адрес на котором запущен сервер (после start)
    bound_addr: Option<std::net::SocketAddr>,
    /// Флаг работы сервера
    running: Arc<AtomicBool>,
    /// Handle главной задачи сервера
    server_handle: Option<JoinHandle<()>>,
    /// Канал для graceful shutdown
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl SocketEmulator {
    /// Создает новый эмулятор (синхронный конструктор)
    pub fn new(config: EmulatorConfig) -> Self {
        Self {
            state: Arc::new(Mutex::new(
                SocketState::new().with_device_id(config.device_id.clone()),
            )),
            config,
            bound_addr: None,
            running: Arc::new(AtomicBool::new(false)),
            server_handle: None,
            shutdown_tx: None,
        }
    }

    /// Возвращает локальный адрес TCP сервера (только после start)
    pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.bound_addr.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Server not started yet - call start() first",
            )
        })
    }

    /// Запускает async TCP сервер (делает bind и старт)
    pub async fn start(&mut self) -> std::io::Result<()> {
        if self.is_running() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Emulator already started",
            ));
        }

        // Bind TCP listener при старте
        let listener = TcpListener::bind(&self.config.bind_address).await?;
        let bound_addr = listener.local_addr()?;
        println!("[SocketEmulator] Bound to {}", bound_addr);

        // Сохраняем адрес
        self.bound_addr = Some(bound_addr);

        // Создаем канал для graceful shutdown
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);

        let state = Arc::clone(&self.state);
        let running = Arc::clone(&self.running);
        let config = self.config.clone();

        // Помечаем что запустились
        running.store(true, Ordering::Relaxed);

        // Запускаем главную async задачу TCP сервера
        let handle = tokio::spawn(async move {
            println!("[SocketEmulator] Started accepting connections");

            loop {
                tokio::select! {
                    // Принимаем TCP соединения
                    result = listener.accept() => {
                        match result {
                            Ok((stream, addr)) => {
                                println!("[SocketEmulator] New client: {}", addr);

                                let client_state = Arc::clone(&state);
                                let client_config = config.clone();

                                // Каждый клиент в отдельной async задаче
                                tokio::spawn(async move {
                                    if let Err(e) = Self::handle_client(stream, client_state, client_config).await {
                                        println!("[SocketEmulator] Client {} error: {}", addr, e);
                                    } else {
                                        println!("[SocketEmulator] Client {} disconnected", addr);
                                    }
                                });
                            }
                            Err(e) => {
                                eprintln!("[SocketEmulator] Accept error: {}", e);
                                break;
                            }
                        }
                    }
                    // Ждем сигнал graceful shutdown
                    _ = &mut shutdown_rx => {
                        println!("[SocketEmulator] Shutdown signal received");
                        break;
                    }
                }
            }

            println!("[SocketEmulator] Server stopped");
        });

        // Сохраняем handle
        self.server_handle = Some(handle);

        Ok(())
    }

    /// Останавливает async сервер (graceful shutdown)
    pub async fn stop(&mut self) {
        println!("[SocketEmulator] Stopping...");
        self.running.store(false, Ordering::Relaxed);

        // Отправляем сигнал остановки через канал
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        // Graceful shutdown - ждем завершения задачи
        if let Some(handle) = self.server_handle.take() {
            let _ = handle.await;
        }

        // Очищаем адрес
        self.bound_addr = None;

        println!("[SocketEmulator] Stopped");
    }

    /// Проверяет, запущен ли эмулятор
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    /// Async обработка одного TCP клиента
    async fn handle_client(
        mut stream: TcpStream,
        state: Arc<Mutex<SocketState>>,
        config: EmulatorConfig,
    ) -> std::io::Result<()> {
        loop {
            let command = match receive_command(&mut stream).await {
                Ok(cmd) => cmd,
                Err(e) => {
                    // Ошибка чтения команды (клиент отключился или невалидная команда)
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        // Клиент закрыл соединение
                        break;
                    }

                    // Невалидная команда - отправляем ошибку
                    let error_response = SocketResponse::Error {
                        message: format!("Invalid command: {}", e),
                    };

                    // Пытаемся отправить ошибку (если stream еще жив)
                    let _ = send_response(&mut stream, &error_response).await;
                    continue;
                }
            };

            let response = Self::process_command(command, &state, &config);

            if let Err(e) = send_response(&mut stream, &response).await {
                // Ошибка отправки - клиент отключился
                println!("[SocketEmulator] Send error: {}", e);
                break;
            }
        }

        Ok(())
    }

    /// Обрабатывает команду и возвращает ответ
    fn process_command(
        command: SocketCommand,
        state: &Arc<Mutex<SocketState>>,
        config: &EmulatorConfig,
    ) -> SocketResponse {
        let mut state_guard = match state.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return SocketResponse::Error {
                    message: "Internal state lock error".to_string(),
                };
            }
        };

        match command {
            SocketCommand::TurnOn => {
                state_guard.turn_on(config.power_rating);
                SocketResponse::Ok(state_guard.to_data())
            }
            SocketCommand::TurnOff => {
                state_guard.turn_off();
                SocketResponse::Ok(state_guard.to_data())
            }
            SocketCommand::Power => SocketResponse::Ok(state_guard.to_data()),
        }
    }
}

// Автоматическая остановка при Drop
impl Drop for SocketEmulator {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);

        // Пытаемся отправить сигнал остановки
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        println!("[SocketEmulator] Drop - sending shutdown signal");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[test]
    fn emulator_creation() {
        let config = EmulatorConfig::new(1500.0);
        let emulator = SocketEmulator::new(config.clone());

        assert!(!emulator.is_running());
        assert_eq!(emulator.config.power_rating, 1500.0);
        assert_eq!(emulator.config.device_id, "socket_emulator");
        assert!(emulator.local_addr().is_err()); // Не запущен
    }

    #[test]
    fn config_builder_pattern() {
        let config = EmulatorConfig::new(2000.0)
            .with_address("127.0.0.1:9999")
            .with_device_id("test_socket");

        assert_eq!(config.power_rating, 2000.0);
        assert_eq!(config.bind_address, "127.0.0.1:9999");
        assert_eq!(config.device_id, "test_socket");
    }

    #[test]
    fn socket_state_management() {
        let state = SocketState::new().with_device_id("test".to_string());
        assert!(!state.active);
        assert_eq!(state.current_power, 0.0);
        assert_eq!(state.device_id, Some("test".to_string()));

        let mut state = SocketState::new();
        state.turn_on(1000.0);
        assert!(state.active);
        assert_eq!(state.current_power, 1000.0);

        state.turn_off();
        assert!(!state.active);
        assert_eq!(state.current_power, 0.0);
    }

    #[test]
    fn socket_data_conversion() {
        let state = SocketState::new().with_device_id("kitchen_socket".to_string());

        let data = state.to_data();
        assert!(!data.active);
        assert_eq!(data.power, 0.0);
        assert_eq!(data.device_id, Some("kitchen_socket".to_string()));
    }

    #[test]
    fn command_processing() {
        let state = Arc::new(Mutex::new(SocketState::new()));
        let config = EmulatorConfig::new(1500.0);

        // Test TurnOn command
        let response = SocketEmulator::process_command(SocketCommand::TurnOn, &state, &config);

        if let SocketResponse::Ok(data) = response {
            assert!(data.active);
            assert_eq!(data.power, 1500.0);
        } else {
            panic!("Expected Ok response");
        }

        // Test Power command
        let response = SocketEmulator::process_command(SocketCommand::Power, &state, &config);

        if let SocketResponse::Ok(data) = response {
            assert!(data.active);
            assert_eq!(data.power, 1500.0);
        } else {
            panic!("Expected Ok response");
        }

        // Test TurnOff command
        let response = SocketEmulator::process_command(SocketCommand::TurnOff, &state, &config);

        if let SocketResponse::Ok(data) = response {
            assert!(!data.active);
            assert_eq!(data.power, 0.0);
        } else {
            panic!("Expected Ok response");
        }
    }

    #[tokio::test]
    #[ignore = "integration test with async TCP server"]
    async fn emulator_lifecycle() {
        let config = EmulatorConfig::new(1000.0).with_address("127.0.0.1:0"); // Let OS choose port

        let mut emulator = SocketEmulator::new(config);

        // Start emulator
        emulator.start().await.expect("Failed to start emulator");
        assert!(emulator.is_running());
        assert!(emulator.local_addr().is_ok());

        let addr = emulator.local_addr().unwrap();
        println!("Emulator started on: {}", addr);

        // Stop emulator
        emulator.stop().await;
        assert!(!emulator.is_running());
        assert!(emulator.local_addr().is_err());
    }

    #[tokio::test]
    #[ignore = "integration test with async TCP server"]
    async fn emulator_double_start_fails() {
        let config = EmulatorConfig::new(1000.0).with_address("127.0.0.1:0");

        let mut emulator = SocketEmulator::new(config);

        // should succeed
        emulator.start().await.expect("First start failed");

        // should fail
        let result = emulator.start().await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            std::io::ErrorKind::AlreadyExists
        );

        emulator.stop().await;
    }

    #[tokio::test]
    #[ignore = "integration test with async TCP networking"]
    async fn client_server_communication() {
        use crate::protocol::socket_protocol::send_command_and_receive;
        use tokio::net::TcpStream;

        let config = EmulatorConfig::new(2000.0)
            .with_address("127.0.0.1:0")
            .with_device_id("test_socket");

        let mut emulator = SocketEmulator::new(config);
        emulator.start().await.expect("Failed to start emulator");

        let addr = emulator.local_addr().expect("No local address");

        let mut client = timeout(Duration::from_secs(5), TcpStream::connect(addr))
            .await
            .expect("Connection timeout")
            .expect("Failed to connect");

        // Test TurnOn command
        let response = timeout(
            Duration::from_secs(2),
            send_command_and_receive(&mut client, &SocketCommand::TurnOn),
        )
        .await
        .expect("Command timeout")
        .expect("Failed to send command");

        if let SocketResponse::Ok(data) = response {
            assert!(data.active);
            assert_eq!(data.power, 2000.0);
            assert_eq!(data.device_id, Some("test_socket".to_string()));
        } else {
            panic!("Expected Ok response, got: {:?}", response);
        }

        // Test Power query
        let response = timeout(
            Duration::from_secs(2),
            send_command_and_receive(&mut client, &SocketCommand::Power),
        )
        .await
        .expect("Command timeout")
        .expect("Failed to send command");

        if let SocketResponse::Ok(data) = response {
            assert!(data.active);
            assert_eq!(data.power, 2000.0);
        } else {
            panic!("Expected Ok response, got: {:?}", response);
        }

        // Test TurnOff command
        let response = timeout(
            Duration::from_secs(2),
            send_command_and_receive(&mut client, &SocketCommand::TurnOff),
        )
        .await
        .expect("Command timeout")
        .expect("Failed to send command");

        if let SocketResponse::Ok(data) = response {
            assert!(!data.active);
            assert_eq!(data.power, 0.0);
        } else {
            panic!("Expected Ok response, got: {:?}", response);
        }

        emulator.stop().await;
    }

    #[tokio::test]
    #[ignore = "integration test with async TCP networking"]
    async fn multiple_clients() {
        use crate::protocol::socket_protocol::send_command_and_receive;
        use tokio::net::TcpStream;

        let config = EmulatorConfig::new(1500.0).with_address("127.0.0.1:0");
        let mut emulator = SocketEmulator::new(config);
        emulator.start().await.expect("Failed to start emulator");

        let addr = emulator.local_addr().expect("No local address");

        // Connect multiple clients
        let mut client1 = TcpStream::connect(addr)
            .await
            .expect("Client1 connection failed");
        let mut client2 = TcpStream::connect(addr)
            .await
            .expect("Client2 connection failed");

        // Client1
        let response1 = send_command_and_receive(&mut client1, &SocketCommand::TurnOn)
            .await
            .expect("Client1 command failed");

        if let SocketResponse::Ok(data) = response1 {
            assert!(data.active);
            assert_eq!(data.power, 1500.0);
        } else {
            panic!("Expected Ok response from client1");
        }

        // Client2
        let response2 = send_command_and_receive(&mut client2, &SocketCommand::Power)
            .await
            .expect("Client2 command failed");

        if let SocketResponse::Ok(data) = response2 {
            assert!(data.active);
            assert_eq!(data.power, 1500.0);
        } else {
            panic!("Expected Ok response from client2");
        }

        emulator.stop().await;
    }

    #[test]
    fn drop_behavior() {
        let config = EmulatorConfig::new(1000.0);
        let emulator = SocketEmulator::new(config);

        assert!(!emulator.is_running());

        drop(emulator);
    }
}

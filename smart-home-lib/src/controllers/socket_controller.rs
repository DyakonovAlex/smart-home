//! Async TCP контроллер для умной розетки

use crate::devices::SmartSocket;
use crate::protocol::socket_protocol::{
    SocketCommand, SocketData, SocketResponse, send_command_and_receive,
};
use crate::traits::Reporter;
use crate::units::Watts;
use std::fmt;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Ошибки контроллера розетки
#[derive(Debug, Clone)]
pub enum SocketError {
    /// Ошибка подключения к розетке
    ConnectionError(String),
    /// Ошибка отправки команды
    CommandError(String),
    /// Розетка вернула ошибку
    DeviceError(String),
    /// Ошибка блокировки
    LockError,
    /// Таймаут операции
    Timeout,
}

impl std::fmt::Display for SocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectionError(msg) => write!(f, "Ошибка подключения: {}", msg),
            Self::CommandError(msg) => write!(f, "Ошибка команды: {}", msg),
            Self::DeviceError(msg) => write!(f, "Ошибка устройства: {}", msg),
            Self::LockError => write!(f, "Ошибка блокировки"),
            Self::Timeout => write!(f, "Таймаут операции"),
        }
    }
}

impl std::error::Error for SocketError {}

/// Async контроллер умной розетки (TCP)
pub struct SocketController {
    /// Внутренняя розетка (модель состояния)
    socket: Arc<RwLock<SmartSocket>>,
    /// Адрес розетки для TCP подключения
    address: SocketAddr,
    /// Таймаут для TCP операций
    timeout: Duration,
    /// Постоянное TCP соединение
    connection: Option<TcpStream>,
}

impl SocketController {
    /// Создает новый контроллер розетки
    pub fn new(address: SocketAddr, power_rating: f64, timeout: Duration) -> Self {
        Self {
            socket: Arc::new(RwLock::new(SmartSocket::new(power_rating))),
            address,
            timeout,
            connection: None,
        }
    }

    /// Обеспечивает наличие соединения (переподключается при необходимости)
    async fn ensure_connected(&mut self) -> Result<&mut TcpStream, SocketError> {
        // Проверяем существующее соединение
        let need_reconnect = match &self.connection {
            Some(stream) => !self.is_connection_alive(stream),
            None => true,
        };

        // Если соединение живое, возвращаем его
        if !need_reconnect {
            return Ok(self.connection.as_mut().unwrap());
        }

        // Переподключаемся
        self.connection = None;

        // Создаем новое соединение с таймаутом
        let stream = timeout(self.timeout, TcpStream::connect(self.address))
            .await
            .map_err(|_| SocketError::Timeout)?
            .map_err(|e| SocketError::ConnectionError(e.to_string()))?;

        self.connection = Some(stream);
        Ok(self.connection.as_mut().unwrap())
    }

    /// Проверяет живость TCP соединения
    fn is_connection_alive(&self, stream: &TcpStream) -> bool {
        stream.peer_addr().is_ok()
    }

    /// Отправляет команду, получает ответ и синхронизирует состояние
    async fn send_command_and_sync(
        &mut self,
        command: SocketCommand,
    ) -> Result<SocketData, SocketError> {
        let cmd_timeout = self.timeout;
        let stream = self.ensure_connected().await?;

        let response = timeout(cmd_timeout, send_command_and_receive(stream, &command))
            .await
            .map_err(|_| SocketError::Timeout)?
            .map_err(|e| SocketError::CommandError(e.to_string()))?;

        match response {
            SocketResponse::Ok(data) => {
                // Синхронизируем локальное состояние с данными от железки
                let mut socket = self.socket.write().map_err(|_| SocketError::LockError)?;

                if data.active {
                    socket.turn_on();
                } else {
                    socket.turn_off();
                }

                Ok(data)
            }
            SocketResponse::Error { message } => Err(SocketError::DeviceError(message)),
        }
    }

    /// Включает розетку
    pub async fn turn_on(&mut self) -> Result<(), SocketError> {
        self.send_command_and_sync(SocketCommand::TurnOn).await?;
        Ok(())
    }

    /// Выключает розетку
    pub async fn turn_off(&mut self) -> Result<(), SocketError> {
        self.send_command_and_sync(SocketCommand::TurnOff).await?;
        Ok(())
    }

    /// Получает актуальную мощность с железки
    pub async fn power(&mut self) -> Result<Watts, SocketError> {
        let _data = self.send_command_and_sync(SocketCommand::Power).await?;

        let socket = self.socket.read().map_err(|_| SocketError::LockError)?;
        Ok(socket.current_power())
    }

    /// Получает копию внутренней розетки
    pub fn device(&self) -> Result<SmartSocket, SocketError> {
        self.socket
            .read()
            .map(|socket| socket.clone())
            .map_err(|_| SocketError::LockError)
    }

    /// Разрывает соединение
    pub fn disconnect(&mut self) {
        self.connection = None;
    }

    /// Возвращает адрес розетки
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// Возвращает таймаут
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

impl Drop for SocketController {
    fn drop(&mut self) {
        self.connection = None;
    }
}

impl Reporter for SocketController {
    fn report(&self) -> String {
        match self.device() {
            Ok(device) => device.report(),
            Err(_) => format!("SocketController({}) - Error reading state", self.address),
        }
    }
}

impl fmt::Display for SocketController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_controller_creation() {
        let addr = "127.0.0.1:8080".parse().unwrap();
        let controller = SocketController::new(addr, 1500.0, Duration::from_secs(5));

        assert_eq!(controller.address(), addr);
        assert_eq!(controller.timeout(), Duration::from_secs(5));

        let device = controller.device().unwrap();
        assert_eq!(device.power_rating(), Watts::new(1500.0));
        assert!(!device.is_active());
    }

    #[tokio::test]
    async fn test_connection_error() {
        let addr = "127.0.0.1:9999".parse().unwrap();
        let mut controller = SocketController::new(addr, 1500.0, Duration::from_millis(100));

        let result = controller.turn_on().await;
        assert!(result.is_err());

        if let Err(SocketError::Timeout) | Err(SocketError::ConnectionError(_)) = result {
            // Ожидаемые варианты ошибок
        } else {
            panic!("Expected Timeout or ConnectionError, got: {:?}", result);
        }
    }

    #[test]
    fn test_sync_device_access() {
        let addr = "127.0.0.1:8080".parse().unwrap();
        let controller = SocketController::new(addr, 1500.0, Duration::from_secs(5));

        let device = controller.device().unwrap();
        assert!(!device.is_active());
        assert_eq!(device.power_rating(), Watts::new(1500.0));
    }

    #[test]
    fn test_report() {
        let addr = "127.0.0.1:8080".parse().unwrap();
        let controller = SocketController::new(addr, 1500.0, Duration::from_secs(5));

        let report = controller.report();
        assert!(report.contains("Smart Socket") || report.contains("SocketController"));
    }
}

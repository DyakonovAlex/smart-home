//! Контроллеры для взаимодействия с внешними устройствами

// Экспортируем модули
pub mod socket_controller;
pub mod therm_controller;

// Реэкспортируем основные типы и функции для удобства
pub use socket_controller::{SocketController, SocketError};
pub use therm_controller::{SubscriptionHandle, ThermController, ThermError};

// ---

use crate::traits::Reporter;
use std::fmt;

/// Универсальный тип для контроллеров
pub enum DeviceController {
    Socket(SocketController),
    Therm(ThermController),
}

impl Reporter for DeviceController {
    fn report(&self) -> String {
        match self {
            Self::Socket(s) => s.report(),
            Self::Therm(t) => t.report(),
        }
    }
}

impl fmt::Display for DeviceController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
    }
}

impl From<SocketController> for DeviceController {
    fn from(socket: SocketController) -> Self {
        Self::Socket(socket)
    }
}

impl From<ThermController> for DeviceController {
    fn from(therm: ThermController) -> Self {
        Self::Therm(therm)
    }
}

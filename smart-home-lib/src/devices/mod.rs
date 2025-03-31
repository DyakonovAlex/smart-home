//! Модуль устройств умного дома

use std::fmt;

mod smart_socket;
mod smart_therm;

pub use smart_socket::SmartSocket;
pub use smart_therm::SmartTherm;

/// Универсальный тип для устройств умного дома
#[derive(Debug)]
pub enum SmartDevice {
    Socket(SmartSocket),
    Therm(SmartTherm),
}

impl SmartDevice {
    /// Возвращает текстовый отчет о состоянии устройства
    pub fn status_report(&self) -> String {
        match self {
            Self::Socket(s) => s.status_report(),
            Self::Therm(t) => t.status_report(),
        }
    }
}

impl fmt::Display for SmartDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.status_report())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_enum() {
        let mut socket = SmartDevice::Socket(SmartSocket::new(1500.0));
        let therm = SmartDevice::Therm(SmartTherm::new(22.5));

        if let SmartDevice::Socket(s) = &mut socket {
            s.turn_on();
        }

        assert!(socket.status_report().contains("1500.0W"));
        assert!(therm.status_report().contains("22.5°C"));
    }

    #[test]
    fn test_device_display() {
        let socket = SmartDevice::Socket(SmartSocket::new(1500.0));
        let therm = SmartDevice::Therm(SmartTherm::new(22.5));

        let output = format!("{}", socket);
        assert!(output.starts_with("Smart Socket"));

        let output = format!("{}", therm);
        assert!(output.starts_with("Smart Thermometer"));
    }
}

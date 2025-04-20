//! Модуль устройств умного дома

use crate::traits::Reporter;
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

impl Reporter for SmartDevice {
    fn report(&self) -> String {
        match self {
            Self::Socket(s) => s.report(),
            Self::Therm(t) => t.report(),
        }
    }
}

impl fmt::Display for SmartDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
    }
}

impl From<SmartSocket> for SmartDevice {
    fn from(socket: SmartSocket) -> Self {
        Self::Socket(socket)
    }
}

impl From<SmartTherm> for SmartDevice {
    fn from(therm: SmartTherm) -> Self {
        Self::Therm(therm)
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

        assert!(socket.report().contains("1500.0W"));
        assert!(therm.report().contains("22.5°C"));
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

    #[test]
    fn test_device_from() {
        let socket = SmartSocket::new(1500.0);
        let therm = SmartTherm::new(22.5);

        let socket_device: SmartDevice = socket.into();
        let therm_device: SmartDevice = therm.into();

        assert!(matches!(socket_device, SmartDevice::Socket(_)));
        assert!(matches!(therm_device, SmartDevice::Therm(_)));

        assert!(socket_device.report().contains("1500.0W"));
        assert!(therm_device.report().contains("22.5°C"));
    }
}

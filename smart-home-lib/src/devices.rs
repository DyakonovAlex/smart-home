//! Модуль устройств умного дома

use crate::traits::Reporter;
use std::fmt;

mod smart_socket;
mod smart_therm;

pub use smart_socket::SmartSocket;
pub use smart_therm::SmartTherm;

/// Универсальный тип для устройств умного дома
#[derive(Debug)]
pub enum Device {
    Socket(SmartSocket),
    Therm(SmartTherm),
}

impl Reporter for Device {
    fn report(&self) -> String {
        match self {
            Self::Socket(s) => s.report(),
            Self::Therm(t) => t.report(),
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
    }
}

impl From<SmartSocket> for Device {
    fn from(socket: SmartSocket) -> Self {
        Self::Socket(socket)
    }
}

impl From<SmartTherm> for Device {
    fn from(therm: SmartTherm) -> Self {
        Self::Therm(therm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_enum() {
        let mut socket = Device::Socket(SmartSocket::new(1500.0));
        let therm = Device::Therm(SmartTherm::new(22.5));

        if let Device::Socket(s) = &mut socket {
            s.turn_on();
        }

        assert!(socket.report().contains("1500.0W"));
        assert!(therm.report().contains("22.5°C"));
    }

    #[test]
    fn device_display() {
        let socket = Device::Socket(SmartSocket::new(1500.0));
        let therm = Device::Therm(SmartTherm::new(22.5));

        let output = format!("{}", socket);
        assert!(output.starts_with("Smart Socket"));

        let output = format!("{}", therm);
        assert!(output.starts_with("Smart Thermometer"));
    }

    #[test]
    fn device_from() {
        let socket = SmartSocket::new(1500.0);
        let therm = SmartTherm::new(22.5);

        let socket_device: Device = socket.into();
        let therm_device: Device = therm.into();

        assert!(matches!(socket_device, Device::Socket(_)));
        assert!(matches!(therm_device, Device::Therm(_)));

        assert!(socket_device.report().contains("1500.0W"));
        assert!(therm_device.report().contains("22.5°C"));
    }
}

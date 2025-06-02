//! Умная розетка с возможностью управления и мониторинга

use super::Reporter;
use crate::units::Watts;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct SmartSocket {
    is_active: bool,
    power_rating: Watts,  // Номинальная мощность в ваттах
    current_power: Watts, // Текущая потребляемая мощность в ваттах
}

impl SmartSocket {
    /// Создает новую розетку с указанной номинальной мощностью
    pub fn new(power_rating: f64) -> Self {
        Self {
            is_active: false,
            power_rating: Watts::new(power_rating),
            current_power: Watts::new(0.0),
        }
    }

    /// Включает розетку и начинает потребление энергии
    pub fn turn_on(&mut self) {
        self.is_active = true;
        self.current_power = self.power_rating;
    }

    /// Выключает розетку и останавливает потребление энергии
    pub fn turn_off(&mut self) {
        self.is_active = false;
        self.current_power = Watts::new(0.0);
    }

    /// Возвращает текущее состояние розетки (включена / выключена)
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Возвращает текущую потребляемую мощность в ваттах
    pub fn current_power(&self) -> Watts {
        self.current_power
    }

    /// Возвращает номинальную мощность в ваттах
    pub fn power_rating(&self) -> Watts {
        self.power_rating
    }

    /// Устанавливает текущую потребляемую мощность в ваттах
    pub fn set_current_power(&mut self, power: Watts) {
        self.current_power = power;
    }
}

impl Reporter for SmartSocket {
    fn report(&self) -> String {
        format!(
            "Smart Socket: {} | Power: {} (Rated: {})",
            if self.is_active { "ACTIVE" } else { "INACTIVE" },
            self.current_power,
            self.power_rating
        )
    }
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socket_creation() {
        let socket = SmartSocket::new(1500.0);
        assert!(!socket.is_active());
        assert_eq!(socket.current_power(), Watts::new(0.0));
    }

    #[test]
    fn power_management() {
        let mut socket = SmartSocket::new(2000.0);

        socket.turn_on();
        assert!(socket.is_active());
        assert_eq!(socket.current_power(), Watts::new(2000.0));

        socket.turn_off();
        assert!(!socket.is_active());
        assert_eq!(socket.current_power(), Watts::new(0.0));
    }

    #[test]
    fn report() {
        let mut socket = SmartSocket::new(1500.0);
        assert!(socket.report().contains("INACTIVE"));

        socket.turn_on();
        assert!(socket.report().contains("ACTIVE"));
        assert!(socket.report().contains("1500.0W"));
    }

    #[test]
    fn set_current_power() {
        let mut socket = SmartSocket::new(1500.0);
        socket.set_current_power(Watts::new(1000.0));
        assert_eq!(socket.current_power(), Watts::new(1000.0));
    }
}

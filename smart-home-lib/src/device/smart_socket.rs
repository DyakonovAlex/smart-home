//! Умная розетка с возможностью управления и мониторинга

use super::Reporter;
use std::fmt;

/// Умная розетка с измерением потребляемой мощности
#[derive(Debug, Clone, PartialEq)]
pub struct SmartSocket {
    is_active: bool,
    power_rating: f64,  // Номинальная мощность в ваттах
    current_power: f64, // Текущая потребляемая мощность в ваттах
}

impl SmartSocket {
    /// Создает новую розетку с указанной номинальной мощностью
    pub fn new(power_rating: f64) -> Self {
        Self {
            is_active: false,
            power_rating,
            current_power: 0.0,
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
        self.current_power = 0.0;
    }

    /// Возвращает текущее состояние розетки (включена / выключена)
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Возвращает текущую потребляемую мощность в ваттах
    pub fn current_power(&self) -> f64 {
        self.current_power
    }

    /// Возвращает номинальную мощность в ваттах
    pub fn power_rating(&self) -> f64 {
        self.power_rating
    }
}

impl Reporter for SmartSocket {
    fn report(&self) -> String {
        format!(
            "Smart Socket: {} | Power: {:.1}W (Rated: {:.1}W)",
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
    use approx::assert_relative_eq;

    #[test]
    fn test_socket_creation() {
        let socket = SmartSocket::new(1500.0);
        assert!(!socket.is_active());
        assert_relative_eq!(socket.current_power(), 0.0);
    }

    #[test]
    fn test_power_management() {
        let mut socket = SmartSocket::new(2000.0);

        socket.turn_on();
        assert!(socket.is_active());
        assert_relative_eq!(socket.current_power(), 2000.0);

        socket.turn_off();
        assert!(!socket.is_active());
        assert_relative_eq!(socket.current_power(), 0.0);
    }

    #[test]
    fn test_report() {
        let mut socket = SmartSocket::new(1500.0);
        assert!(socket.report().contains("INACTIVE"));

        socket.turn_on();
        assert!(socket.report().contains("ACTIVE"));
        assert!(socket.report().contains("1500.0W"));
    }
}

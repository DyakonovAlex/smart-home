//! Умный термометр

use super::Reporter;
use std::fmt;

/// Умный термометр
#[derive(Debug, Clone, PartialEq)]
pub struct SmartTherm {
    temperature: f64, // Текущая температура в градусах Цельсия
}

impl SmartTherm {
    /// Создает новый термометр с указанной начальной температурой
    pub fn new(temperature: f64) -> Self {
        Self { temperature }
    }

    /// Возвращает текущую температуру в градусах Цельсия
    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }

    /// Устанавливает новую температуру
    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
    }
}

impl Reporter for SmartTherm {
    fn report(&self) -> String {
        format!("Smart Thermometer: {:.1}°C", self.temperature)
    }
}

impl fmt::Display for SmartTherm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_smart_therm_creation() {
        let therm = SmartTherm::new(22.5);
        assert_relative_eq!(therm.get_temperature(), 22.5);
    }

    #[test]
    fn test_temperature_update() {
        let mut therm = SmartTherm::new(22.5);
        assert_relative_eq!(therm.get_temperature(), 22.5);

        therm.set_temperature(25.0);
        assert_relative_eq!(therm.get_temperature(), 25.0);
    }

    #[test]
    fn test_report() {
        let mut therm = SmartTherm::new(23.7);
        assert!(therm.report().contains("23.7°C"));

        therm.set_temperature(-5.2);
        assert!(therm.report().contains("-5.2°C"));
    }
}

//! Умный термометр

use super::Reporter;
use crate::units::Celsius;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct SmartTherm {
    temperature: Celsius, // Текущая температура в градусах Цельсия
}

impl SmartTherm {
    /// Создает новый термометр с указанной начальной температурой
    pub fn new(temperature: f64) -> Self {
        Self {
            temperature: Celsius::new(temperature),
        }
    }

    /// Возвращает текущую температуру в градусах Цельсия
    pub fn temperature(&self) -> Celsius {
        self.temperature
    }

    /// Устанавливает новую температуру
    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = Celsius::new(temperature);
    }
}

impl Reporter for SmartTherm {
    fn report(&self) -> String {
        format!("Smart Thermometer: {}", self.temperature)
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

    #[test]
    fn smart_therm_creation() {
        let therm = SmartTherm::new(22.5);
        assert_eq!(therm.temperature(), Celsius::new(22.5));
    }

    #[test]
    fn temperature_update() {
        let mut therm = SmartTherm::new(22.5);
        assert_eq!(therm.temperature(), Celsius::new(22.5));

        therm.set_temperature(25.0);
        assert_eq!(therm.temperature(), Celsius::new(25.0));
    }

    #[test]
    fn report() {
        let mut therm = SmartTherm::new(23.7);
        assert!(therm.report().contains("23.7°C"));

        therm.set_temperature(-5.2);
        assert!(therm.report().contains("-5.2°C"));
    }
}

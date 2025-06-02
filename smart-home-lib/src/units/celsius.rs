//! Градусы Цельсия

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Sub};

///  Абсолютный ноль по Цельсию
const ABSOLUTE_ZERO_C: f64 = -273.15;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Celsius(f64);

impl Celsius {
    pub fn new(value: f64) -> Self {
        if value < ABSOLUTE_ZERO_C {
            panic!("Temperature below absolute zero");
        }

        Celsius(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

impl Add for Celsius {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Celsius(self.0 + rhs.0)
    }
}

impl Sub for Celsius {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Celsius(self.0 - rhs.0)
    }
}

impl Add<f64> for Celsius {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Celsius(self.0 + rhs)
    }
}

impl Sub<f64> for Celsius {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Celsius(self.0 - rhs)
    }
}

#[cfg(test)]
mod celsius_tests {
    use super::*;

    #[test]
    fn celsius_creation() {
        let c = Celsius::new(25.0);
        assert_eq!(c.value(), 25.0);
    }

    #[test]
    fn celsius_display() {
        let c = Celsius::new(21.5);
        assert_eq!(format!("{}", c), "21.5°C");
    }

    #[test]
    fn celsius_operations() {
        let c1 = Celsius::new(20.0);
        let c2 = Celsius::new(5.0);

        assert_eq!(c1 + c2, Celsius::new(25.0));
        assert_eq!(c1 - c2, Celsius::new(15.0));
        assert_eq!(c1 + 10.0, Celsius::new(30.0));
        assert_eq!(c1 - 10.0, Celsius::new(10.0));
    }

    #[test]
    #[should_panic(expected = "Temperature below absolute zero")]
    fn celsius_below_absolute_zero() {
        Celsius::new(-300.0);
    }
}

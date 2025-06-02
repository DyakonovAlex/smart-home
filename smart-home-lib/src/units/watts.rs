//! Мощность в ваттах

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Watts(f64);

impl Watts {
    pub fn new(value: f64) -> Self {
        if value < 0.0 {
            panic!("Power rating must be positive");
        }
        Watts(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Watts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}W", self.0)
    }
}

impl Add for Watts {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Watts(self.0 + rhs.0)
    }
}

impl Sub for Watts {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let result = self.0 - rhs.0;
        Watts(if result < 0.0 { 0.0 } else { result })
    }
}

impl Mul<f64> for Watts {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Watts(self.0 * rhs)
    }
}

impl Div<f64> for Watts {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("Division by zero");
        }

        Watts(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn watts_creation() {
        let w = Watts::new(100.0);
        assert_eq!(w.value(), 100.0);
    }

    #[test]
    fn watts_operations() {
        let w1 = Watts::new(100.0);
        let w2 = Watts::new(50.0);

        assert_eq!(w1 + w2, Watts::new(150.0));
        assert_eq!(w1 - w2, Watts::new(50.0));
        assert_eq!(w2 - w1, Watts::new(0.0)); // Убедимся, что не становится отрицательным
        assert_eq!(w1 * 2.0, Watts::new(200.0));
        assert_eq!(w1 / 2.0, Watts::new(50.0));
    }

    #[test]
    fn watts_display() {
        let w = Watts::new(1234.56);
        assert_eq!(format!("{}", w), "1234.6W");
    }

    #[test]
    #[should_panic(expected = "Power rating must be positive")]
    fn watts_negative_value() {
        Watts::new(-10.0);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn watts_division_by_zero() {
        let w = Watts::new(100.0);
        let _ = w / 0.0;
    }
}

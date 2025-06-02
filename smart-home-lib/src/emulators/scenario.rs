//! Сценарии эмуляции термометра

use std::fmt;

/// Сценарии эмуляции термометра
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmulationScenario {
    /// Нормальная работа - стабильная температура
    Normal,
    /// Пожар - быстрый рост температуры  
    Fire,
    /// Заморозка - быстрое падение температуры
    Freeze,
    /// Колебания - циклические изменения температуры
    Fluctuate,
}

impl fmt::Display for EmulationScenario {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            Self::Normal => "🌡️ Нормальная работа",
            Self::Fire => "🔥 Пожар",
            Self::Freeze => "🧊 Заморозка",
            Self::Fluctuate => "📈 Колебания",
        };
        write!(f, "{}", description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_display() {
        assert_eq!(
            format!("{}", EmulationScenario::Normal),
            "🌡️ Нормальная работа"
        );
        assert_eq!(format!("{}", EmulationScenario::Fire), "🔥 Пожар");
        assert_eq!(format!("{}", EmulationScenario::Freeze), "🧊 Заморозка");
        assert_eq!(format!("{}", EmulationScenario::Fluctuate), "📈 Колебания");
    }

    #[test]
    fn scenario_debug() {
        assert_eq!(format!("{:?}", EmulationScenario::Normal), "Normal");
    }
}

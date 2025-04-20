//! Модуль для работы с комнатами умного дома

use crate::device::SmartDevice;
use crate::traits::Reporter;
use std::collections::HashMap;
use std::fmt;

/// Макрос для упрощения создания комнаты с устройствами
#[macro_export]
macro_rules! room {
    ($(($key:expr, $device:expr)),* $(,)?) => {{
        let mut room = Room::default();
        $(
            room.add_device($key, $device);
        )*
        room
    }};
}

/// Комната умного дома, содержащая список устройств
#[derive(Debug, Default)]
pub struct Room {
    devices: HashMap<String, SmartDevice>,
}

impl Room {
    /// Создает новую комнату с заданными устройствами
    pub fn new(devices: HashMap<String, SmartDevice>) -> Self {
        Self { devices }
    }

    /// Возвращает неизменяемую ссылку на устройство по ключу
    pub fn get_device(&self, key: &str) -> Option<&SmartDevice> {
        self.devices.get(key)
    }

    /// Возвращает изменяемую ссылку на устройство по ключу
    pub fn get_device_mut(&mut self, key: &str) -> Option<&mut SmartDevice> {
        self.devices.get_mut(key)
    }

    /// Добавляет устройство в комнату
    pub fn add_device(&mut self, key: &str, device: SmartDevice) {
        self.devices.insert(key.to_string(), device);
    }

    pub fn remove_device(&mut self, key: &str) -> Option<SmartDevice> {
        self.devices.remove(key)
    }

    /// Формирует текстовый отчет о состоянии всех устройств в комнате
    pub fn report_lines(&self) -> Vec<String> {
        self.devices
            .iter()
            .map(|(key, device)| format!("[Device:{}] {}", key, device))
            .collect()
    }

    /// Возвращает количество устройств в комнате
    pub fn devices_count(&self) -> usize {
        self.devices.len()
    }

    /// Возвращает список ключей всех устройств в комнате
    pub fn devices_keys(&self) -> Vec<String> {
        self.devices.keys().cloned().collect()
    }
}

impl Reporter for Room {
    fn report(&self) -> String {
        self.report_lines().join("\n")
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
    }
}

#[cfg(test)]
mod tests {
    use crate::device;

    use super::*;

    fn test_room() -> Room {
        crate::room![
            (
                "kitchen_therm",
                SmartDevice::Therm(device::SmartTherm::new(22.5))
            ),
            (
                "living_socket",
                SmartDevice::Socket(device::SmartSocket::new(1500.0))
            ),
        ]
    }

    #[test]
    fn test_device_access() {
        let mut room = test_room();

        let therm = room.get_device("kitchen_therm");
        assert!(matches!(therm, Some(SmartDevice::Therm(_))));

        if let Some(SmartDevice::Socket(s)) = room.get_device_mut("living_socket") {
            s.turn_on();
            assert!(s.is_active());
        }

        // Проверяем доступ к несуществующему устройству
        assert!(room.get_device("not_exists").is_none());
    }

    #[test]
    fn test_add_remove_device() {
        let mut room = Room::default();

        // Добавляем устройство
        room.add_device(
            "bedroom_therm",
            SmartDevice::Therm(device::SmartTherm::new(20.0)),
        );
        assert_eq!(room.devices_count(), 1);

        // Проверяем что устройство добавлено
        assert!(matches!(
            room.get_device("bedroom_therm"),
            Some(SmartDevice::Therm(_))
        ));

        // Удаляем устройство
        let removed = room.remove_device("bedroom_therm");
        assert!(matches!(removed, Some(SmartDevice::Therm(_))));
        assert_eq!(room.devices_count(), 0);

        // Проверяем что устройство удалено
        assert!(room.get_device("bedroom_therm").is_none());
    }

    #[test]
    fn test_report_lines() {
        let mut room = test_room();

        if let Some(SmartDevice::Socket(s)) = room.get_device_mut("living_socket") {
            s.turn_on();
        }

        let report_lines = room.report_lines();
        assert_eq!(report_lines.len(), 2);

        // Проверяем что отчет содержит информацию об устройствах
        let contains_socket = report_lines
            .iter()
            .any(|s| s.contains("living_socket") && s.contains("1500.0W"));
        let contains_therm = report_lines
            .iter()
            .any(|s| s.contains("kitchen_therm") && s.contains("22.5°C"));

        assert!(contains_socket);
        assert!(contains_therm);
    }

    #[test]
    fn test_report() {
        let mut room = test_room();

        if let Some(SmartDevice::Socket(s)) = room.get_device_mut("living_socket") {
            s.turn_on();
        }

        let report = room.report();

        // Проверяем, что отчет содержит информацию об устройствах
        assert!(report.contains("living_socket"));
        assert!(report.contains("1500.0W"));
        assert!(report.contains("kitchen_therm"));
        assert!(report.contains("22.5°C"));

        // Проверяем, что в отчете есть две строки (один перенос строки)
        assert_eq!(report.matches("\n").count(), 1);
    }

    #[test]
    fn test_display() {
        let room = test_room();
        let display_output = format!("{}", room);

        // Проверяем, что вывод содержит информацию об устройствах
        assert!(display_output.contains("kitchen_therm"));
        assert!(display_output.contains("22.5°C"));
        assert!(display_output.contains("living_socket"));
        assert!(display_output.contains("1500.0W"));
    }

    #[test]
    fn test_devices_count() {
        let room = test_room();
        assert_eq!(room.devices_count(), 2);
    }

    #[test]
    fn test_macro() {
        let room = crate::room![
            (
                "socket1",
                SmartDevice::Socket(device::SmartSocket::new(1000.0))
            ),
            ("therm1", SmartDevice::Therm(device::SmartTherm::new(18.0))),
        ];

        assert_eq!(room.devices_count(), 2);
        assert!(room.get_device("socket1").is_some());
        assert!(room.get_device("therm1").is_some());
    }
}

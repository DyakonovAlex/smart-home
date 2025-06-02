//! Модуль для работы с комнатами умного дома

use crate::controllers::DeviceController;
use crate::devices::Device;
use crate::traits::Reporter;
use std::collections::HashMap;
use std::fmt;

/// Макрос для упрощения создания комнаты с устройствами
#[macro_export]
macro_rules! room {
    ($(($key:expr, $device:expr)),* $(,)?) => {{
        let mut room = Room::default();
        $(
            room.add_item($key, $device);
        )*
        room
    }};
}

/// Комната умного дома, содержащая список устройств
#[derive(Default)]
pub struct Room {
    devices: HashMap<String, Device>,
    controllers: HashMap<String, DeviceController>,
}

impl Room {
    /// Создает новую комнату
    pub fn new() -> Self {
        Self::default()
    }

    /// Возвращает неизменяемую ссылку на устройство по ключу
    pub fn device(&self, key: &str) -> Option<&Device> {
        self.devices.get(key)
    }

    /// Возвращает изменяемую ссылку на устройство по ключу
    pub fn device_mut(&mut self, key: &str) -> Option<&mut Device> {
        self.devices.get_mut(key)
    }

    /// Добавляет устройство в комнату
    pub fn add_device(&mut self, key: &str, device: Device) {
        self.devices.insert(key.to_string(), device);
    }

    /// Удаляет устройство из комнаты
    pub fn remove_device(&mut self, key: &str) -> Option<Device> {
        self.devices.remove(key)
    }

    /// Возвращает неизменяемую ссылку на контроллер по ключу
    pub fn controller(&self, key: &str) -> Option<&DeviceController> {
        self.controllers.get(key)
    }

    /// Возвращает изменяемую ссылку на контроллер по ключу
    pub fn controller_mut(&mut self, key: &str) -> Option<&mut DeviceController> {
        self.controllers.get_mut(key)
    }

    /// Добавляет контроллер в комнату
    pub fn add_controller(&mut self, key: &str, controller: DeviceController) {
        self.controllers.insert(key.to_string(), controller);
    }

    /// Удаляет контроллер из комнаты
    pub fn remove_controller(&mut self, key: &str) -> Option<DeviceController> {
        self.controllers.remove(key)
    }

    /// Универсальный метод для добавления любого элемента в комнату
    pub fn add_item<T>(&mut self, key: &str, item: T)
    where
        T: Into<RoomItem>,
    {
        match item.into() {
            RoomItem::Device(device) => self.add_device(key, device),
            RoomItem::Controller(controller) => self.add_controller(key, controller),
        }
    }

    /// Формирует текстовый отчет о состоянии всех устройств и контроллеров в комнате
    pub fn report_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();

        for (key, device) in &self.devices {
            lines.push(format!("[Device:{}] {}", key, device));
        }

        for (key, controller) in &self.controllers {
            lines.push(format!("[Controller:{}] {}", key, controller));
        }

        lines
    }

    /// Возвращает количество устройств в комнате
    pub fn devices_count(&self) -> usize {
        self.devices.len()
    }

    /// Возвращает список ключей всех устройств в комнате
    pub fn devices_keys(&self) -> Vec<String> {
        self.devices.keys().cloned().collect()
    }

    /// Возвращает количество контроллеров в комнате
    pub fn controllers_count(&self) -> usize {
        self.controllers.len()
    }

    /// Возвращает список ключей всех контроллеров в комнате
    pub fn controllers_keys(&self) -> Vec<String> {
        self.controllers.keys().cloned().collect()
    }

    /// Возвращает общее количество устройств и контроллеров в комнате
    pub fn items_count(&self) -> usize {
        self.devices_count() + self.controllers_count()
    }

    /// Возвращает список ключей всех устройств и контроллеров в комнате
    pub fn keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        keys.extend(self.devices.keys().cloned());
        keys.extend(self.controllers.keys().cloned());
        keys
    }
}

/// Универсальный элемент комнаты
pub enum RoomItem {
    Device(Device),
    Controller(DeviceController),
}

impl From<Device> for RoomItem {
    fn from(device: Device) -> Self {
        Self::Device(device)
    }
}

impl From<DeviceController> for RoomItem {
    fn from(controller: DeviceController) -> Self {
        Self::Controller(controller)
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
    use crate::devices::{Device, SmartSocket, SmartTherm};

    use super::*;

    fn test_room() -> Room {
        crate::room![
            ("kitchen_therm", Device::Therm(SmartTherm::new(22.5))),
            ("living_socket", Device::Socket(SmartSocket::new(1500.0))),
        ]
    }

    #[test]
    fn device_access() {
        let mut room = test_room();

        let therm = room.device("kitchen_therm");
        assert!(matches!(therm, Some(Device::Therm(_))));

        if let Some(Device::Socket(s)) = room.device_mut("living_socket") {
            s.turn_on();
            assert!(s.is_active());
        }

        // Проверяем доступ к несуществующему устройству
        assert!(room.device("not_exists").is_none());
    }

    #[test]
    fn add_remove_device() {
        let mut room = Room::default();

        // Добавляем устройство
        room.add_device("bedroom_therm", Device::Therm(SmartTherm::new(20.0)));
        assert_eq!(room.devices_count(), 1);

        // Проверяем что устройство добавлено
        assert!(matches!(
            room.device("bedroom_therm"),
            Some(Device::Therm(_))
        ));

        // Удаляем устройство
        let removed = room.remove_device("bedroom_therm");
        assert!(matches!(removed, Some(Device::Therm(_))));
        assert_eq!(room.devices_count(), 0);

        // Проверяем что устройство удалено
        assert!(room.device("bedroom_therm").is_none());
    }

    #[test]
    fn report_lines() {
        let mut room = test_room();

        if let Some(Device::Socket(s)) = room.device_mut("living_socket") {
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
    fn report() {
        let mut room = test_room();

        if let Some(Device::Socket(s)) = room.device_mut("living_socket") {
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
    fn display() {
        let room = test_room();
        let display_output = format!("{}", room);

        // Проверяем, что вывод содержит информацию об устройствах
        assert!(display_output.contains("kitchen_therm"));
        assert!(display_output.contains("22.5°C"));
        assert!(display_output.contains("living_socket"));
        assert!(display_output.contains("1500.0W"));
    }

    #[test]
    fn devices_count() {
        let room = test_room();
        assert_eq!(room.devices_count(), 2);
    }

    #[test]
    fn macros() {
        let room = crate::room![
            ("socket1", Device::Socket(SmartSocket::new(1000.0))),
            ("therm1", Device::Therm(SmartTherm::new(18.0))),
        ];

        assert_eq!(room.devices_count(), 2);
        assert!(room.device("socket1").is_some());
        assert!(room.device("therm1").is_some());
    }
}

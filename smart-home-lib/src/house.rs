//! Модуль для работы с умным домом

use crate::controllers::DeviceController;
use crate::devices::Device;
use crate::room::Room;
use crate::traits::Reporter;
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Макрос для упрощенного создания умного дома с комнатами
#[macro_export]
macro_rules! house {
    ($(($key:expr, $room:expr)),* $(,)?) => {{
        let mut house = $crate::house::SmartHouse::default();
        $(
            house.add_room($key, $room);
        )*
        house
    }};
}

/// Ошибки, возникающие при работе с умным домом
#[derive(Debug, Error)]
pub enum SmartHouseError {
    #[error("Room not found: '{0}'")]
    RoomNotFound(String),

    #[error("Device '{1}' not found in room '{0}'")]
    DeviceNotFound(String, String),
}

/// Результат выполнения операции
pub type SmartHouseResult<T> = Result<T, SmartHouseError>;

/// Умный дом, содержащий список комнат
#[derive(Default)]
pub struct SmartHouse {
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    /// Создает новый дом с заданными комнатами
    pub fn new(rooms: HashMap<String, Room>) -> Self {
        Self { rooms }
    }

    /// Возвращает неизменяемую ссылку на комнату по индексу
    pub fn room(&self, key: &str) -> Option<&Room> {
        self.rooms.get(key)
    }

    /// Возвращает изменяемую ссылку на комнату по индексу
    pub fn room_mut(&mut self, key: &str) -> Option<&mut Room> {
        self.rooms.get_mut(key)
    }

    /// Добавляет комнату в дом
    pub fn add_room(&mut self, key: &str, room: Room) {
        self.rooms.insert(key.to_string(), room);
    }

    /// Удаляет комнату из дома
    pub fn remove_room(&mut self, key: &str) -> Option<Room> {
        self.rooms.remove(key)
    }

    /// Получает прямую ссылку на устройство по имени комнаты и устройства
    pub fn device(&self, room_key: &str, device_key: &str) -> SmartHouseResult<&Device> {
        self.room(room_key)
            .ok_or(SmartHouseError::RoomNotFound(room_key.to_string()))?
            .device(device_key)
            .ok_or(SmartHouseError::DeviceNotFound(
                room_key.to_string(),
                device_key.to_string(),
            ))
    }

    /// Получает прямую ссылку на контроллер по имени комнаты и контроллера
    pub fn controller(
        &self,
        room_key: &str,
        controller_key: &str,
    ) -> SmartHouseResult<&DeviceController> {
        self.room(room_key)
            .ok_or(SmartHouseError::RoomNotFound(room_key.to_string()))?
            .controller(controller_key)
            .ok_or(SmartHouseError::DeviceNotFound(
                room_key.to_string(),
                controller_key.to_string(),
            ))
    }

    /// Получает прямую изменяяемую ссылку на устройство по имени комнаты и устройства
    pub fn device_mut(
        &mut self,
        room_key: &str,
        device_key: &str,
    ) -> SmartHouseResult<&mut Device> {
        self.room_mut(room_key)
            .ok_or(SmartHouseError::RoomNotFound(room_key.to_string()))?
            .device_mut(device_key)
            .ok_or(SmartHouseError::DeviceNotFound(
                room_key.to_string(),
                device_key.to_string(),
            ))
    }

    /// Получает прямую изменяяемую ссылку на контроллер по имени комнаты и контроллера
    pub fn controller_mut(
        &mut self,
        room_key: &str,
        controller_key: &str,
    ) -> SmartHouseResult<&mut DeviceController> {
        self.room_mut(room_key)
            .ok_or(SmartHouseError::RoomNotFound(room_key.to_string()))?
            .controller_mut(controller_key)
            .ok_or(SmartHouseError::DeviceNotFound(
                room_key.to_string(),
                controller_key.to_string(),
            ))
    }

    /// Формирует текстовый отчет о состоянии всех комнат в доме
    pub fn report_lines(&self) -> Vec<String> {
        self.rooms
            .iter()
            .flat_map(|(key, room)| {
                let mut report = vec![format!("Room: {}", key)];
                report.extend(room.report_lines().iter().map(|s| format!("  {}", s)));
                report
            })
            .collect()
    }

    /// Возвращает количество комнат в доме
    pub fn rooms_count(&self) -> usize {
        self.rooms.len()
    }

    /// Возвращает список ключей всех комнат в доме
    pub fn rooms_keys(&self) -> Vec<String> {
        self.rooms.keys().cloned().collect()
    }
}

impl Reporter for SmartHouse {
    fn report(&self) -> String {
        self.report_lines().join("\n")
    }
}

impl fmt::Display for SmartHouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::{Device, SmartSocket, SmartTherm};
    use crate::room;

    use super::*;

    fn test_house() -> SmartHouse {
        crate::house![
            (
                "kitchen",
                room![("therm", Device::Therm(SmartTherm::new(22.5)))]
            ),
            (
                "living_room",
                room![("socket", Device::Socket(SmartSocket::new(1500.0)))]
            )
        ]
    }

    #[test]
    fn room_access() {
        let mut house = test_house();

        let kitchen = house.room("kitchen").unwrap();
        assert_eq!(kitchen.devices_count(), 1);

        if let Ok(Device::Socket(s)) = house.device_mut("living_room", "socket") {
            s.turn_on();
            assert!(s.is_active());
        }

        // Проверяем доступ к несуществующей комнате
        assert!(house.room("not_exists").is_none());
    }

    #[test]
    fn direct_device_access() {
        let mut house = test_house();

        // Успешный доступ к устройству
        let result = house.device("kitchen", "therm");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Device::Therm(_)));

        // Ошибка при доступе к несуществующей комнате
        let error = house.device("bathroom", "therm").unwrap_err();
        assert!(matches!(error, SmartHouseError::RoomNotFound(_)));

        // Ошибка при доступе к несуществующему устройству
        let error = house.device("kitchen", "socket").unwrap_err();
        assert!(matches!(error, SmartHouseError::DeviceNotFound(_, _)));

        // Изменение устройства через прямой доступ
        if let Ok(Device::Socket(s)) = house.device_mut("living_room", "socket") {
            s.turn_on();
            assert!(s.is_active());
        }
    }

    #[test]
    fn add_remove_room() {
        let mut house = SmartHouse::default();

        // Добавляем комнату
        house.add_room(
            "bedroom",
            room![("therm", Device::Therm(SmartTherm::new(20.0)))],
        );
        assert_eq!(house.rooms_count(), 1);

        // Проверяем что комната добавлена
        assert!(house.room("bedroom").is_some());

        // Удаляем комнату
        let removed = house.remove_room("bedroom");
        assert!(removed.is_some());
        assert_eq!(house.rooms_count(), 0);

        // Проверяем что комната удалена
        assert!(house.room("bedroom").is_none());
    }

    #[test]
    fn report_lines() {
        let mut house = test_house();

        if let Ok(Device::Socket(s)) = house.device_mut("living_room", "socket") {
            s.turn_on();
        }

        let report_lines = house.report_lines();

        // Проверяем количество строк в отчете (2 комнаты x (1 заголовок + 1 устройство))
        assert_eq!(report_lines.len(), 4);

        // Проверяем содержимое отчета
        let contains_kitchen = report_lines.iter().any(|s| s.contains("Room: kitchen"));
        let contains_living = report_lines.iter().any(|s| s.contains("Room: living_room"));
        let contains_therm = report_lines.iter().any(|s| s.contains("22.5°C"));
        let contains_socket = report_lines.iter().any(|s| s.contains("1500.0W"));

        assert!(contains_kitchen);
        assert!(contains_living);
        assert!(contains_therm);
        assert!(contains_socket);
    }

    #[test]
    fn report() {
        let mut house = test_house();

        if let Ok(Device::Socket(s)) = house.device_mut("living_room", "socket") {
            s.turn_on();
        }

        let report = house.report();

        // Проверяем, что отчет содержит информацию о комнатах и устройствах
        assert!(report.contains("Room: kitchen"));
        assert!(report.contains("Room: living_room"));
        assert!(report.contains("22.5°C"));
        assert!(report.contains("1500.0W"));

        // Проверяем, что в отчете правильное количество строк
        assert_eq!(report.matches("\n").count(), 3); // 4 строки = 3 переноса
    }

    #[test]
    fn display() {
        let house = test_house();
        let display_output = format!("{}", house);

        // Проверяем, что вывод содержит информацию о комнатах и устройствах
        assert!(display_output.contains("Room: kitchen"));
        assert!(display_output.contains("22.5°C"));
        assert!(display_output.contains("Room: living_room"));
        assert!(display_output.contains("socket"));
    }

    #[test]
    fn rooms_count() {
        let house = test_house();
        assert_eq!(house.rooms_count(), 2);
    }

    #[test]
    fn macros() {
        let house = crate::house![
            (
                "room1",
                room![("socket1", Device::Socket(SmartSocket::new(1000.0)))]
            ),
            (
                "room2",
                room![("therm1", Device::Therm(SmartTherm::new(18.0)))]
            ),
        ];

        assert_eq!(house.rooms_count(), 2);
        assert!(house.room("room1").is_some());
        assert!(house.room("room2").is_some());
    }
}

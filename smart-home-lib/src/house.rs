//! Модуль для работы с умным домом

use crate::room::Room;

/// Умный дом, содержащий список комнат
#[derive(Debug, Default)]
pub struct SmartHouse {
    rooms: Vec<Room>,
}

impl SmartHouse {
    /// Создает новый дом с заданными комнатами
    pub fn new(rooms: Vec<Room>) -> Self {
        Self { rooms }
    }

    /// Возвращает неизменяемую ссылку на комнату по индексу
    /// # Panics
    /// Паникует, если индекс выходит за пределы списка комнат
    pub fn get_room(&self, index: usize) -> &Room {
        &self.rooms[index]
    }

    /// Возвращает изменяемую ссылку на комнату по индексу
    /// # Panics
    /// Паникует, если индекс выходит за пределы списка комнат
    pub fn get_room_mut(&mut self, index: usize) -> &mut Room {
        &mut self.rooms[index]
    }

    /// Формирует текстовый отчет о состоянии всех комнат в доме
    pub fn report(&self) -> Vec<String> {
        self.rooms
            .iter()
            .enumerate()
            .flat_map(|(i, room)| {
                let mut report = vec![format!("Room {}:", i + 1)];
                report.extend(room.report().iter().map(|s| format!("  {}", s)));
                report
            })
            .collect()
    }

    /// Возвращает количество комнат в доме
    pub fn rooms_count(&self) -> usize {
        self.rooms.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::{SmartDevice, SmartSocket, SmartTherm};

    use super::*;

    fn test_house() -> SmartHouse {
        let kitchen = Room::new(vec![SmartDevice::Therm(SmartTherm::new(22.5))]);
        let living_room = Room::new(vec![SmartDevice::Socket(SmartSocket::new(1500.0))]);

        SmartHouse::new(vec![kitchen, living_room])
    }

    #[test]
    fn test_room_access() {
        let mut house = test_house();

        let kitchen = house.get_room(0);
        assert_eq!(kitchen.devices_count(), 1);

        if let SmartDevice::Socket(s) = house.get_room_mut(1).get_device_mut(0) {
            s.turn_on();
            assert!(s.is_active())
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_invalid_index() {
        let house = test_house();
        house.get_room(5);
    }

    #[test]
    fn test_report_generation() {
        let mut house = test_house();

        if let SmartDevice::Socket(s) = house.get_room_mut(1).get_device_mut(0) {
            s.turn_on();
        }

        let report = house.report();
        assert_eq!(report.len(), 4); // 2 комнаты x (1 заголовок + 1 устройство)
        assert!(report[0].contains("Room 1"));
        assert!(report[1].contains("22.5°C"));
        assert!(report[2].contains("Room 2"));
        assert!(report[3].contains("1500.0W"));
    }

    #[test]
    fn test_rooms_count() {
        let house = test_house();
        assert_eq!(house.rooms_count(), 2);
    }
}

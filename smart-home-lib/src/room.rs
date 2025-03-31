//! Модуль для работы с комнатами умного дома

use crate::devices::SmartDevice;

/// Комната умного дома, содержащая список устройств
#[derive(Debug, Default)]
pub struct Room {
    devices: Vec<SmartDevice>,
}

impl Room {
    /// Создает новую комнату с заданными устройствами
    pub fn new(devices: Vec<SmartDevice>) -> Self {
        Self { devices }
    }

    /// Возвращает неизменяемую ссылку на устройство по индексу
    /// # Panics
    /// Паникует, если индекс выходит за пределы списка устройств
    pub fn get_device(&self, index: usize) -> &SmartDevice {
        &self.devices[index]
    }

    /// Возвращает изменяемую ссылку на устройство по индексу
    /// # Panics
    /// Паникует, если индекс выходит за пределы списка устройств
    pub fn get_device_mut(&mut self, index: usize) -> &mut SmartDevice {
        &mut self.devices[index]
    }

    /// Формирует текстовый отчет о состоянии всех устройств в комнате
    pub fn report(&self) -> Vec<String> {
        self.devices
            .iter()
            .enumerate()
            .map(|(i, d)| format!("[Device:{}] {}", i + 1, d))
            .collect()
    }

    /// Возвращает количество устройств в комнате
    pub fn devices_count(&self) -> usize {
        self.devices.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices;

    use super::*;

    fn test_room() -> Room {
        let devices = vec![
            SmartDevice::Socket(devices::SmartSocket::new(1500.0)),
            SmartDevice::Therm(devices::SmartTherm::new(22.5)),
        ];

        Room::new(devices)
    }

    #[test]
    fn test_device_access() {
        let mut room = test_room();

        let therm = room.get_device(1);
        assert!(matches!(therm, SmartDevice::Therm(_)));

        if let SmartDevice::Socket(s) = room.get_device_mut(0) {
            s.turn_on();
            assert!(s.is_active());
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_invalid_index() {
        let room = test_room();
        room.get_device(5);
    }

    #[test]
    fn test_report_generation() {
        let mut room = test_room();

        if let SmartDevice::Socket(s) = room.get_device_mut(0) {
            s.turn_on();
        }

        let report = room.report();
        assert_eq!(report.len(), 2);
        assert!(report[0].contains("1500.0W)"));
        assert!(report[1].contains("22.5°C"));
    }

    #[test]
    fn test_devices_count() {
        let room = test_room();
        assert_eq!(room.devices_count(), 2);
    }
}

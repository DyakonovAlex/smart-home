//! Протокол обмена данными между устройствами и контроллерами

pub mod socket_protocol;
pub mod therm_protocol;

pub use socket_protocol::{
    SocketCommand, SocketData, SocketResponse, receive_message, send_command,
};
pub use therm_protocol::ThermData;

use std::time::{SystemTime, UNIX_EPOCH};

/// Получает текущее время в миллисекундах с Unix epoch
/// Используется контроллерами для добавления timestamp при получении данных
pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn now_ms_returns_valid_timestamp() {
        let timestamp = now_ms();

        // Проверяем что timestamp > 0 (после Unix epoch)
        assert!(timestamp > 0);

        // Проверяем что timestamp разумный (больше 2020 года)
        // 2020-01-01 00:00:00 UTC = 1577836800000 мс
        let year_2020_ms = 1_577_836_800_000u64;
        assert!(
            timestamp > year_2020_ms,
            "Timestamp {} seems too old",
            timestamp
        );

        // Проверяем что timestamp не слишком далеко в будущем (< 2050 года)
        // 2050-01-01 00:00:00 UTC = 2524608000000 мс
        let year_2050_ms = 2_524_608_000_000u64;
        assert!(
            timestamp < year_2050_ms,
            "Timestamp {} seems too far in future",
            timestamp
        );
    }

    #[test]
    fn now_ms_monotonic() {
        // Проверяем что время идет вперед (с учетом возможной неточности)
        let t1 = now_ms();
        std::thread::sleep(Duration::from_millis(10));
        let t2 = now_ms();

        assert!(t2 >= t1, "Time should be monotonic: {} -> {}", t1, t2);

        // Разница должна быть разумной (от 10мс до 100мс)
        let diff = t2 - t1;
        assert!(diff >= 10, "Time difference too small: {}ms", diff);
        assert!(diff <= 100, "Time difference too large: {}ms", diff);
    }
}

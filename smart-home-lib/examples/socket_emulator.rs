//! Эмулятор умной розетки (имитирует реальное IoT-устройство)

use smart_home_lib::emulators::socket_emulator::{EmulatorConfig, SocketEmulator};
use std::env;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Запуск async эмулятора умной розетки...");

    // Парсим аргументы командной строки
    let args: Vec<String> = env::args().collect();
    let (device_id, tcp_address, power_rating) = parse_args(&args)?;

    println!("🏷️ ID устройства: {}", device_id);
    println!("📡 TCP адрес: {}", tcp_address);
    println!("⚡ Номинальная мощность: {:.1}W", power_rating);

    // Создаем конфигурацию эмулятора
    let config = EmulatorConfig::new(power_rating)
        .with_address(&tcp_address)
        .with_device_id(&device_id);

    // Создаем async эмулятор
    let mut emulator = SocketEmulator::new(config);

    // Запускаем неблокирующий TCP сервер
    emulator.start().await?;

    let actual_addr = emulator.local_addr()?;
    println!("✅ Async эмулятор запущен на TCP адресе: {}", actual_addr);
    println!("🌐 Принимает множественные соединения одновременно");
    println!("📊 Доступные команды через TCP:");
    println!("   • TurnOn  - включить розетку");
    println!("   • TurnOff - выключить розетку");
    println!("   • Power   - запросить текущую мощность");
    println!("\n⏹️  Нажмите Ctrl+C для graceful остановки");

    // Async ожидание сигнала завершения
    signal::ctrl_c().await?;
    println!("\n🛑 Получен сигнал остановки...");

    // Graceful async остановка
    emulator.stop().await;
    println!("✅ Эмулятор корректно остановлен");

    Ok(())
}

/// Парсит аргументы командной строки
/// Порядок: 1-device_id, 2-address, 3-power
fn parse_args(args: &[String]) -> Result<(String, String, f64), Box<dyn std::error::Error>> {
    if args.len() < 4 {
        println!(
            "📝 Использование: {} <device_id> <tcp_address> <power_rating>",
            args[0]
        );
        println!("🔧 Используем значения по умолчанию");

        return Ok((
            "smart_socket_001".to_string(),
            "127.0.0.1:3030".to_string(),
            1500.0,
        ));
    }

    let device_id = args[1].clone();
    let tcp_address = args[2].clone();

    let power_rating = args[3]
        .parse::<f64>()
        .map_err(|_| "❌ Неверное значение мощности (должно быть числом)")?;

    if power_rating <= 0.0 {
        return Err("❌ Мощность должна быть положительным числом".into());
    }

    Ok((device_id, tcp_address, power_rating))
}

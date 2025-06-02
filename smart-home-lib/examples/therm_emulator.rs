//! Эмулятор умного термометра (имитирует реальное IoT-устройство)

use smart_home_lib::emulators::{EmulationScenario, ThermEmulator};

use std::env;
use std::time::Duration;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌡️ Запуск эмулятора умного термометра...");

    // Читаем конфигурацию из аргументов командной строки
    let args: Vec<String> = env::args().collect();
    let (device_id, target_addr, initial_temp, scenario) = parse_args(&args)?;

    println!("🏷️ ID устройства: {}", device_id);
    println!("📡 Отправка данных на: {}", target_addr);
    println!("🌡️ Начальная температура: {:.1}°C", initial_temp);
    println!("📊 Сценарий: {}", scenario);

    // Создаем эмулятор
    let mut emulator = ThermEmulator::new(initial_temp)
        .with_device_id(&device_id)
        .with_scenario(scenario)
        .with_update_interval(Duration::from_secs(2));

    // Включаем сетевую отправку
    emulator.connect_to(&target_addr)?;

    // Запускаем эмулятор в фоне
    emulator.start();

    println!("✅ Эмулятор запущен! Отправляю данные каждые 2 секунды...");
    println!("Нажмите Ctrl+C для остановки");

    signal::ctrl_c().await?;

    println!("\n🛑 Получен сигнал остановки. Завершаю работу...");
    emulator.stop();

    Ok(())
}

fn parse_args(args: &[String]) -> Result<(String, String, f64, EmulationScenario), String> {
    if args.len() < 5 {
        return Ok((
            "default_device_id".to_string(),
            "127.0.0.1:8080".to_string(),
            20.0,
            EmulationScenario::Normal,
        ));
    }

    let device_id = args[1].clone();
    let target_addr = args[2].clone();

    let initial_temp = args[3]
        .parse::<f64>()
        .map_err(|_| "Invalid temperature value")?;

    let scenario = match args[4].as_str() {
        "normal" => EmulationScenario::Normal,
        "fire" => EmulationScenario::Fire,
        "freeze" => EmulationScenario::Freeze,
        "fluctuate" => EmulationScenario::Fluctuate,
        _ => EmulationScenario::Normal,
    };

    Ok((device_id, target_addr, initial_temp, scenario))
}

//! Демонстрация клиента для термометра

use smart_home_lib::prelude::*;
use std::time::Duration;
use tokio::time::{sleep, timeout};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏠 Умный дом с UDP термометрами\n");

    // Создаем контроллер для термометров
    println!("📡 Подключение к термометрам...");

    let mut kitchen_therm = ThermController::new(
        20.0,                    // Начальная температура
        "127.0.0.1:8080",        // Адрес для получения UDP
        Duration::from_secs(10), // Максимальный возраст данных
    );

    let mut living_therm = ThermController::new(
        20.0,                    // Начальная температура
        "127.0.0.1:8081",        // Адрес для получения UDP
        Duration::from_secs(10), // Максимальный возраст данных
    );

    // Запускаем автоматическое обновление
    kitchen_therm.start();
    living_therm.start();

    println!("✅ Термометры подключены и слушают UDP пакеты");
    println!("💡 Запустите эмуляторы в отдельных терминалах:");
    println!("   cargo run --example therm_emulator kitchen_001 127.0.0.1:8080 22.5 normal");
    println!("   cargo run --example therm_emulator living_001 127.0.0.1:8081 24.0 fluctuate\n");

    // Демонстрация polling (опрос)
    println!("=== Polling Demo ===");
    for i in 1..=5 {
        println!("Опрос #{}", i);

        // Синхронное получение
        match kitchen_therm.temperature() {
            Ok(temp) => println!("  Кухня: {}", temp),
            Err(e) => println!("  Кухня: Нет данных - {}", e),
        }

        match living_therm.temperature() {
            Ok(temp) => println!("  Гостиная: {}", temp),
            Err(e) => println!("  Гостиная: Нет данных - {}", e),
        }

        sleep(Duration::from_secs(3)).await;
    }

    // Демонстрация async/await
    println!("\n=== Async/Await Demo ===");
    println!("Ожидание новых данных (таймаут 5 секунд)...");

    let kitchen_future = kitchen_therm.wait_for_new_data();
    let living_future = living_therm.wait_for_new_data();

    match timeout(Duration::from_secs(5), kitchen_future).await {
        Ok(Ok(temp)) => println!("✅ Кухня (async): {}", temp),
        Ok(Err(e)) => println!("❌ Кухня (async): {}", e),
        Err(_) => println!("⏱️ Кухня (async): Таймаут"),
    }

    match timeout(Duration::from_secs(5), living_future).await {
        Ok(Ok(temp)) => println!("✅ Гостиная (async): {}", temp),
        Ok(Err(e)) => println!("❌ Гостиная (async): {}", e),
        Err(_) => println!("⏱️ Гостиная (async): Таймаут"),
    }

    // Демонстрация callbacks
    println!("\n=== Callbacks Demo ===");

    println!("=== Short Subscription Demo ===");
    println!("Подписка на изменения температуры (3 сек)...");
    let subscription = kitchen_therm.on_temperature_change(|result| {
        if let Ok(temp) = result {
            println!("🌡️ Короткая подписка: {}", temp);
        }
    });

    tokio::time::sleep(Duration::from_secs(3)).await;
    subscription.unsubscribe();
    println!("✅ Отписались, больше уведомлений не будет");

    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("⏰ Прошло еще 3 секунды - уведомлений нет!");

    println!("=== Long Subscription Demo ===");
    println!("Подписка на изменения температуры (10 сек)...");

    let kitchen_subscription = kitchen_therm.on_temperature_change(|result| match result {
        Ok(temp) => println!("🔥 Кухня: температура изменилась: {}", temp),
        Err(e) => println!("❌ Кухня: ошибка: {}", e),
    });

    let living_subscription = living_therm.on_temperature_change(|result| match result {
        Ok(temp) => println!("🔥 Гостиная: температура изменилась: {}", temp),
        Err(e) => println!("❌ Гостиная: ошибка: {}", e),
    });

    sleep(Duration::from_secs(10)).await;
    kitchen_subscription.unsubscribe();
    living_subscription.unsubscribe();

    // Демонстрация интеграции с умным домом
    println!("\n=== Интеграция с умным домом ===");

    // Создаем дом с UDP термометрами
    let house = house![
        (
            "кухня",
            room![("Кухня термометр", DeviceController::Therm(kitchen_therm)),]
        ),
        (
            "гостиная",
            room![("Гостиная термометр", DeviceController::Therm(living_therm)),]
        )
    ];

    // Выводим отчет дома
    for i in 1..=5 {
        println!("Опрос #{}", i);
        println!("{}", house);
        sleep(Duration::from_secs(3)).await;
    }

    println!("\n🏁 Демонстрация завершена!");

    Ok(())
}

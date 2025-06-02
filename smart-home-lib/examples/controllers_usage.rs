//! Пример работы с сетевыми контроллерами умного дома
//!
//! Демонстрирует управление реальными устройствами через TCP (розетки) и UDP (термометры).
//!
//! Для тестирования запустите эмуляторы в отдельных терминалах:
//!
//! **Термометры (UDP):**
//! ```bash
//! cargo run --example therm_emulator kitchen_therm_001 127.0.0.1:4001 22.5 normal
//! cargo run --example therm_emulator ac_therm_001 127.0.0.1:4002 24.0 normal
//! ```
//!
//! **Розетки (TCP):**
//! ```bash
//! cargo run --example socket_emulator kettle_001 127.0.0.1:3001 2000.0
//! cargo run --example socket_emulator tv_001 127.0.0.1:3002 150.0
//! ```
//!
//! Затем запустите этот пример:
//! ```bash
//! cargo run --example controllers_usage
//! ```

use smart_home_lib::prelude::*;
use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;

/// Создает кухню с сетевыми контроллерами
fn create_kitchen() -> Room {
    let mut room = Room::new();

    // TCP контроллер для чайника
    let kettle_addr: SocketAddr = "127.0.0.1:3001".parse().unwrap();
    let kettle_controller = SocketController::new(
        kettle_addr,
        2000.0, // 2кВт чайник
        Duration::from_secs(3),
    );
    room.add_controller("чайник", DeviceController::Socket(kettle_controller));

    // UDP контроллер для термометра
    let kitchen_therm = ThermController::new(22.5, "127.0.0.1:4001", Duration::from_secs(5));
    room.add_controller("термометр", DeviceController::Therm(kitchen_therm));

    room
}

/// Создает гостиную с сетевыми контроллерами
fn create_living_room() -> Room {
    let mut room = Room::new();

    // TCP контроллер для телевизора
    let tv_addr: SocketAddr = "127.0.0.1:3002".parse().unwrap();
    let tv_controller = SocketController::new(
        tv_addr,
        150.0, // 150Вт телевизор
        Duration::from_secs(3),
    );
    room.add_controller("телевизор", DeviceController::Socket(tv_controller));

    // UDP контроллер для кондиционера
    let ac_therm = ThermController::new(24.0, "127.0.0.1:4002", Duration::from_secs(10));
    room.add_controller("кондиционер", DeviceController::Therm(ac_therm));

    room
}

/// Демонстрирует управление розетками через TCP
async fn demo_socket_controllers(house: &mut SmartHouse) -> Result<(), Box<dyn Error>> {
    println!("🔌 === Управление розетками через TCP ===");

    // Управление чайником
    println!("\n☕ Управление чайником:");
    if let Ok(DeviceController::Socket(kettle)) = house.controller_mut("кухня", "чайник")
    {
        println!("📡 Подключение к чайнику...");

        match kettle.turn_on().await {
            Ok(_) => {
                println!("✅ Чайник включен");

                // Проверяем мощность
                match kettle.power().await {
                    Ok(power) => println!("⚡ Мощность чайника: {}", power),
                    Err(e) => println!("❌ Ошибка получения мощности: {}", e),
                }

                // Ждем 2 секунды, затем выключаем
                sleep(Duration::from_secs(2)).await;

                match kettle.turn_off().await {
                    Ok(_) => println!("🔴 Чайник выключен"),
                    Err(e) => println!("❌ Ошибка выключения: {}", e),
                }
            }
            Err(e) => println!("❌ Ошибка включения чайника: {}", e),
        }
    }

    // Управление телевизором
    println!("\n📺 Управление телевизором:");
    if let Ok(DeviceController::Socket(tv)) = house.controller_mut("гостиная", "телевизор")
    {
        println!("📡 Подключение к телевизору...");

        match tv.turn_on().await {
            Ok(_) => {
                println!("✅ Телевизор включен");

                match tv.power().await {
                    Ok(power) => println!("⚡ Потребление телевизора: {}", power),
                    Err(e) => println!("❌ Ошибка получения мощности: {}", e),
                }
            }
            Err(e) => println!("❌ Ошибка включения телевизора: {}", e),
        }
    }

    Ok(())
}

/// Демонстрирует мониторинг термометров через UDP
async fn demo_therm_controllers(house: &mut SmartHouse) -> Result<(), Box<dyn Error>> {
    println!("\n🌡️ === Мониторинг термометров через UDP ===");

    // Запускаем термометр на кухне
    println!("\n🍳 Мониторинг температуры на кухне:");
    if let Ok(DeviceController::Therm(kitchen_therm)) = house.controller_mut("кухня", "термометр")
    {
        println!("📡 Подключение к термометру кухни...");

        kitchen_therm.start();
        println!("🌡️ Термометр кухни запущен");

        // Собираем несколько показаний
        for i in 1..=3 {
            sleep(Duration::from_secs(2)).await;

            match kitchen_therm.temperature() {
                Ok(temp) => println!("📊 Показание #{}: {}", i, temp),
                Err(e) => println!("❌ Ошибка показания #{}: {}", i, e),
            }
        }

        kitchen_therm.stop();
        println!("🛑 Термометр кухни остановлен");
    }

    // Запускаем термометр кондиционера
    println!("\n❄️ Мониторинг кондиционера:");
    if let Ok(DeviceController::Therm(ac_therm)) = house.controller_mut("гостиная", "кондиционер")
    {
        println!("📡 Подключение к термометру кондиционера...");

        ac_therm.start();
        println!("🌡️ Термометр кондиционера запущен");

        sleep(Duration::from_secs(3)).await;

        match ac_therm.temperature() {
            Ok(temp) => {
                println!("📊 Температура кондиционера: {}", temp);

                // Имитируем корректировку температуры
                if temp.value() > 25.0 {
                    println!("🔥 Температура высокая, кондиционер работает интенсивнее");
                } else if temp.value() < 20.0 {
                    println!("🧊 Температура низкая, кондиционер работает слабее");
                } else {
                    println!("✅ Температура в норме");
                }
            }
            Err(e) => println!("❌ Ошибка получения температуры: {}", e),
        }

        ac_therm.stop();
        println!("🛑 Термометр кондиционера остановлен");
    }

    Ok(())
}

/// Демонстрирует работу с контроллерами через обобщенный интерфейс
async fn demo_generic_controllers(house: &SmartHouse) {
    println!("\n🔧 === Обобщенная работа с контроллерами ===");

    for room_name in house.rooms_keys() {
        if let Some(room) = house.room(&room_name) {
            println!("\n🏠 Комната: {}", room_name);
            println!("🌐 Контроллеров в комнате: {}", room.controllers_count());

            for controller_key in room.controllers_keys() {
                if let Some(controller) = room.controller(&controller_key) {
                    println!("  📱 {}: {}", controller_key, controller.report());
                }
            }
        }
    }
}

/// Демонстрирует обработку ошибок соединения
async fn demo_connection_errors(house: &mut SmartHouse) {
    println!("\n❌ === Демонстрация обработки ошибок ===");

    // Создаем контроллер с неверным адресом
    let mut temp_room = Room::new();
    let broken_socket = SocketController::new(
        "127.0.0.1:9999".parse().unwrap(), // Несуществующий порт
        1000.0,
        Duration::from_secs(1), // Короткий таймаут
    );
    temp_room.add_controller("broken_socket", DeviceController::Socket(broken_socket));

    if let Some(DeviceController::Socket(socket)) = temp_room.controller_mut("broken_socket") {
        println!("🔌 Попытка подключения к несуществующей розетке...");

        match socket.turn_on().await {
            Ok(_) => println!("✅ Неожиданно удалось подключиться"),
            Err(e) => println!("❌ Ожидаемая ошибка: {}", e),
        }
    }

    // Пытаемся получить доступ к несуществующему контроллеру
    match house.controller("несуществующая_комната", "контроллер") {
        Ok(_) => println!("✅ Неожиданно нашли контроллер"),
        Err(e) => println!("❌ Ожидаемая ошибка: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🏡 Умный дом: сетевые контроллеры");
    println!("═══════════════════════════════════");

    // Создаем дом только с сетевыми контроллерами
    let mut house = house![
        ("кухня", create_kitchen()),
        ("гостиная", create_living_room()),
    ];

    println!("🏗️ Создан умный дом с сетевыми контроллерами:");
    println!("   🍳 Кухня: чайник (TCP) + термометр (UDP)");
    println!("   🛋️ Гостиная: телевизор (TCP) + кондиционер (UDP)");

    // Правильные команды запуска эмуляторов
    println!("\n💡 Запустите эмуляторы в отдельных терминалах:");
    println!("📋 Термометры (UDP):");
    println!(
        "   🌡️ cargo run --example therm_emulator kitchen_therm_001 127.0.0.1:4001 22.5 normal"
    );
    println!("   🌡️ cargo run --example therm_emulator ac_therm_001 127.0.0.1:4002 24.0 normal");
    println!("📋 Розетки (TCP):");
    println!("   🔌 cargo run --example socket_emulator kettle_001 127.0.0.1:3001 2000.0");
    println!("   🔌 cargo run --example socket_emulator tv_001 127.0.0.1:3002 150.0");

    // Небольшая пауза для подготовки
    println!("\n⏳ Начинаем через 5 секунд (время для запуска эмуляторов)...");
    sleep(Duration::from_secs(5)).await;

    // Демонстрации
    demo_socket_controllers(&mut house).await?;
    demo_therm_controllers(&mut house).await?;
    demo_generic_controllers(&house).await;
    demo_connection_errors(&mut house).await;

    // Итоговый отчет
    println!("\n📋 === Итоговый отчет контроллеров ===");
    println!("{}", house.report());

    println!("\n✅ Демонстрация завершена!");
    Ok(())
}

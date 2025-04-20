use smart_home_lib::prelude::*;
use std::error::Error;

fn print_report(reporter: &impl Reporter) {
    println!("{}", reporter);
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Умный дом: демонстрация возможностей ===\n");

    // Демонстрация создания дома с помощью макроса
    println!("Создание дома с помощью макроса house!:");
    let mut house = house![
        (
            "кухня",
            room![
                ("термометр", SmartDevice::Therm(SmartTherm::new(22.5))),
                ("чайник", SmartDevice::Socket(SmartSocket::new(2000.0)))
            ]
        ),
        (
            "гостиная",
            room![
                ("розетка", SmartDevice::Socket(SmartSocket::new(1500.0))),
                ("кондиционер", SmartDevice::Therm(SmartTherm::new(24.0)))
            ]
        )
    ];

    // Вывод начального отчета
    println!("\n=== Первоначальный отчет о доме ===");
    print_report(&house);

    // Демонстрация динамического добавления комнаты
    println!("\n=== Динамическое добавление комнаты ===");
    house.add_room(
        "спальня",
        room![("ночник", SmartDevice::Socket(SmartSocket::new(60.0)))],
    );
    println!("Комната 'спальня' добавлена в дом");
    println!("Список комнат в доме: {:?}", house.rooms_keys());

    // Демонстрация динамического добавления устройства в существующую комнату
    println!("\n=== Динамическое добавление устройства ===");
    if let Some(kitchen) = house.get_room_mut("кухня") {
        kitchen.add_device("холодильник", SmartDevice::Socket(SmartSocket::new(150.0)));
        println!("Устройство 'холодильник' добавлено в комнату 'кухня'");
        println!("Список устройств в 'кухня': {:?}", kitchen.devices_keys());
    }

    // Управление устройством и вывод отчета одного устройства
    println!("\n=== Управление устройством и отчет о нем ===");
    match house.get_device_mut("гостиная", "розетка") {
        Ok(device) => {
            if let SmartDevice::Socket(socket) = device {
                socket.turn_on();
                println!("Розетка в гостиной включена");
                print_report(device);
            }
        }
        Err(e) => println!("Ошибка: {}", e),
    }

    // Вывод отчета об отдельной комнате
    println!("\n=== Отчет об отдельной комнате ===");
    if let Some(room) = house.get_room("кухня") {
        print_report(room);
    }

    // Демонстрация обработки ошибок при доступе к устройствам
    println!("\n=== Обработка ошибок ===");
    match house.get_device("ванная", "бойлер") {
        Ok(_) => println!("Устройство найдено"),
        Err(e) => println!("Ошибка: {}", e),
    }

    match house.get_device("кухня", "телевизор") {
        Ok(_) => println!("Устройство найдено"),
        Err(e) => println!("Ошибка: {}", e),
    }

    // Демонстрация удаления устройства
    println!("\n=== Удаление устройства ===");
    if let Some(room) = house.get_room_mut("гостиная") {
        if let Some(removed) = room.remove_device("кондиционер") {
            println!("Устройство удалено: {}", removed);
            println!("Оставшиеся устройства: {:?}", room.devices_keys());
        }
    }

    // Демонстрация удаления комнаты
    println!("\n=== Удаление комнаты ===");
    if let Some(removed_room) = house.remove_room("спальня") {
        println!(
            "Комната 'спальня' удалена, в ней было {} устройств",
            removed_room.devices_count()
        );
        println!("Оставшиеся комнаты: {:?}", house.rooms_keys());
    }

    // Итоговый отчет
    println!("\n=== Итоговый отчет о доме ===");
    print_report(&house);

    Ok(())
}

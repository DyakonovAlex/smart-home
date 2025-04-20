# Smart Home Library 🏠⚡

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Rust библиотека для управления умным домом. Проект создан в учебных целях.

## Особенности

- 📦 Модульная архитектура
- 🛠 Поддержка устройств: розетки, термометры
- 📊 Генерация отчетов
- 🧪 Полное покрытие тестами
- 🚀 Готовность к расширению (клиент/сервер, веб-интерфейс)

## Установка

Добавьте в `Cargo.toml`:

```toml
[dependencies]
smart-home-lib = "0.1"
```

Или для локальной разработки:

```toml
smart-home-lib = { path = "../smart-home-lib" }
```

## Пример использования

```rust
use smart_home_lib::prelude::*;
use std::error::Error;

// Функция для вывода отчета от любого объекта, реализующего Reporter
fn print_report(reporter: &impl Reporter) {
    println!("{}", reporter);
}

fn main() -> Result<(), Box<dyn Error>> {
    // Создание дома с использованием макросов
    let mut house = house![
        (
            "kitchen",
            room![
                ("therm", SmartDevice::Therm(SmartTherm::new(22.5))),
                ("socket", SmartDevice::Socket(SmartSocket::new(1500.0)))
            ]
        ),
        (
            "living_room",
            room![
                ("socket", SmartDevice::Socket(SmartSocket::new(2000.0)))
            ]
        )
    ];

    // Вывод начального отчета
    println!("Initial report:");
    print_report(&house);
    
    // Включение розетки в кухне
    if let Ok(device) = house.get_device_mut("kitchen", "socket") {
        if let SmartDevice::Socket(socket) = device {
            socket.turn_on();
            println!("\nKitchen socket turned on");
        }
    }
    
    // Динамическое добавление новой комнаты
    house.add_room("bedroom", room![
        ("lamp", SmartDevice::Socket(SmartSocket::new(60.0)))
    ]);
    
    // Вывод обновленного отчета
    println!("\nUpdated report:");
    print_report(&house);
    
    Ok(())
}
```

Пример демонстрирует основные возможности библиотеки:

- 🧩 Создание структуры умного дома с помощью макросов
- 🔑 Доступ к устройствам по ключам
- 🔌 Управление состоянием устройств
- 📝 Генерация отчетов через универсальный интерфейс
- ➕ Динамическое добавление комнат и устройств
- ⚠️ Безопасную обработку ошибок

Более подробные примеры использования можно найти в директории `examples/`.

## Разработка

### Сборка

```bash
cargo build
```

### Тестирование

```bash
cargo test --all-features
```

### Проверка стиля

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

### Запуск примеров

```bash
cargo run --example basic_usage
```

## Лицензия

MIT © [DyakonovAlex](https://github.com/DyakonovAlex).  
Пожалуйста, соблюдайте условия лицензии при использовании кода.

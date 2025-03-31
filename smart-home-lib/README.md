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

fn main() {
    let mut house = SmartHouse::new(vec![
        Room::new(vec![
            SmartDevice::Therm(SmartTherm::new(22.5)),
            SmartDevice::Socket(SmartSocket::new(1500.0)),
        ])
    ]);

    println!("Initial report:\n{}", house.report().join("\n"));
    
    house.get_room_mut(0).get_device_mut(1).turn_on();
    println!("Updated report:\n{}", house.report().join("\n"));
}
```

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

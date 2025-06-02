# Smart Home Library 🏠⚡

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Rust библиотека для управления умным домом. Проект создан в учебных целях.

## Архитектура

### 🏗️ Ядро системы
- **`devices/`** - Умные устройства (розетки, термометры)
- **`room.rs`** - Комнаты с HashMap устройств
- **`house.rs`** - Умный дом с HashMap комнат  
- **`units/`** - Типобезопасные единицы измерения (Watts, Celsius)
- **`traits.rs`** - Общие интерфейсы (Reporter)

### 🌐 Сетевой слой
- **`protocol/`** - Async протоколы TCP/UDP для коммуникации
- **`controllers/`** - Контроллеры для управления устройствами по сети
- **`emulators/`** - TCP/UDP эмуляторы устройств для тестирования

## Особенности

- 📦 **Модульная архитектура** с четким разделением ответственности
- 🛠 **Поддержка устройств**: розетки, термометры
- 🔑 **HashMap-based storage** для доступа по ключам
- 🧩 **Макросы** `room![]` и `house![]` для упрощенного создания
- 📊 **Единый интерфейс отчетов** через трейт `Reporter`
- 🌐 **Async TCP/UDP протоколы** для сетевого взаимодействия
- 🧪 **Эмуляторы устройств** для разработки без реального железа
- 🎯 **Типобезопасность** с newtype паттернами (Watts, Celsius)
- 🔍 **Полное покрытие тестами** (unit + integration)

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

## Примеры

В директории `examples/` доступны следующие примеры:

- **`basic_usage.rs`** - Основы работы с библиотекой
- **`controllers_usage.rs`** - Использование сетевых контроллеров
- **`socket_emulator.rs`** - Запуск TCP эмулятора розетки
- **`therm_emulator.rs`** - Запуск UDP эмулятора термометра
- **`socket_client.rs`** - TCP клиент для управления розеткой
- **`therm_client.rs`** - UDP клиент для чтения термометра
- **`udp_listener.rs`** - UDP сервер для приема данных

### Запуск примеров

#### Основное использование
```bash
cargo run --example basic_usage
```

#### Сетевые контроллеры

Для демонстрации работы с реальными устройствами через TCP (розетки) и UDP (термометры) необходимо запустить эмуляторы в отдельных терминалах:

**1. Запустите эмуляторы термометров (UDP):**
```bash
# Терминал 1
cargo run --example therm_emulator kitchen_therm_001 127.0.0.1:4001 22.5 normal

# Терминал 2  
cargo run --example therm_emulator ac_therm_001 127.0.0.1:4002 24.0 normal
```

**2. Запустите эмуляторы розеток (TCP):**
```bash
# Терминал 3
cargo run --example socket_emulator kettle_001 127.0.0.1:3001 2000.0

# Терминал 4
cargo run --example socket_emulator tv_001 127.0.0.1:3002 150.0
```

**3. Запустите пример контроллеров:**
```bash
# Терминал 5
cargo run --example controllers_usage
```

#### Отдельные клиенты

```bash
# TCP клиент для розетки
cargo run --example socket_client

# UDP клиент для термометра
cargo run --example therm_client

# UDP сервер
cargo run --example udp_listener
```

## Разработка

### Сборка

```bash
cargo build
```

### Тестирование

```bash
# Все тесты
cargo test

# Только unit тесты (быстро)
cargo test --lib

# Интеграционные тесты с сетью
cargo test -- --ignored
```

### Проверка стиля

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

## Модули

| Модуль | Описание |
|--------|----------|
| `devices` | Умные устройства (розетки, термометры) |
| `room` | Комнаты с устройствами |
| `house` | Умный дом с комнатами |
| `protocol` | Async протоколы TCP/UDP |
| `controllers` | Сетевые контроллеры устройств |
| `emulators` | Эмуляторы для тестирования |
| `units` | Типобезопасные единицы измерения |
| `traits` | Общие интерфейсы |

## Лицензия

MIT © [DyakonovAlex](https://github.com/DyakonovAlex).  
Пожалуйста, соблюдайте условия лицензии при использовании кода.

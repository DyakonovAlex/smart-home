# Smart Home Library 🏠⚡

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Rust библиотека для управления умным домом с поддержкой сетевых протоколов.  
Проект создан в учебных целях.

## Особенности

- **📦 Ядро системы**: Устройства, комнаты, умный дом с HashMap storage
- **🌐 Сетевые протоколы**: Async TCP/UDP для коммуникации с устройствами  
- **🎮 Контроллеры**: Управление устройствами через сеть
- **🧪 Эмуляторы**: TCP/UDP эмуляторы для разработки без реального железа
- **🔑 HashMap доступ**: Устройства и комнаты доступны по строковым ключам
- **🧩 Макросы**: `room![]` и `house![]` для упрощенного создания структур
- **🎯 Типобезопасность**: Newtype паттерны для единиц измерения
- **🧪 Полное тестирование**: Unit и интеграционные тесты

## Структура проекта

```
smart-home/
├── Cargo.toml              # Workspace configuration  
├── README.md               # Этот файл
└── smart-home-lib/         # Основная библиотека
    ├── src/
    │   ├── devices/        # Умные устройства
    │   ├── protocol/       # TCP/UDP протоколы
    │   ├── controllers/    # Сетевые контроллеры
    │   ├── emulators/      # Эмуляторы устройств
    │   └── units/          # Типобезопасные единицы
    └── examples/           # Примеры использования
```

## Быстрый старт

### Предварительные требования

- Rust 1.70+
- Cargo

### Установка

```bash
git clone https://github.com/DyakonovAlex/smart-home.git
cd smart-home
```

### Сборка

```bash
cargo build
```

### Базовый пример

```bash
cargo run --example basic_usage
```

### Сетевые примеры

Демонстрация работы с реальными устройствами через TCP (розетки) и UDP (термометры):

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

**3. Запустите контроллеры:**

```bash
# Терминал 5
cargo run --example controllers_usage
```

### Тестирование

```bash
# Все тесты
cargo test

# Только быстрые unit тесты
cargo test --lib

# Интеграционные тесты с сетью
cargo test -- --ignored
```

## Архитектура

### Ядро системы

- **Устройства**: Розетки и термометры с типобезопасными единицами
- **Комнаты**: HashMap устройств с доступом по ключам  
- **Дом**: HashMap комнат с безопасным доступом к устройствам

### Сетевой слой

- **Протоколы**: Async TCP для розеток, UDP для термометров
- **Контроллеры**: Высокоуровневые интерфейсы управления
- **Эмуляторы**: Виртуальные устройства для тестирования

## Примеры использования

Все примеры находятся в `smart-home-lib/examples/`:

| Пример | Описание |
|--------|----------|
| `basic_usage.rs` | Основы работы с библиотекой |
| `controllers_usage.rs` | Сетевые контроллеры |
| `socket_emulator.rs` | TCP эмулятор розетки |
| `therm_emulator.rs` | UDP эмулятор термометра |
| `socket_client.rs` | TCP клиент для розетки |
| `therm_client.rs` | UDP клиент для термометра |
| `udp_listener.rs` | UDP сервер |

## Разработка

### Code Style

```bash
cargo fmt --all
cargo clippy --all-targets
```

### CI/CD

- Форматирование: `cargo fmt --all -- --check`
- Линтинг: `cargo clippy --all-targets -- -D warnings`  
- Тесты: `cargo test`

### Документация

Подробная документация доступна в [smart-home-lib/README.md](./smart-home-lib/README.md).

## Лицензия

MIT © [DyakonovAlex](https://github.com/DyakonovAlex).  
Пожалуйста, соблюдайте условия лицензии при использовании кода.

## Вклад в проект

1. Форкните репозиторий
2. Создайте feature-ветку  
3. Отправьте Pull Request с описанием изменений

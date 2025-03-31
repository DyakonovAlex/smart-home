# Smart Home Ecosystem 🏠⚡🌐

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Модульная экосистема для управления умным домом на Rust.  
Проект разработан в учебных целях с возможностью расширения до production-решения.

## Особенности

- **📦 Ядро системы**: Библиотека с базовой логикой (устройства, комнаты, отчеты)
- **🌐 Сетевой слой**: Готовность к реализации клиент-серверного взаимодействия
- **🔌 Расширяемость**: Поддержка плагинов и новых типов устройств
- **🧪 Тестирование**: Полное покрытие модульными и интеграционными тестами

## Компоненты

| Крейт                | Тип       | Описание                          | Статус       |
|----------------------|-----------|-----------------------------------|--------------|
| [smart-home-lib]     | Библиотека | Ядро системы (устройства, комнаты) | В работе    |
| [smart-home-server]  | Сервер    | REST/WebSocket API                | Планируется |
| [smart-home-client]  | Клиент    | CLI-утилита управления            | Планируется  |
| [smart-home-ui]      | Web UI    | Интерфейс на Yew/WASM             | Планируется     |

[smart-home-lib]: ./smart-home-lib/README.md
[smart-home-server]: ./smart-home-server/README.md
[smart-home-client]: ./smart-home-client/README.md
[smart-home-ui]: ./smart-home-ui/README.md

## Быстрый старт

### Предварительные требования

- Rust 1.85+
- Cargo
- Docker (опционально для будущих компонентов)

### Установка

```bash
git clone https://github.com/DyakonovAlex/smart-home.git
cd smart-home
```

### Сборка всех компонентов

```bash
cargo build --workspace
```

### Запуск примеров

```bash
cargo run --package smart-home-lib --example basic_usage
```

### Тестирование

```bash
cargo test --workspace
```

## Документация

- [API ядра](https://DyakonovAlex.github.io/smart-home/smart_home_lib)
- [Схема протоколов](docs/PROTOCOL.md) (в разработке)
- [Архитектура](docs/ARCHITECTURE.md) (в разработке)

## Лицензия

Проект распространяется под лицензией MIT.  

## Разработка

### Code Style

```bash
cargo fmt --all
cargo clippy --workspace --all-targets
```

### CI/CD

- Форматирование: `cargo fmt --all -- --check`
- Линтинг: `cargo clippy --workspace --all-targets -- -D warnings`
- Тесты: `cargo test --workspace`

### Вклад в проект

1. Форкните репозиторий
2. Создайте feature-ветку
3. Отправьте Pull Request с описанием изменений

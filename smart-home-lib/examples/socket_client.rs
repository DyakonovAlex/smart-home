//! Простой TCP клиент для тестирования эмулятора умной розетки

use smart_home_lib::protocol::socket_protocol::{SocketCommand, send_command_and_receive};
use std::env;
use std::error::Error;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Парсинг аргументов
    let server_addr = if args.len() >= 2 {
        &args[1]
    } else {
        "127.0.0.1:3030" // Дефолтный адрес
    };

    println!("🔌 TCP клиент для тестирования умной розетки");
    println!("📡 Подключение к серверу: {}", server_addr);

    // Подключаемся к эмулятору
    let mut stream = match TcpStream::connect(server_addr).await {
        Ok(stream) => {
            println!("✅ Подключение установлено!");
            stream
        }
        Err(e) => {
            eprintln!("❌ Ошибка подключения: {}", e);
            eprintln!("💡 Убедитесь что эмулятор запущен на {}", server_addr);
            return Err(e.into());
        }
    };

    println!("\n🧪 Начинаем тестирование...\n");

    // Тестируем команды
    let test_commands = vec![
        ("Запрос текущего состояния", SocketCommand::Power),
        ("Включение розетки", SocketCommand::TurnOn),
        ("Запрос состояния после включения", SocketCommand::Power),
        ("Выключение розетки", SocketCommand::TurnOff),
        ("Запрос состояния после выключения", SocketCommand::Power),
    ];

    for (description, command) in test_commands {
        println!("📤 {}: {:?}", description, command);

        match send_command_and_receive(&mut stream, &command).await {
            Ok(response) => {
                println!("📥 Ответ: {}", format_response(&response));
            }
            Err(e) => {
                eprintln!("❌ Ошибка: {}", e);
                break;
            }
        }

        println!(); // Пустая строка для читаемости

        // Небольшая пауза между командами
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    println!("✨ Тестирование завершено!");
    println!("🔌 Соединение будет закрыто автоматически");

    Ok(())
}

/// Форматирует ответ от сервера для красивого вывода
fn format_response(response: &smart_home_lib::protocol::socket_protocol::SocketResponse) -> String {
    use smart_home_lib::protocol::socket_protocol::SocketResponse;

    match response {
        SocketResponse::Ok(data) => {
            let status = if data.active {
                "🟢 ВКЛЮЧЕНА"
            } else {
                "🔴 ВЫКЛЮЧЕНА"
            };
            let device_id = data.device_id.as_deref().unwrap_or("unknown");

            format!(
                "✅ Успех | Устройство: {} | Статус: {} | Мощность: {:.1}W",
                device_id, status, data.power
            )
        }
        SocketResponse::Error { message } => {
            format!("❌ Ошибка: {}", message)
        }
    }
}

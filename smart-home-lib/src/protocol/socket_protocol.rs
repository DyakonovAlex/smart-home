//! Async протокол TCP для управления умной розеткой

use serde::{Deserialize, Serialize};
use std::io::Result as IoResult;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Команды для управления розеткой
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(tag = "command")]
pub enum SocketCommand {
    #[serde(rename = "turn_on")]
    TurnOn,
    #[serde(rename = "turn_off")]
    TurnOff,
    #[serde(rename = "power")]
    Power,
}

/// Ответы от розетки
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "result")]
pub enum SocketResponse {
    #[serde(rename = "ok")]
    Ok(SocketData),
    #[serde(rename = "error")]
    Error { message: String },
}

/// Данные от розетки (примитивные типы, которые железка реально отправляет)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocketData {
    pub active: bool, // включена ли подача питания
    pub power: f64,   // текущее потребление в ваттах (как число)
    pub device_id: Option<String>,
}

/// Async отправка сообщения с length-prefix
pub async fn send_message<W>(writer: &mut W, message: &str) -> IoResult<()>
where
    W: AsyncWrite + Unpin,
{
    let bytes = message.as_bytes();
    let length = bytes.len() as u32;

    // Отправляем длину (4 байта, big-endian)
    writer.write_all(&length.to_be_bytes()).await?;

    // Отправляем данные
    writer.write_all(bytes).await?;

    // Сбрасываем буфер
    writer.flush().await?;

    Ok(())
}

/// Async получение сообщения с length-prefix
pub async fn receive_message<R>(reader: &mut R) -> IoResult<String>
where
    R: AsyncRead + Unpin,
{
    // Читаем длину (4 байта)
    let mut length_bytes = [0u8; 4];
    reader.read_exact(&mut length_bytes).await?;
    let length = u32::from_be_bytes(length_bytes) as usize;

    // Проверяем разумный размер сообщения (защита от DoS)
    if length > 1024 * 1024 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Message too large",
        ));
    }

    // Читаем точно столько данных сколько указано
    let mut buffer = vec![0u8; length];
    reader.read_exact(&mut buffer).await?;

    // Конвертируем в строку
    String::from_utf8(buffer).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

/// Async отправка команды
pub async fn send_command<W>(writer: &mut W, command: &SocketCommand) -> IoResult<()>
where
    W: AsyncWrite + Unpin,
{
    // Сериализуем команду
    let json_command = serde_json::to_string(command)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // Отправляем
    send_message(writer, &json_command).await
}

/// Async получение ответа
pub async fn receive_response<R>(reader: &mut R) -> IoResult<SocketResponse>
where
    R: AsyncRead + Unpin,
{
    // Получаем ответ
    let response_json = receive_message(reader).await?;

    // Парсим ответ
    serde_json::from_str(&response_json)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

/// Async отправка команды и получение ответа
pub async fn send_command_and_receive<S>(
    stream: &mut S,
    command: &SocketCommand,
) -> IoResult<SocketResponse>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    // Отправляем команду
    send_command(stream, command).await?;

    // Получаем ответ
    receive_response(stream).await
}

/// Async отправка ответа
pub async fn send_response<W>(writer: &mut W, response: &SocketResponse) -> IoResult<()>
where
    W: AsyncWrite + Unpin,
{
    // Сериализуем ответ
    let json_response = serde_json::to_string(response)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // Отправляем
    send_message(writer, &json_response).await
}

/// Async получение команды
pub async fn receive_command<R>(reader: &mut R) -> IoResult<SocketCommand>
where
    R: AsyncRead + Unpin,
{
    // Получаем команду
    let command_json = receive_message(reader).await?;

    // Парсим команду
    serde_json::from_str(&command_json)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::duplex;

    #[tokio::test]
    #[ignore = "integration test with async networking"]
    async fn test_send_receive_message() {
        let (mut client, mut server) = duplex(1024);
        let test_message = "Hello, async world!";

        // Отправляем сообщение
        let client_task = tokio::spawn(async move {
            send_message(&mut client, test_message).await.unwrap();
        });

        // Получаем сообщение
        let received = receive_message(&mut server).await.unwrap();
        assert_eq!(received, test_message);

        client_task.await.unwrap();
    }

    #[tokio::test]
    #[ignore = "integration test with async networking"]
    async fn test_send_receive_command() {
        let (mut client, mut server) = duplex(1024);
        let command = SocketCommand::TurnOn;

        // Отправляем команду
        let client_task = tokio::spawn(async move {
            send_command(&mut client, &command).await.unwrap();
        });

        // Получаем команду
        let received = receive_command(&mut server).await.unwrap();
        assert_eq!(received, command);

        client_task.await.unwrap();
    }

    #[tokio::test]
    #[ignore = "integration test with async networking"]
    async fn test_send_receive_response() {
        let (mut client, mut server) = duplex(1024);
        let response = SocketResponse::Ok(SocketData {
            active: true,
            power: 1500.0,
            device_id: Some("test_socket".to_string()),
        });

        // Отправляем ответ
        let response_clone = response.clone();
        let client_task = tokio::spawn(async move {
            send_response(&mut client, &response_clone).await.unwrap();
        });

        // Получаем ответ
        let received = receive_response(&mut server).await.unwrap();
        assert_eq!(received, response);

        client_task.await.unwrap();
    }

    #[tokio::test]
    #[ignore = "integration test with async networking"]
    async fn test_full_command_response_cycle() {
        let (mut client, mut server) = duplex(1024);
        let command = SocketCommand::Power;
        let expected_response = SocketResponse::Ok(SocketData {
            active: false,
            power: 0.0,
            device_id: Some("kitchen_socket".to_string()),
        });

        // Сервер: принимает команду и отвечает
        let server_response = expected_response.clone();
        let server_task = tokio::spawn(async move {
            let received_command = receive_command(&mut server).await.unwrap();
            assert_eq!(received_command, command);
            send_response(&mut server, &server_response).await.unwrap();
        });

        // Клиент: отправляет команду и получает ответ
        let received_response = send_command_and_receive(&mut client, &command)
            .await
            .unwrap();
        assert_eq!(received_response, expected_response);

        server_task.await.unwrap();
    }

    #[tokio::test]
    #[ignore = "integration test with async networking"]
    async fn test_error_response() {
        let (mut client, mut server) = duplex(1024);
        let error_response = SocketResponse::Error {
            message: "Device overheating".to_string(),
        };

        let response_clone = error_response.clone();
        let client_task = tokio::spawn(async move {
            send_response(&mut client, &response_clone).await.unwrap();
        });

        let received = receive_response(&mut server).await.unwrap();
        assert_eq!(received, error_response);

        client_task.await.unwrap();
    }

    #[tokio::test]
    #[ignore = "integration test with async networking"]
    async fn test_message_size_limit() {
        let (mut client, mut server) = duplex(1024);

        // Создаем очень большое сообщение
        let huge_message = "x".repeat(2 * 1024 * 1024); // 2MB

        let client_task = tokio::spawn(async move {
            let _ = send_message(&mut client, &huge_message).await;
        });

        // Должна быть ошибка из-за превышения лимита
        let result = receive_message(&mut server).await;
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.kind(), std::io::ErrorKind::InvalidData);
            assert!(e.to_string().contains("Message too large"));
        }

        client_task.await.unwrap();
    }

    #[test]
    fn test_serialization_formats() {
        let command = SocketCommand::TurnOn;
        let json = serde_json::to_string(&command).unwrap();
        assert!(json.contains("turn_on"));

        let response = SocketResponse::Ok(SocketData {
            active: true,
            power: 1000.0,
            device_id: None,
        });
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"result\":\"ok\""));
        assert!(json.contains("\"active\":true"));
        assert!(json.contains("\"power\":1000.0"));
    }
}

//! Простой UDP listener для проверки эмулятора

use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    println!("🎧 UDP Listener запущен на 127.0.0.1:8080");
    println!("Ожидание пакетов от эмулятора...\n");

    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    let mut buf = [0; 1024];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                let data = String::from_utf8_lossy(&buf[..size]);
                println!("📦 Получен пакет от {}: {}", src, data);
            }
            Err(e) => {
                eprintln!("❌ Ошибка получения: {}", e);
            }
        }
    }
}

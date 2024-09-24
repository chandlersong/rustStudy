use std::time::Duration;
use tokio::time;
use tokio_tungstenite::connect_async;
use tungstenite::Message;
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() {
    // WebSocket 服务器的 URL
    let url = "wss://echo.websocket.org";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
    // 连接到 WebSocket 服务器
    let (mut write, mut read) = ws_stream.split();

    // 创建一个任务来监听消息
    let read_task = tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    match msg {
                        Message::Text(text) => {
                            println!("Received text: {}", text);
                        }
                        Message::Binary(_) => {
                            println!("Received binary message");
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    eprintln!("Error while receiving message: {}", e);
                    break;
                }
            }
        }
    });

    // 创建一个任务来定期发送消息
    let write_task = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await; // 等待下一个间隔
            let msg = Message::Text("Hello, WebSocket!".to_string());
            if let Err(e) = write.send(msg).await {
                eprintln!("Error while sending message: {}", e);
                break;
            }
        }
    });

    // 等待两个任务完成
    let _ = tokio::try_join!(read_task, write_task);
}

use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个无界通道
    let (tx, _rx) = channel::unbounded();

    // 启动多个消费者
    for id in 0..3 {
        let rx = _rx.clone(); // 克隆发送者
        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(message) => {
                        println!("Consumer {} received: {}", id, message);
                    }
                    Err(_) => {
                        println!("Consumer {}: No more messages", id);
                        break; // 如果通道关闭，退出循环
                    }
                }
            }
        });
    }

    // 生产者发送消息
    for i in 1..=3 {
        let message = format!("Message {}", i);
        // 这里我们需要为每个消费者分别发送消息
        for _ in 0..3 {
            tx.send(message.clone()).unwrap(); // 发送给每个消费者
        }
        println!("Produced: Message {}", i);
        thread::sleep(Duration::from_secs(1)); // 模拟生产过程
    }
}

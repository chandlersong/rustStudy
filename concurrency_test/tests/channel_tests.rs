#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use crossbeam::channel;
    #[test]
    fn test_channel() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }


    #[test]
    fn test_channel_multi_receive() {
        // 创建一个多生产者多消费者的通道
        let (tx, rx) = channel::unbounded();

        // 创建多个接收者
        let num_receivers = 3;
        let mut handles = vec![];

        for id in 0..num_receivers {
            let rx = rx.clone(); // 克隆接收者
            let handle = thread::spawn(move || {
                while let Ok(message) = rx.recv() {
                    println!("接收者 {} 收到消息: {}", id, message);
                }
            });
            handles.push(handle);
        }

        // 发送一些消息
        for i in 0..5 {
            tx.send(i).unwrap();
        }

        // 关闭发送者
        drop(tx);

        // 等待所有接收者完成
        for handle in handles {
            handle.join().unwrap();
        }

        println!("所有接收者已完成");

    }
}

use tokio::sync::{mpsc, oneshot};
use tokio::time::{self, Duration};

/*
 * 这个例子，主要是在producer和consumer的模式下。
 * consumer为一个单独的协程。但是主协程不太清楚怎么来做。
 * 这个是每个协程单独起一个。后台运行
 */
struct Consumer {
    tx: mpsc::Sender<(i32, oneshot::Sender<i32>)>,
}

impl Consumer {
    // 创建一个新的 Consumer 实例
    fn new() -> (Self, tokio::task::JoinHandle<()>) {
        let (tx, mut rx): (mpsc::Sender<(i32, oneshot::Sender<i32>)>, mpsc::Receiver<(i32, oneshot::Sender<i32>)>) = mpsc::channel(32); // 明确指定通道的类型

        // 启动消费者任务
        let handle = tokio::spawn(async move {
            while let Some((value, response_tx)) = rx.recv().await {
                // 模拟处理时间
                time::sleep(Duration::from_secs(1)).await; // 假设处理需要 1 秒
                // 处理接收到的值
                let result = value * 2; // 示例处理：将值乘以 2
                // 发送处理结果
                let _ = response_tx.send(result);
            }
        });

        (Self { tx }, handle)
    }

    // 发送数据到消费者并等待响应
    fn send(&self, value: i32) -> i32 {
        let (response_tx, response_rx) = oneshot::channel();
        // 发送数据和响应通道
        let _ = self.tx.try_send((value, response_tx)).unwrap(); // 使用 try_send 发送数据

        // 在这里阻塞等待消费者的响应
        tokio::task::block_in_place(|| {
            // 这里需要使用 block_in_place 来等待异步操作
            tokio::runtime::Runtime::new().unwrap().block_on(response_rx).unwrap_or(0) // 如果接收失败，返回 0
        })
    }
}

#[tokio::main]
async fn main() {
    // 创建消费者
    let (consumer, _consumer_handle) = Consumer::new();

    // 主协程
    for i in 1..=5 {
        let result = consumer.send(i); // 这里不需要 await
        println!("Received from consumer: {}", result);
    }
}

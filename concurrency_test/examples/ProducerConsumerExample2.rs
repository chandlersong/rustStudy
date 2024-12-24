use tokio::sync::oneshot;
use tokio::time::{self, Duration};


/*
 * 这个例子，主要是在producer和consumer的模式下。
 * consumer为一个单独的协程。但是主协程不太清楚怎么来做。
 * 每次运行，都有启动一个单独的consumer
 */
struct Consumer;

impl Consumer {
    // 处理消息的函数
    async fn process(value: i32) -> i32 {
        // 模拟处理时间
        time::sleep(Duration::from_secs(1)).await; // 假设处理需要 1 秒
        // 处理接收到的值
        value * 2 // 示例处理：将值乘以 2
    }

    // 发送数据到消费者并等待响应
    fn send(&self, value: i32) -> i32 {
        // 创建一个 oneshot 通道用于接收响应
        let (response_tx, response_rx) = oneshot::channel();

        // 启动一个新的协程来处理消息
        tokio::spawn(async move {
            let result = Consumer::process(value).await; // 处理消息
            let _ = response_tx.send(result); // 发送处理结果
        });

        // 在这里阻塞等待消费者的响应
        // 使用 block_in_place 来等待异步操作
        tokio::task::block_in_place(|| {
            // 创建一个新的运行时来等待 response_rx
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(response_rx).unwrap_or(0) // 如果接收失败，返回 0
        })
    }
}

#[tokio::main]
async fn main() {
    let consumer = Consumer;

    // 主协程
    for i in 1..=5 {
        let result = consumer.send(i); // 这里不需要 await
        println!("Received from consumer: {}", result);
    }
}

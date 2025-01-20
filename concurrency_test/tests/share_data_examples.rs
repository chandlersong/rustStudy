#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio::task;
    // 定义一个自定义结构体
    struct Counter {
        name: String,
        value: usize,
    }

    #[tokio::test]
    async fn test_tokio_chain() {
        // 创建一个 Arc 和 Tokio 的 Mutex 来共享 Counter 对象
        let counter = Arc::new(Mutex::new(Counter {
            name: String::from("MyCounter"),
            value: 0,
        }));
        let mut tasks = vec![];

        // 启动 10 个异步任务
        for i in 0..10 {
            // 克隆 Arc，以便每个任务都有一个引用
            let counter_clone = Arc::clone(&counter);

            let task = task::spawn(async move {
                // 锁定 Tokio 的 Mutex，获取对 Counter 的可变引用
                let mut counter = counter_clone.lock().await; // 注意这里是 .await
                counter.value += 1; // 递增计数器
                println!("Task {} incremented {} to {}", i, counter.name, counter.value);
            });

            tasks.push(task);
        }

        // 等待所有任务完成
        for task in tasks {
            task.await.unwrap();
        }

        // 打印最终的计数器值
        let final_counter = counter.lock().await; // 注意这里也是 .await
        println!("Final counter value: {}", final_counter.value);
    }
}

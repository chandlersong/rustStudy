#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::sync::watch;
    use tokio::time::sleep;

    ///
    /// 主要学些了一下啊tokio的watch。
    /// 1. 会读取第一个新的。
    /// 2，如果没有更新，他会反复读取旧的数据
    #[tokio::test]
    async fn test_tokio_channel() {
        let (tx, mut rx) = watch::channel::<i32>(0);

        tokio::spawn(async move {
            tx.send(1).expect("TODO: panic message");
            tx.send(2).expect("TODO: panic message");
        });

        tokio::spawn(async move {
            loop {
                sleep(Duration::from_millis(300)).await;
                let result = *rx.borrow_and_update();
                println!("{}! ", result);
                // if rx.changed().await.is_err() {
                //     break;
                // }
                assert_eq!(result,2);
            }
        });

        sleep(Duration::from_millis(1000)).await;
    }
}

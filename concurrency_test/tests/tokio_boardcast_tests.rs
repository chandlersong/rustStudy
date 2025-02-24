#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::sync::{broadcast, Barrier};

    #[tokio::test]
    async fn test_tokio_broadcast() {
        let (tx, mut rx1) = broadcast::channel(16);
        let mut rx2 = tx.subscribe();
        let barrier = Arc::new(Barrier::new(3));
        let b1 = barrier.clone();
        tokio::spawn(async move {
            assert_eq!(rx1.recv().await.unwrap(), 10);
            assert_eq!(rx1.recv().await.unwrap(), 20);
            print!("thread 1");
            b1.wait().await;
        });

        let b2 = barrier.clone();
        tokio::spawn(async move {
            assert_eq!(rx2.recv().await.unwrap(), 10);
            assert_eq!(rx2.recv().await.unwrap(), 20);
            print!("thread 2");
            b2.wait().await;
        });

        tx.send(10).unwrap();
        tx.send(20).unwrap();
        barrier.wait().await;
    }
}
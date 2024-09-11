#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_tokio_channel() {
        let (tx, mut rx) = mpsc::channel(1);

        tokio::spawn(async move {
            let val = String::from("hi");
            tx.send(val).await.unwrap();
        });

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
    }
}

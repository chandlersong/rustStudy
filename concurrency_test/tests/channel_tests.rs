#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
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
}

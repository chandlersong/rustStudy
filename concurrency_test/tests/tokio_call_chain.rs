use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

async fn a() -> i32{
    // 异步方法A的实现
   18
}

fn b() -> Receiver<i32> {
    let (tx, rx) = mpsc::channel();
    // 同步方法B调用异步方法A
    // 在新线程中执行"异步"任务
    thread::spawn(move || {
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(a());
        tx.send(result).unwrap();
    });
    rx

}

async fn c() -> i32 {
    // 异步方法C调用同步方法B
    let rx = b();

    let res = match rx.recv() {
        Ok(result) => { result * 2 }
        Err(e) => {
            print!("Error: {} \n", e);
            panic!("error")
        }
    };
    res + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tokio_chain() {
        let final_result = c().await;
        println!("最终结果: {}", final_result);
    }
}

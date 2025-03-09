use macro_example::log;

#[log("Starting computation")]
fn compute(x: i32) -> i32 {
    println!("Computing...");
    x * 2
}

fn main() {
    let result = compute(5);
    println!("Result: {}", result);
}
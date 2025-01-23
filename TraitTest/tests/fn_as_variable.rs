fn call_fn_mut<F>(mut func: F, a: i32, b: i32)
where
    F: FnMut(i32, i32), // 定义 FnMut 特征，接受两个 i32 参数
{
    func(a, b); // 调用闭包并传入参数
}

fn execute_fn(a: i32, b: i32) {
    println!("a = {}, b = {}", a, b);
}


#[cfg(test)]
mod tests {
    use crate::{call_fn_mut, execute_fn};

    #[test]
    fn test_fn_as_variable_closure() {
        let mut sum = 0;
        let my_closure = |x: i32, y: i32| {
            sum += x + y; // 可变借用
            // 不返回任何值
        };
        call_fn_mut(execute_fn, 3, 7);
        println!("Total sum: {}", sum); // 打印 sum 的最终值
    }

    #[test]
    fn test_fn_as_variable_fn() {

        call_fn_mut(execute_fn, 3, 7);
    }
}

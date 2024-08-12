trait AsyncShape {
    async fn display_info(&self);
}

struct Container<T: AsyncShape> {
    items: T,
}

impl<T: AsyncShape> Container<T> {
    fn new(shape: T) -> Self {
        Container { items: shape }
    }

    async fn display_items(&self) {
        self.items.display_info().await
    }
}

struct Circle {
    radius: f64,
}

impl AsyncShape for Circle {
    async fn display_info(&self) {
        println!("Circle with radius {}", self.radius);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl AsyncShape for Rectangle {
    async fn display_info(&self) {
        println!("Rectangle with width {} and height {}", self.width, self.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_hello() {
        let circle = Circle { radius: 5.0 };
        let mut container1 = Container::new(circle);

        let rectangle = Rectangle { width: 3.0, height: 4.0 };
        let mut container2 = Container::new(rectangle);

        // 下面这个编译错误。container2的类型不匹配。
        // let array = vec![container1,container2]
    }
}

use std::fmt::Display;

trait Shape {
    fn display_info(&self);
}

struct Container {
    items: Vec<Box<dyn Shape>>,
}

impl Container {
    fn new() -> Self {
        Container { items: Vec::new() }
    }

    fn add_item(&mut self, item: Box<dyn Shape>) {
        self.items.push(item);
    }

    fn display_items(&self) {
        for item in &self.items {
            item.display_info();
        }
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn display_info(&self) {
        println!("Circle with radius {}", self.radius);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn display_info(&self) {
        println!("Rectangle with width {} and height {}", self.width, self.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hello() {
        let mut container = Container::new();

        let circle = Circle { radius: 5.0 };
        let rectangle = Rectangle { width: 3.0, height: 4.0 };

        container.add_item(Box::new(circle));
        container.add_item(Box::new(rectangle));

        container.display_items();

    }
}

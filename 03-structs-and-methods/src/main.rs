/// A simple rectangle struct representing a shape with width and height.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Associated function (like a static method) to create a new Rectangle.
    /// This is a common pattern in Rust for constructors.
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Instance method that calculates the area of the rectangle.
    /// Takes &self to borrow the instance immutably.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Instance method that checks if one rectangle can completely contain another.
    /// Takes &self and a reference to another Rectangle.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle::new(60, 45);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Rect1 debug info: {:?}", rect1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(10, 5);
        assert_eq!(rect.area(), 50);
    }

    #[test]
    fn test_rectangle_can_hold() {
        let large = Rectangle::new(100, 100);
        let small = Rectangle::new(50, 50);
        let tall = Rectangle::new(10, 150);

        assert!(large.can_hold(&small));
        assert!(!small.can_hold(&large));
        assert!(!large.can_hold(&tall));
    }

    #[test]
    fn test_rectangle_new() {
        let rect = Rectangle::new(20, 30);
        assert_eq!(rect.width, 20);
        assert_eq!(rect.height, 30);
    }
}

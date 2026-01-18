/// An enum representing different types of messages.
/// Enums in Rust can store diverse data in their variants.
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Anonymous struct
    Write(String),              // Single String
    ChangeColor(i32, i32, i32), // Tuple of three integers
}

impl Message {
    /// Method on the Message enum that uses pattern matching to handle variants.
    /// Pattern matching ensures all cases are handled.
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit variant triggered."),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to R:{}, G:{}, B:{}", r, g, b),
        }
    }
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 30 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(0, 255, 255),
    ];

    for msg in messages {
        msg.call();
    }

    // Option enum example
    let some_number = Some(5);
    let _absent_number: Option<i32> = None;

    if let Some(n) = some_number {
        println!("Found a number: {}", n);
    } else {
        println!("No number found.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_variants_exist() {
        // Technically this just ensures the variants can be constructed
        let _q = Message::Quit;
        let _m = Message::Move { x: 1, y: 2 };
        let _w = Message::Write(String::from("test"));
        let _c = Message::ChangeColor(1, 2, 3);
    }

    #[test]
    fn test_option_handling() {
        let some_val = Some(10);
        let none_val: Option<i32> = None;

        assert_eq!(some_val.unwrap(), 10);
        assert!(none_val.is_none());
    }
}

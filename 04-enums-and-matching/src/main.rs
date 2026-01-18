enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit variant"),
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

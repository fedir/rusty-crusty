use std::fs::{self, File};
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    // Shorter way using fs::read_to_string
    // fs::read_to_string("hello.txt")

    // Explicit way for demo
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    // Setup a dummy file
    let path = "hello.txt";
    let content = "Alice";

    println!("Writing '{}' to {}", content, path);
    fs::write(path, content).expect("Unable to write file");

    match read_username_from_file() {
        Ok(s) => println!("Read username: {}", s),
        Err(e) => println!("Error reading file: {:?}", e),
    }

    // Cleanup
    println!("Cleaning up {}", path);
    fs::remove_file(path).unwrap_or_else(|e| println!("Failed to delete: {}", e));
}

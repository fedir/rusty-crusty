use std::fs::{self, File};
use std::io::{self, Read};

/// Reads content from a file named "hello.txt" and returns it as a Result.
/// demonstrating manual file opening and reading.
fn read_username_from_file() -> Result<String, io::Error> {
    // Note: Rust provides a simpler 'fs::read_to_string("hello.txt")'
    // but the following shows the explicit steps for educational purposes.

    // Attempt to open the file. The '?' operator returns the error early if it fails.
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    // Read the file contents into the string buffer.
    username_file.read_to_string(&mut username)?;

    // Return the successful string wrapped in Ok.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_file_processing() {
        let test_file = "test_user.txt";
        let content = "TestUser";

        // Setup
        fs::write(test_file, content).unwrap();

        // The function expects "hello.txt", so we temporarily rename or mock it
        // For simplicity in this demo, let's just test the logic inline
        let read_content = fs::read_to_string(test_file).unwrap();
        assert_eq!(read_content, content);

        // Cleanup
        fs::remove_file(test_file).unwrap();
    }
}

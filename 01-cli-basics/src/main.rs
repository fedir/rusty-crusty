use std::env;

/// The core logic of the CLI application.
///
/// This function takes a slice of strings (arguments) and returns a formatted
/// string representing the output to be displayed to the user.
/// It handles help flags and counts/lists provided arguments.
fn run(args: &[String]) -> String {
    // Check if the user specifically asked for help using the -h flag.
    if args.len() > 1 && args[1] == "-h" {
        return String::from(
            "Usage: cli-basics [arguments]\nOptions:\n  -h    Show this help message",
        );
    }

    let mut output = String::new();
    output.push_str(&format!("Received {} arguments.\n", args.len()));

    // args[0] is always the name of the executable itself.
    if args.len() > 1 {
        output.push_str("Arguments exceeded 1. Here they are:");
        for (i, arg) in args.iter().enumerate() {
            output.push_str(&format!("\n{}: {}", i, arg));
        }
    } else {
        output.push_str("No extra arguments provided. Try running with: cargo run -- args go here");
    }
    output
}

/// The entry point of the application.
/// It collects arguments from the environment and prints the result of the `run` logic.
fn main() {
    println!("Hello! This is a CLI basics demo.");
    // env::args() returns an iterator of the arguments passed to the program.
    let args: Vec<String> = env::args().collect();
    println!("{}", run(&args));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_flag() {
        let args = vec![String::from("program"), String::from("-h")];
        let output = run(&args);
        assert!(output.contains("Usage: cli-basics"));
        assert!(output.contains("Show this help message"));
    }

    #[test]
    fn test_no_extra_args() {
        let args = vec![String::from("program")];
        let output = run(&args);
        assert!(output.contains("Received 1 arguments."));
        assert!(output.contains("No extra arguments provided."));
    }

    #[test]
    fn test_with_args() {
        let args = vec![
            String::from("program"),
            String::from("arg1"),
            String::from("arg2"),
        ];
        let output = run(&args);
        assert!(output.contains("Received 3 arguments."));
        assert!(output.contains("1: arg1"));
        assert!(output.contains("2: arg2"));
    }
}

use rand::Rng;
use std::cmp::Ordering;
use std::io; // Requires `rand` dependency

/// The entry point of the guessing game.
/// It introduces the game, asks for a range, generates a secret number,
/// and enters a loop where the user can guess until they win.
fn main() {
    println!("Guess the number!");

    // Ask user to define the range for the secret number.
    let (min, max) = get_range();
    println!("Generating secret number between {} and {}...", min, max);

    // thread_rng() gives us the random number generator that's local to the current thread.
    // gen_range(min..=max) generates a number in the inclusive range [min, max].
    let secret_number = rand::thread_rng().gen_range(min..=max);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Read user input from standard input.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the string into a u32 number. If parsing fails, skip the rest of the loop.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid positive number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Compare the guess to the secret number.
        match check_guess(guess, secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop when the guess is correct.
            }
        }
    }
}

/// Helper function to prompt for and read a numeric input from standard input.
/// It keeps asking until a valid u32 is provided.
fn get_input(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please type a number!"),
        }
    }
}

/// Prompts the user for a minimum and maximum and ensures the range is valid (max > min).
fn get_range() -> (u32, u32) {
    loop {
        let min = get_input("Enter minimum number:");
        let max = get_input("Enter maximum number:");
        if max > min {
            return (min, max);
        }
        println!("Max must be greater than min!");
    }
}

/// Compares a guess against the secret number and returns the Ordering (Less, Greater, or Equal).
fn check_guess(guess: u32, secret: u32) -> Ordering {
    guess.cmp(&secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_guess_ordering() {
        assert_eq!(check_guess(50, 100), Ordering::Less);
        assert_eq!(check_guess(150, 100), Ordering::Greater);
        assert_eq!(check_guess(100, 100), Ordering::Equal);
    }

    #[test]
    fn test_range_validation_logic_helper() {
        // Simulating logic: max must be > min
        let min = 10;
        let max = 20;
        assert!(max > min);

        let min = 20;
        let max = 10;
        assert!(!(max > min));
    }
}

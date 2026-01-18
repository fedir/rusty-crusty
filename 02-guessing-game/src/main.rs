use rand::Rng;
use std::cmp::Ordering;
use std::io; // Requires `rand` dependency

fn main() {
    println!("Guess the number!");

    let (min, max) = get_range();
    println!("Generating secret number between {} and {}...", min, max);

    let secret_number = rand::thread_rng().gen_range(min..=max);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match check_guess(guess, secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

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

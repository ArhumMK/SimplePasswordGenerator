// TODO: replace repitive code with something like match statements

use rand::{thread_rng, Rng};
use std::io;

// Constants to be used in the password
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_-+=<>?";

pub struct PasswordConfig {
    pub length: usize,
    pub use_uppercase: bool,
    pub use_numbers: bool,
    pub use_symbols: bool,
}

fn generate_password(config: &PasswordConfig) -> String {
    // Start with lowercase characters
    let mut chars = LOWERCASE.to_string();

    // Then add uppercase, numbers, and symbols based on user preferences
    if config.use_uppercase {
        chars.push_str(UPPERCASE);
    }
    if config.use_numbers {
        chars.push_str(NUMBERS);
    }
    if config.use_symbols {
        chars.push_str(SYMBOLS);
    }

    let mut rng = thread_rng(); // Create a random number generator
    let mut password = String::new(); // Create an empty String to hold the password

    // Iterate to generate the specified length of password
    for _ in 0..config.length {
        // Generate a random index to pick a character
        let index = rng.gen_range(0..chars.len());
        // Get the character at the random index and append it to the password
        password.push(chars.chars().nth(index).unwrap());
    }

    return password;
}

fn get_password_preferences() -> (usize, bool, bool, bool) {
    // Start asking the user's preferences
    println!("Enter password length:");
    let mut length = String::new();
    io::stdin().read_line(&mut length).unwrap();
    let length: usize = length.trim().parse().expect("Please enter a valid number");

    println!("Include uppercase letters? (y/n):");
    let mut uppercase = String::new();
    io::stdin().read_line(&mut uppercase).unwrap();
    let use_uppercase = uppercase.trim().eq_ignore_ascii_case("y");

    println!("Include numbers? (y/n):");
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).unwrap();
    let use_numbers = numbers.trim().eq_ignore_ascii_case("y");

    println!("Include symbols? (y/n):");
    let mut symbols = String::new();
    io::stdin().read_line(&mut symbols).unwrap();
    let use_symbols = symbols.trim().eq_ignore_ascii_case("y");

    // Return this as a tuple which will then be passed into generate_password()
    (length, use_uppercase, use_numbers, use_symbols)
}

fn main() {
    // Get user preferences for password generation
    //let (length, use_uppercase, use_numbers, use_symbols) = get_password_preferences();

    // Generate the password based on user preferences
    //let password = generate_password(PasswordConfig);

    //println!("Generated password: {}", password);
}

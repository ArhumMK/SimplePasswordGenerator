//use std::io;
use rand::{thread_rng, Rng};

// Define constants for different character sets
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_-+=<>?";

// Function to generate a 16-character password
// TODO: add option to customize password length and properties
fn generate_password() -> String {
    // Combine all character sets into one string
    let chars = LOWERCASE.to_string() + UPPERCASE + NUMBERS + SYMBOLS;
    let mut rng = thread_rng();
    let mut password = String::new(); // Create an empty String to hold the password

    // Iterate 16 times to generate a 16-digit password
    for _ in 0..16 {
        // Generate a random index to pick a character
        let index = rng.gen_range(0..chars.len());
        // Get the character at the random index and append it to the password
        password.push(chars.chars().nth(index).unwrap());
    }

    return password;
}

fn main() {
    let password = generate_password();
    println!("Generated password: {}", password);
}

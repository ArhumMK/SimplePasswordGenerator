use std::io;
use rand::{thread_rng, Rng};

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_-+=<>?";

fn generate_password() -> String {
    let mut chars = LOWERCASE.to_string() + UPPERCASE + NUMBERS + SYMBOLS;
    let mut rng = thread_rng();

    (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars.chars().nth(idx).unwrap()
        })
        .collect()
}

fn main() {
    let password = generate_password();
    println!("Generated password: {}", password);
}

use rand::{thread_rng, Rng};

// Constants to be used in the password
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_-+=<>?";

// Struct for password string features,
pub struct PasswordConfig {
    pub length: usize,
    pub use_uppercase: bool,
    pub use_numbers: bool,
    pub use_symbols: bool,
}

pub fn generate_password(config: &PasswordConfig) -> String {
    let mut chars = LOWERCASE.to_string();

    if config.use_uppercase {
        chars.push_str(UPPERCASE);
    }
    if config.use_numbers {
        chars.push_str(NUMBERS);
    }
    if config.use_symbols {
        chars.push_str(SYMBOLS);
    }

    let mut rng = thread_rng();
    (0..config.length)
        .map(|_| {
            let index = rng.gen_range(0..chars.len());
            chars.chars().nth(index).unwrap()
        })
        .collect()
}

fn main() {
    println!("PasswordGenerator 0.2.0");
}

// TODO: add more tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        let config = PasswordConfig {
            length: 12,
            use_uppercase: true,
            use_numbers: true,
            use_symbols: true,
        };
        let password = generate_password(&config);
        assert_eq!(password.len(), 12);
    }
}

// Decouple CLI functionality into seperate module here
// TODO: add stricter user input, e.g accept y/n only instead of any character
#[cfg(feature = "cli")]
mod cli {
    use super::*;
    use std::io;

    pub fn get_password_preferences() -> PasswordConfig {
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

        PasswordConfig {
            length,
            use_uppercase,
            use_numbers,
            use_symbols,
        }
    }
}

#[cfg(feature = "cli")]
fn main() {
    let config = cli::get_password_preferences();
    let password = generate_password(&config);
    println!("Generated password: {}", password);
}

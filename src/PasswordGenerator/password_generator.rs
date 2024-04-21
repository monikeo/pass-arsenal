use rand::seq::SliceRandom;
use rand::Rng;
use std::iter;

//  Define the character sets
const NUMBERS: &str = "0123456789";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const SPECIAL_CHARS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?''\"\"";

#[derive(Debug)]
pub struct PasswordOption {
    length: usize,
    include_numbers: bool,
    include_uppercase: bool,
    include_lowercase: bool,
    include_special: bool,
    exclude_duplicates: bool,
    exclude_similar: bool,
}

impl PasswordOption {
    pub fn new() -> Self {
        Self {
            length: 12, // Default length
            include_numbers: true,
            include_uppercase: true,
            include_lowercase: true,
            include_special: true,
            exclude_duplicates: false,
            exclude_similar: false,
        }
    }

    pub fn generate_password(&self) -> String {
        let mut password = String::new();
        let mut rng = rand::thread_rng();
        let mut character_pool = String::new();

        if self.include_numbers() {
            character_pool.push_str(NUMBERS);
        }
        if self.include_uppercase() {
            character_pool.push_str(UPPERCASE);
        }
        if self.include_lowercase() {
            character_pool.push_str(LOWERCASE);
        }
        if self.include_special() {
            character_pool.push_str(SPECIAL_CHARS);
        }
        // Exclude similar characters if the option is set
        let similar_chars = "ilIoO0";
        if self.exclude_similar() {
            character_pool = character_pool
                .chars()
                .filter(|ch| !similar_chars.contains(*ch))
                .collect();
        }

        // Convert character pool to Vec<char> once for efficiency
        let char_vec: Vec<char> = character_pool.chars().collect();

        if self.exclude_duplicates() {
            let mut unique_chars = Vec::new();
            while unique_chars.len() < self.length() {
                let ch = *char_vec.choose(&mut rng).expect("chracter pool is empty");
                if !unique_chars.contains(&ch) {
                    unique_chars.push(ch);
                }
            }
            password = unique_chars.into_iter().collect();
        } else {
            password = iter::repeat(())
                .map(|()| *char_vec.choose(&mut rng).expect("character pool is empty"))
                .take(self.length)
                .collect()
        }

        password
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn include_numbers(&self) -> bool {
        self.include_numbers
    }

    pub fn include_uppercase(&self) -> bool {
        self.include_uppercase
    }

    pub fn include_lowercase(&self) -> bool {
        self.include_lowercase
    }

    pub fn include_special(&self) -> bool {
        self.include_special
    }

    pub fn exclude_duplicates(&self) -> bool {
        self.exclude_duplicates
    }

    pub fn exclude_similar(&self) -> bool {
        self.exclude_similar
    }

    pub fn set_length(&mut self, length: usize) {
        self.length = length;
    }

    pub fn set_include_numbers(&mut self, flag: bool) {
        self.include_numbers = flag;
    }

    pub fn set_include_uppercase(&mut self, flag: bool) {
        self.include_uppercase = flag;
    }

    pub fn set_include_lowercase(&mut self, flag: bool) {
        self.include_lowercase = flag;
    }

    pub fn set_include_special(&mut self, flag: bool) {
        self.include_special = flag;
    }

    pub fn set_exclude_duplicates(&mut self, flag: bool) {
        self.exclude_duplicates = flag;
    }

    pub fn set_exclude_similar(&mut self, flag: bool) {
        self.exclude_similar = flag;
    }
}

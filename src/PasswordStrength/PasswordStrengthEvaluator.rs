use crate::PasswordStrength::PasswordStrengthCriteria::password_strength_criteria::*;
use std::collections::HashSet;

pub fn evaluate_password_strength(password: &str) -> u32 {
    let mut score = 0;

    println!();
    println!("Analyzing password strength ... ");
    if is_common_password(password) {
        score = 10;
    }
    score
}

pub fn evaluate_password_length(password: &str) -> u32 {
    match password.len() {
        0..=6 => 10,
        7..=10 => 20,
        11..=14 => 30,
        _ => 40,
    }
}

pub fn evaluate_password_characters_diversity(password: &str) -> u32 {
    let mut unique_chars: HashSet<char> = HashSet::new();
    for ch in password.chars() {
        unique_chars.insert(ch);
    }
    match unique_chars.len() {
        0..=3 => 10,
        4..=6 => 20,
        7..=9 => 30,
        _ => 40,
    }
}

pub fn evaluate_password_common_pattern(password: &str) -> u32 {
    // full score 60
    let mut score: u32 = 0;

    let checks = [
        (contains_lowercase_chars(password), "Lowercase Letters"),
        (contains_uppercase_chars(password), "Uppercase Letters"),
        (contains_number(password), "Numbers"),
        (contains_special_chars(password), "Special Characters"),
        (
            !contains_sequential_chars(password),
            "No Sequential Characters",
        ),
        (!contains_repeated_chars(password), "No Repeated Characters"),
    ];

    for (condition, label) in checks {
        println!(" {} {}", if condition { " ✅ " } else { " V " }, label);
        if condition {
            score += 10;
        }
    }
    score
}

/*
*   To find out how strong your password is, you can use a formula: E = log2 (RL).
*   It shows how many guesses a hacker needs to crack your password by trying all possible combinations of symbols.
*
*   Here's what each part means:
*
*   E is the password strength in bits.
*   Log2 is a math trick that changes the number of possible combinations into bits.
*   R — Size of the pool of unique characters from which we build the password
*   L — Password length, i.e., the number of characters in the password.
*/
pub fn evaluate_password_entropy(password: &str) -> u32 {
    let mut entropy: f64 = 0.0;
    let number_pool = if contains_number(password) { 10 } else { 0 };
    let uppercase_pool = if contains_uppercase_chars(password) {
        26
    } else {
        0
    };
    let lowercase_pool = if contains_lowercase_chars(password) {
        26
    } else {
        0
    };
    let special_chars_pool = if contains_special_chars(password) {
        32
    } else {
        0
    };
    let pool_size = number_pool + uppercase_pool + lowercase_pool + special_chars_pool;
    let password_length = password.len();

    entropy = f64::log2(pool_size as f64 * password_length as f64);
    entropy as u32
}

pub fn evaluate_password_shannon_entropy(password: &str) -> u32 {
    let mut entropy: f64 = 0.0;
    let mut char_counts: HashSet<(char, u32)> = HashSet::new();

    for ch in password.chars() {
        let count = char_counts
            .iter()
            .find(|(x, _)| x == &ch)
            .map(|(_, y)| *y)
            .unwrap_or(0);
        char_counts.insert((ch, count + 1));
        let p = (count + 1) as f64 / password.len() as f64;
        entropy -= p * p.log2();
    }
    entropy as u32
}

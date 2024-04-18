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

fn evaluate_password_length(password: &str) -> u32 {
    match password.len() {
        0..=6 => 10,
        7..=10 => 20,
        11..=14 => 30,
        _ => 40,
    }
}

fn evaluate_password_characters_diversity(password: &str) -> u32 {
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

fn evaluate_password_common_pattern(password: &str) -> u32 {
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
fn evaluate_password_common_pattern(password: &str) -> u32 {
    // full score 60
    let mut score: u32 = 0;
    println!();
    if contains_lowercase_chars(password) {
        println!(" [+] lowercase");
        score += 10;
    } else {
        println!(" [-] lowercase");
    }

    if contains_uppercase_chars(password) {
        println!(" [+] uppercase");
        score += 10;
    } else {
        println!(" [-] uppercase");
    }

    if contains_number(password) {
        println!(" [+] number");
        score += 10;
    } else {
        println!(" [-] number");
    }

    if contains_special_chars(password) {
        println!(" [+] special characters");
        score += 10;
    } else {
        println!(" [-] special characters");
    }

    if !contains_sequential_chars(password) {
        println!(" [+] no sequential characters");
        score += 10;
    } else {
        println!(" [-] contains sequential chracters");
    }

    if !contains_repeated_chars(password) {
        println!(" [+] no repeated characters");
        score += 10;
    } else {
        println!(" [-] contains repeated characters");
    }
    score
}
*/

fn evaluate_password_entropy(password: &str) -> u32 {
    let mut entropy: f64 = 0.0;
    let mut char_counts: HashSet<(char, u32)> = HashSet::new();

    for ch in password.chars() {
        let count = char_counts
            .iter()
            .find(|(x, _)| x == &ch)
            .map(|(_, y)| *y)
            .unwrap_or(0);
        char_counts.insert((ch, count + 1));
    }
    todo!();
}

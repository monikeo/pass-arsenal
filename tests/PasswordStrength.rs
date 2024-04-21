use pass_arsenal::PasswordStrength::{PasswordStrengthCriteria::*, PasswordStrengthEvaluator::*};

#[test]
fn test_pwned_password_exposure() {
    let password01 = "ILoveYou";
    let password02 = "cat222";
    let password03 = "Rust@#$2024!Rust";

    let result01 = password_strength_criteria::pwned_password_exposure(password01);
    let result02 = password_strength_criteria::pwned_password_exposure(password02);
    let result03 = password_strength_criteria::pwned_password_exposure(password03);
    assert!(result01);
    assert!(result02);
    assert!(!result03);
}

#[test]
fn test_contains_uppercase_lowercase_number_spacial_chars() {
    let password01 = "ILoveYou";
    let password02 = "cat222";
    let password03 = "Rust@#$2024!Rust";
    let result01 = password_strength_criteria::contains_uppercase_chars(password01);
    let result02 = password_strength_criteria::contains_uppercase_chars(password02);
    let result03 = password_strength_criteria::contains_uppercase_chars(password03);
    assert!(result01);
    assert!(!result02);
    assert!(result03);

    let result01 = password_strength_criteria::contains_lowercase_chars(password01);
    let result02 = password_strength_criteria::contains_lowercase_chars(password02);
    let result03 = password_strength_criteria::contains_lowercase_chars(password03);
    assert!(result01);
    assert!(result02);
    assert!(result03);
    assert!(!password_strength_criteria::contains_lowercase_chars(
        "ABCS34!#"
    ));

    let result01 = password_strength_criteria::contains_number(password01);
    let result02 = password_strength_criteria::contains_number(password02);
    let result03 = password_strength_criteria::contains_number(password03);
    assert!(!result01);
    assert!(result02);
    assert!(result03);

    let result01 = password_strength_criteria::contains_special_chars(password01);
    let result02 = password_strength_criteria::contains_special_chars(password02);
    let result03 = password_strength_criteria::contains_special_chars(password03);
    assert!(!result01);
    assert!(!result02);
    assert!(result03);
}

#[test]
fn test_contains_sequential_chars() {
    let password01 = "ILoveYou";
    let password02 = "ILoveYou123";
    let password03 = "ILoveYouABC#$2024";

    let result01 = password_strength_criteria::contains_sequential_chars(password01);
    let result02 = password_strength_criteria::contains_sequential_chars(password02);
    let result03 = password_strength_criteria::contains_sequential_chars(password03);

    assert!(!result01);
    assert!(result02);
    assert!(result03);
}

#[test]
fn test_contains_repeated_chars() {
    let password01 = "ILoveYou";
    let password02 = "cat222";
    let password03 = "Rust@1234!";
    let result01 = password_strength_criteria::contains_sequential_chars(password01);
    let result02 = password_strength_criteria::contains_sequential_chars(password02);
    let result03 = password_strength_criteria::contains_sequential_chars(password03);
    assert!(!result01);
    assert!(!result02);
    assert!(result03);
}

#[test]
fn test_is_common_password() {
    let password01 = "ILoveYou";
    let password02 = "Rust@1234!2024Love#";
    let password03 = "vjht008";
    let result01 = password_strength_criteria::is_common_password(password01);
    let result02 = password_strength_criteria::is_common_password(password02);
    let result03 = password_strength_criteria::is_common_password(password03);
    assert!(result01);
    assert!(!result02);
    assert!(result03);
}

#[test]
fn test_evaluate_password() {
    let password01 = "ILoveYou";
    let password02 = "iloveyou";
    let password03 = "vjht008";
    let password04 = "Rust@1234!2024Love#";

    let result01 = evaluate_password_length(password01);
    let result02 = evaluate_password_length(password02);
    let result03 = evaluate_password_length(password03);
    let result04 = evaluate_password_length(password04);
    assert_eq!(result01, 20);
    assert_eq!(result02, 20);
    assert_eq!(result03, 20);
    assert_eq!(result04, 40);

    let result01 = evaluate_password_characters_diversity(password01);
    let result02 = evaluate_password_characters_diversity(password02);
    let result03 = evaluate_password_characters_diversity(password03);
    let result04 = evaluate_password_characters_diversity(password04);
    assert_eq!(result01, 30);
    assert_eq!(result02, 30);
    assert_eq!(result03, 20);
    assert_eq!(result04, 40);

    let password04 = "Rust@1234!Love#";
    let result01 = evaluate_password_common_pattern(password01);
    let result02 = evaluate_password_common_pattern(password02);
    let result03 = evaluate_password_common_pattern(password03);
    let result04 = evaluate_password_common_pattern(password04);
    assert_eq!(result01, 30);
    assert_eq!(result02, 20);
    assert_eq!(result03, 30);
    assert_eq!(result04, 50);
}

#[test]
fn test_evaluate_password_entropy() {
    let password01 = "ILoveYou";
    let password02 = "iloveyou";
    let password03 = "ILoveYou@2024";
    let password04 = "Rust@1234!Love#";
    let result01 = evaluate_password_entropy(password01);
    let result02 = evaluate_password_entropy(password02);
    let result03 = evaluate_password_entropy(password03);
    let result04 = evaluate_password_entropy(password04);
    assert_eq!(result01, 45);
    assert_eq!(result02, 37);
    assert_eq!(result03, 85);
    assert_eq!(result04, 98);
}

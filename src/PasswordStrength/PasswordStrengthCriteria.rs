pub mod password_strength_criteria {
    use crate::Hash::Sha1;
    use reqwest;
    use std::collections::HashSet;

    pub fn is_common_password(password: &str) -> bool {
        todo!();
    }

    pub fn pwned_password_exposure(password: &str) -> bool {
        let password_sha1_hash = Sha1::sha1(password);
        let pwned_api_url = "https://api.pwnedpasswords.com/range/";
        let pwned_api_request_url = format!("{}{}", pwned_api_url, &password_sha1_hash[..5]);
        let api_response = reqwest::blocking::get(pwned_api_request_url);
        match api_response {
            Ok(body) => match body.text() {
                Ok(contents) => {
                    let mut hashes: Vec<(String, String)> = Vec::new();
                    for content in contents.split('\n') {
                        let part: Vec<String> =
                            content.split(':').map(|s| s.trim().to_string()).collect();
                        hashes.push((part[0].clone(), part[1].clone()));
                    }
                    println!("{}", contents);
                    //println!("{:?}", hashes);
                    let found = hashes
                        .iter()
                        .any(|(hash, _)| hash[..].to_lowercase() == password_sha1_hash[5..]);
                    return found;
                }
                Err(_) => {
                    println!("Failed to extract content from response");
                }
            },
            Err(_) => {
                println!("Checking internet connections...");
                println!("Failed to connect to pwned api");
            }
        }
        false
    }

    pub fn contains_uppercase_chars(password: &str) -> bool {
        password.chars().any(|ch| ch.is_uppercase())
    }

    pub fn contains_lowercase_chars(password: &str) -> bool {
        password.chars().any(|ch| ch.is_lowercase())
    }

    pub fn contains_number(password: &str) -> bool {
        password.chars().any(|ch| ch.is_ascii_digit())
    }

    pub fn contains_special_chars(password: &str) -> bool {
        password.contains(|ch| is_special_character(ch))
    }

    pub fn contains_sequential_chars(password: &str) -> bool {
        let mut previous_character: Option<char> = None;
        for ch in password.chars() {
            if let Some(prev_char) = previous_character {
                if ch as u32 == prev_char as u32 + 1 {
                    return true;
                }
            }
            previous_character = Some(ch);
        }
        false
    }

    pub fn contains_repeated_chars(password: &str) -> bool {
        let mut unique_characters: HashSet<char> = HashSet::new();
        for ch in password.chars() {
            if unique_characters.contains(&ch) {
                return true;
            }
            unique_characters.insert(ch);
        }
        false
    }

    // Define a function to check if a character is special
    fn is_special_character(c: char) -> bool {
        match c {
            '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '_' | '+' | '-' | '='
            | '{' | '}' | '[' | ']' | '\\' | '|' | ';' | ':' | '\'' | '?' | ',' | '.' | '/'
            | '<' | '>' => true,
            _ => false,
        }
    }
}

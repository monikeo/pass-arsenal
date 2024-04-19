pub mod password_strength_criteria {
    use crate::Hash::Sha1;
    use rayon::prelude::*;
    use reqwest;
    use std::collections::HashSet;

    fn get_document_html(url: &str) -> Option<String> {
        let response = reqwest::blocking::get(url);
        match response {
            Ok(body) => match body.text() {
                Ok(content) => return Some(content),
                Err(err) => {
                    println!(" [-] Error: Failed to extract content from HTML response");
                    println!(" [-] Error Status: {}", err);
                }
            },
            Err(err) => {
                println!(" [-] Error: Something wrong with URL request");
                println!(" [-] Error Status: {}", err);
            }
        }
        None
    }

    pub fn is_password_in_dictionary(password: &str) -> bool {
        let english_dictionary_370k_url = "https://raw.githubusercontent.com/monikeo281000/data_set_collection/main/english_dictionary/english_words_dictionary_370k.txt";
        let content = get_document_html(english_dictionary_370k_url);
        match content {
            Some(data) => {
                let dictionary_words: Vec<&str> = data.split('\n').collect();
                let status_dictionary_words_check =
                    dictionary_words.par_iter().any(|word| &password == word);
                return status_dictionary_words_check;
            }
            None => {}
        }
        false
    }

    pub fn is_common_password(password: &str) -> bool {
        let common_password_url = "https://raw.githubusercontent.com/monikeo281000/data_set_collection/main/password/passwordlist-1million.txt";
        let response = reqwest::blocking::get(common_password_url);
        match response {
            Ok(body) => match body.text() {
                Ok(content) => {
                    let common_passwords: Vec<&str> = content.split('\n').collect();
                    let status_password_check = common_passwords
                        .par_iter()
                        .any(|common_password| &password == common_password);
                    return status_password_check;
                    //println!("{:?}", &common_passwords[common_passwords.len() - 5..]);
                }
                Err(err) => {
                    println!("[-] Error: Failed to extract content from HTML response");
                    println!("[-] Error Status: {}", err);
                }
            },
            Err(err) => {
                println!("[-] Error: Something with url request");
                println!("[-] Error Status: {}", err);
            }
        }
        false
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
                    //println!("{}", contents);
                    //println!("{:?}", hashes);
                    let found = hashes
                        .iter()
                        .any(|(hash, _)| hash[..].to_lowercase() == password_sha1_hash[5..]);
                    return found;
                }
                Err(err) => {
                    println!("[-] Error: Failed to extract content from response");
                    println!("[-] Error Status: {}", err);
                }
            },
            Err(err) => {
                println!("[-] Error: Failed to connect to pwned api");
                println!("[-] Error Status: {}", err);
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
            | '<' | '>' | '`' | '~' | '"' => true,
            _ => false,
        }
    }
}

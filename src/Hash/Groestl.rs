use groestl::{Digest, Groestl224, Groestl256, Groestl384, Groestl512};

pub fn compare_groestl(cipher: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    cipher == hash_fn(plain_text)
}

pub fn groestl224(plain_text: &str) -> String {
    format!("{:x}", Groestl224::digest(plain_text.as_bytes()))
}

pub fn groestl256(plain_text: &str) -> String {
    format!("{:x}", Groestl256::digest(plain_text.as_bytes()))
}

pub fn groestl384(plain_text: &str) -> String {
    format!("{:x}", Groestl384::digest(plain_text.as_bytes()))
}

pub fn groestl512(plain_text: &str) -> String {
    format!("{:x}", Groestl512::digest(plain_text.as_bytes()))
}

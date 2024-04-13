use sha1::{Digest, Sha1};

pub fn compare_sha1(cipher: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    cipher == hash_fn(plain_text)
}

pub fn sha1(plain_text: &str) -> String {
    format!("{:x}", Sha1::digest(plain_text))
}

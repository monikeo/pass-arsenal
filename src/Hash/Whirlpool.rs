use whirlpool::{Digest, Whirlpool};

pub fn compare_whirlpool(cipher: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    cipher == hash_fn(plain_text)
}

pub fn whirlpool(plain_text: &str) -> String {
    format!("{:x}", Whirlpool::digest(plain_text.as_bytes()))
}

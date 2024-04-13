use blake2::{Blake2b512, Blake2s256, Digest};

pub fn compare_blake2(cipher: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    cipher == hash_fn(plain_text)
}

pub fn blake2s256(plain_text: &str) -> String {
    format!("{:x}", Blake2s256::digest(plain_text.as_bytes()))
}

pub fn blake2b512(plain_text: &str) -> String {
    format!("{:x}", Blake2b512::digest(plain_text.as_bytes()))
}

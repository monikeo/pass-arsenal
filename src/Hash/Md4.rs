use md4::{Digest, Md4};

pub fn compare_md4(hash_md4: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    hash_md4 == hash_fn(plain_text)
}

pub fn md4(plain_text: &str) -> String {
    format!("{:x}", Md4::digest(plain_text.as_bytes()))
}

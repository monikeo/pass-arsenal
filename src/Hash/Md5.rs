use md5::{Digest, Md5};

pub fn compare_md5(hash_md5: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    hash_md5 == hash_fn(plain_text)
}

pub fn md5(plain_text: &str) -> String {
    format!("{:x}", Md5::digest(plain_text.as_bytes()))
}

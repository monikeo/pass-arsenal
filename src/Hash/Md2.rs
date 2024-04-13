use md2::{Digest, Md2};

pub fn compare_md2(md2_hash: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    md2_hash == hash_fn(plain_text)
}

pub fn md2(plain_text: &str) -> String {
    format!("{:x}", Md2::digest(plain_text.as_bytes()))
}

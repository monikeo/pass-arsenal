use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

pub fn compare_sha2(
    original_password: &str,
    compare_password: &str,
    hash_fn: fn(&str) -> String,
) -> bool {
    original_password == hash_fn(compare_password)
}

pub fn sha224(plain_text: &str) -> String {
    format!("{:x}", Sha224::digest(plain_text.as_bytes()))
}

pub fn sha256(plain_text: &str) -> String {
    format!("{:x}", Sha256::digest(plain_text.as_bytes()))
}

pub fn sha384(plain_text: &str) -> String {
    format!("{:x}", Sha384::digest(plain_text.as_bytes()))
}

pub fn sha512(plain_text: &str) -> String {
    format!("{:x}", Sha512::digest(plain_text.as_bytes()))
}

pub fn sha512_224(plain_text: &str) -> String {
    format!("{:x}", Sha512_224::digest(plain_text.as_bytes()))
}

pub fn sha512_256(plain_text: &str) -> String {
    format!("{:x}", Sha512_256::digest(plain_text.as_bytes()))
}

pub fn get_all_sha2_fn_name() -> Vec<&'static str> {
    vec![
        "sha224",
        "sha256",
        "sha384",
        "sha512",
        "sha512_224",
        "sha512_256",
    ]
}

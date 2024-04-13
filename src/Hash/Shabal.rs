use shabal::{Digest, Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};

pub fn compare_shabal(cipher: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    cipher == hash_fn(plain_text)
}

pub fn shabal192(plain_text: &str) -> String {
    format!("{:x}", Shabal192::digest(plain_text.as_bytes()))
}

pub fn shabal224(plain_text: &str) -> String {
    format!("{:x}", Shabal224::digest(plain_text.as_bytes()))
}

pub fn shabal256(plain_text: &str) -> String {
    format!("{:x}", Shabal256::digest(plain_text.as_bytes()))
}

pub fn shabal384(plain_text: &str) -> String {
    format!("{:x}", Shabal384::digest(plain_text.as_bytes()))
}

pub fn shabal512(plain_text: &str) -> String {
    format!("{:x}", Shabal512::digest(plain_text.as_bytes()))
}

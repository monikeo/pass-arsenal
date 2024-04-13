use ripemd::{Digest, Ripemd128, Ripemd160, Ripemd256, Ripemd320};

pub fn compare_ripemd(cipher: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    cipher == hash_fn(plain_text)
}

pub fn ripemd128(plain_text: &str) -> String {
    format!("{:x}", Ripemd128::digest(plain_text.as_bytes()))
}

pub fn ripemd160(plain_text: &str) -> String {
    format!("{:x}", Ripemd160::digest(plain_text.as_bytes()))
}

pub fn ripemd256(plain_text: &str) -> String {
    format!("{:x}", Ripemd256::digest(plain_text.as_bytes()))
}

pub fn ripemd320(plain_text: &str) -> String {
    format!("{:x}", Ripemd320::digest(plain_text.as_bytes()))
}

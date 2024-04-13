use gost94::{Digest, Gost94CryptoPro, Gost94Test, Gost94UA, Gost94s2015};

pub fn compare_gost94(cipher: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    cipher == hash_fn(plain_text)
}

pub fn gost94_crypto_pro(plain_text: &str) -> String {
    format!("{:x}", Gost94CryptoPro::digest(plain_text.as_bytes()))
}

pub fn gost94_test(plain_text: &str) -> String {
    format!("{:x}", Gost94Test::digest(plain_text.as_bytes()))
}

pub fn gost94_ua(plain_text: &str) -> String {
    format!("{:x}", Gost94UA::digest(plain_text.as_bytes()))
}

pub fn gost94_s_2015(plain_text: &str) -> String {
    format!("{:x}", Gost94s2015::digest(plain_text.as_bytes()))
}

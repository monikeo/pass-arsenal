use sha3::{
    Digest, Keccak224, Keccak256, Keccak256Full, Keccak384, Keccak512, Sha3_224, Sha3_256,
    Sha3_384, Sha3_512,
};

pub fn compare_sha3(
    original_password: &str,
    plain_text: &str,
    sha3_fn: fn(&str) -> String,
) -> bool {
    original_password == sha3_fn(plain_text)
}

pub fn sha3_224(plain_text: &str) -> String {
    format!("{:x}", Sha3_224::digest(plain_text.as_bytes()))
}

pub fn sha3_256(plain_text: &str) -> String {
    format!("{:x}", Sha3_256::digest(plain_text.as_bytes()))
}

pub fn sha3_384(plain_text: &str) -> String {
    format!("{:x}", Sha3_384::digest(plain_text.as_bytes()))
}

pub fn sha3_512(plain_text: &str) -> String {
    format!("{:x}", Sha3_512::digest(plain_text.as_bytes()))
}

pub fn keccak224(plain_text: &str) -> String {
    format!("{:x}", Keccak224::digest(plain_text.as_bytes()))
}

pub fn keccak256(plain_text: &str) -> String {
    format!("{:x}", Keccak256::digest(plain_text.as_bytes()))
}

pub fn keccak256full(plain_text: &str) -> String {
    format!("{:x}", Keccak256Full::digest(plain_text.as_bytes()))
}

pub fn keccak384(plain_text: &str) -> String {
    format!("{:x}", Keccak384::digest(plain_text.as_bytes()))
}

pub fn keccak512(plain_text: &str) -> String {
    format!("{:x}", Keccak512::digest(plain_text.as_bytes()))
}

pub fn get_all_sha3_fn_name() -> Vec<&'static str> {
    vec![
        "sha3_224",
        "sha3_256",
        "sha3_384",
        "sha3_512",
        "keccak224",
        "keccak256",
        "keccak256full",
        "keccak384",
        "keccak512",
    ]
}

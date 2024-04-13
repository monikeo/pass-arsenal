use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn compare_base64(cipher: &str, plain_text: &str) -> bool {
    true
}

pub fn base64_encode(plain_text: &str) -> String {
    STANDARD.encode(plain_text.as_bytes())
}

pub fn base64_decode(cipher: &str) -> Option<String> {
    let bytes = STANDARD.decode(cipher).unwrap();
    match std::str::from_utf8(&bytes) {
        Ok(plain_text) => Some(plain_text.to_string()),
        Err(_) => None,
    }
}

use caesar_cipher::{decrypt, encrypt};

pub fn caesar_encode(plain_text: &str, offset: i32) -> String {
    encrypt(plain_text.to_string(), offset)
}

pub fn caesar_decode(cipher: &str, offset: i32) -> String {
    decrypt(cipher.to_string(), offset)
}

pub fn caesar_decode_custom_offset(
    cipher: &str,
    start_offset: i32,
    end_offset: i32,
) -> Vec<String> {
    let decoded = (start_offset..=end_offset)
        .map(|offset| caesar_decode(cipher, offset))
        .collect();
    decoded
}

pub fn caesar_decode_agressive(cipher: &str) -> Vec<String> {
    let decoded = (0..=26)
        .map(|offset| caesar_decode(cipher, offset))
        .collect();
    decoded
}

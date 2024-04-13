use urlencoding::{decode, encode};

pub fn url_encode(url_plain_text: &str) -> String {
    encode(url_plain_text).into_owned()
}

pub fn url_decode(url_cipher: &str) -> Result<String, &'static str> {
    let decoded = decode(url_cipher);
    match decoded {
        Ok(value) => Ok(value.into_owned()),
        Err(_) => Err("Failed to decode the provided URL"),
    }
}

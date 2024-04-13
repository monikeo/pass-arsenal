use password_cracker_rs::Encode::Url::*;

#[test]
fn test_url_encode() {
    let url = "https://www.ILoveYou.com/";
    let url_cipher = url_encode(url);
    assert_eq!(url_cipher, "https%3A%2F%2Fwww.ILoveYou.com%2F");
}

#[test]
fn test_url_decode() {
    let url_cipher = "https%3A%2F%2Fwww.ILoveYou.com%2F";
    let url = url_decode(url_cipher);

    match url {
        Ok(url_plain_text) => {
            assert_eq!(url_plain_text, "https://www.ILoveYou.com/")
        }
        Err(err_value) => assert_eq!(err_value, "Failed to decode the provided URL"),
    }
}

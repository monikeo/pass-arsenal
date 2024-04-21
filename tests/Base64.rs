use pass_arsenal::Encode::Base64::*;

#[test]
fn test_base64_encode() {
    let plain_text = "ILoveYou";
    let cipher = base64_encode(plain_text);
    assert_eq!(cipher, "SUxvdmVZb3U=");
}

#[test]
fn test_base64_decode() {
    let cipher = "SUxvdmVZb3U=";
    let plain_text = base64_decode(cipher);
    assert_eq!(plain_text.unwrap(), "ILoveYou");
}

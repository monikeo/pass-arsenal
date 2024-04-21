use pass_arsenal::Encode::CaesarCipher::*;

#[test]
fn test_caesar_decode_custom_offset() {
    let plain_text = "abcdefghijklmnopqrstuvwxyz";
    let caesar_cipher3 = caesar_encode(plain_text, 3);
    let caesar_cipher25 = caesar_encode(plain_text, 26);

    println!("{}", caesar_cipher3);
    println!("{}", caesar_cipher25);
}

#[test]
fn test_caesar_encode() {
    let plain_text = "ILoveYou";
    let cipher_text = caesar_encode(plain_text, 25);
    assert_eq!(cipher_text, "HKnudXnt");
}

#[test]
fn test_caesar_decode() {
    let cipher_text = "HKnudXnt";
    let plain_text = caesar_decode(cipher_text, 25);
    assert_eq!(plain_text, "ILoveYou");
}

use pass_arsenal::Encode::MorseCode::*;

#[test]
fn test_morse_code_encode() {
    let plain_text = "ILOVEYOU";
    let cipher = morse_code_encode(plain_text);
    assert_eq!(cipher, ".. ._.. ___ ..._ . _.__ ___ .._");
}

#[test]
fn test_morse_code_decode() {
    let cipher = ".. ._.. ___ ..._ . _.__ ___ .._";
    let plain_text = morse_code_decode(cipher).unwrap();
    assert_eq!(plain_text, "ILOVEYOU");
}

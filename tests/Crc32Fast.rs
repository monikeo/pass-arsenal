use ultimate_password_tool::Hash::Crc32Fast::*;

#[test]
fn test_crc32() {
    let plain_text = "ILoveYou";
    let cipher = crc32(plain_text);
    assert_eq!(cipher, "f2f45197");
}

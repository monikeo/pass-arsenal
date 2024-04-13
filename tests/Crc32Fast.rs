use password_cracker_rs::Hash::Crc32Fast::*;

#[test]
fn test_crc32() {
    let plain_text = "ILoveYou";
    let cipher = crc32(plain_text);
    assert_eq!(format!("{:02x}", cipher), "f2f45197");
}

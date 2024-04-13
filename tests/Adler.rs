use password_cracker_rs::Hash::Adler::*;
#[test]
fn test_adler32() {
    let plain_text = "ILoveYou";
    let alder32_checksum = adler32(plain_text);
    assert_eq!(format!("{:x}", alder32_checksum), "d3e031d");
}

use pass_arsenal::Hash::Adler::*;
#[test]
fn test_adler32() {
    let plain_text = "ILoveYou";
    let alder32_checksum = adler32(plain_text);
    assert_eq!(alder32_checksum, "d3e031d");
}

use pass_arsenal::Hash::Md5::*;

#[test]
fn test_md5() {
    let plain_text = "ILoveYou";
    let cipher = md5(plain_text);
    assert_eq!(cipher, "62accaf23ac9a73c0b28765b7dfaf75a");
}

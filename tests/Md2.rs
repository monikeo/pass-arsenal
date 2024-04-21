use pass_arsenal::Hash::Md2::*;

#[test]
fn test_md2() {
    let plain_text = "ILoveYou";
    let cipher = md2(plain_text);
    assert_eq!(cipher, "b8a2af055b43ff8c7d830d110ebb1919");
}

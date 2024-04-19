use ultimate_password_tool::Hash::Sha1::*;

#[test]
fn test_sha1() {
    let plain_text = "ILoveYou";
    let cipher = sha1(plain_text);
    assert_eq!(cipher, "c1d5d5370372ac65308f03f6ed75ec6068c8e1be");
}

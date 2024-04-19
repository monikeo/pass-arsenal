use ultimate_password_tool::Hash::Md4::*;

#[test]
fn test_md4() {
    let plain_text = "ILoveYou";
    let cipher = md4(plain_text);

    assert_eq!(cipher, "e8dfef894914b293febf73cadad00547");
}

use pass_arsenal::Hash::Tiger::*;

#[test]
fn test_tiger() {
    let plain_text = "ILoveYou";
    let cipher = tiger(plain_text);
    assert_eq!(cipher, "9613c8e3ef3fac6d7e71b5a488daf14ae88847abed76fcf6");
}

#[test]
fn test_tiger2() {
    let plain_text = "ILoveYou";
    let cipher = tiger2(plain_text);
    assert_eq!(cipher, "36d363640b6f952b0df40efe5b843569a8c0fdb5e5f368bb");
}

use pass_arsenal::Hash::Streebog::*;

#[test]
fn test_streebog256() {
    let plain_text = "ILoveYou";
    let cipher = streebog256(plain_text);
    assert_eq!(
        cipher,
        "2548003a67bc5794439f417c753700939396ef7ab74092c440947dae7efd7ccb"
    );
}

#[test]
fn test_streebog512() {
    let plain_text = "ILoveYou";
    let cipher = streebog512(plain_text);
    assert_eq!(cipher, "f3e3d217bfd20560f3db25744b1483a4cf2a45c66a517f25308d54394ef89174bfa183fc3cb059a6a32994d8934dc8af2c295d49f4c64af54aebac775a8ba6ea");
}

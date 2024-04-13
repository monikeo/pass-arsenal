use password_cracker_rs::Hash::Sm3::*;

#[test]
fn test_sm3() {
    let plain_text = "ILoveYou";
    let cipher = sm3(plain_text);
    assert_eq!(
        cipher,
        "9943a2eeeec526025e1dc28fc2c936039866a42626663081e18d52e281d3719a"
    );
}

use ultimate_password_tool::Hash::Groestl::*;

#[test]
fn test_groestl224() {
    let plain_text = "ILoveYou";
    let cipher = groestl224(plain_text);
    assert_eq!(
        cipher,
        "5c1098aceb3fb20225d3a066c8a9532b9395424779712fa0559e2754"
    );
    //println!("{}", cipher);
}

#[test]
fn test_groestl256() {
    let plain_text = "ILoveYou";
    let cipher = groestl256(plain_text);
    assert_eq!(
        cipher,
        "bd6009826e0f51c2f51169dc6a40519e66df9bab50197bd461212db712ad53b9"
    );
    //println!("{}", cipher);
}

#[test]
fn test_groestl384() {
    let plain_text = "ILoveYou";
    let cipher = groestl384(plain_text);
    assert_eq!(cipher, "0a282341077e10d1f26a1d9407d6b8cd2155f19c2a478c6cb82fb7b01dae25345e63616fbf163c2f6d5ffd61e7439751");
    //println!("{}", cipher);
}

#[test]
fn test_groestl512() {
    let plain_text = "ILoveYou";
    let cipher = groestl512(plain_text);
    assert_eq!(cipher, "f74d1dca64cd8a7207bafe1171aba8ab284ed35e9e778a50bb4a387dcbaaa3b1497c35ea6665a0fe3348c1780e1e6037fbc68b686598fd086a6ecd3a2028e5e0");
    //println!("{}", cipher);
}

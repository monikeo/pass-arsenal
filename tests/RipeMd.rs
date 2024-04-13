use password_cracker_rs::Hash::RipeMD::*;

#[test]
fn test_ripemd128() {
    let plain_text = "ILoveYou";
    let cipher = ripemd128(plain_text);
    assert_eq!(cipher, "c72bcc955c2f8e5446f835841e546c06");
}

#[test]
fn test_ripemd160() {
    let plain_text = "ILoveYou";
    let cipher = ripemd160(plain_text);
    assert_eq!(cipher, "e7dfe8ceb769e465895b88814e3812af77e2a28b");
}

#[test]
fn test_ripemd256() {
    let plain_text = "ILoveYou";
    let cipher = ripemd256(plain_text);
    assert_eq!(
        cipher,
        "a6c62e2e58a6512ab7673bafb775bba86b49eaf7a7ab14a7e42ce43f8194b6ea"
    );
}

#[test]
fn test_ripemd320() {
    let plain_text = "ILoveYou";
    let cipher = ripemd320(plain_text);
    assert_eq!(
        cipher,
        "a231604d6878f915affb8f174faf05fcf19476fe774f77c29f1d9547cbe22e24b1648d000f20db1c"
    );
}

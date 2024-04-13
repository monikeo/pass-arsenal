use password_cracker_rs::Hash::Blake2::*;

#[test]
fn test_blake2s256() {
    let plain_text = "ILoveYou";
    let cipher = blake2s256(plain_text);
    assert_eq!(
        cipher,
        "78eb828373e9a457885f00ac6aff82572989e9f1e066a2b14d971e4b0c317a69"
    );
}

#[test]
fn test_blake2b512() {
    let plain_text = "ILoveYou";
    let cipher = blake2b512(plain_text);
    assert_eq!(cipher, "eb366ffd8285c3ede8e024d94edbad5d73bf2d87f27354e78660d3ef037c64a10b76d761acf7adbc25a67810c1011ec53a08d90966c8dbdc3e68f0ca61071004");
}

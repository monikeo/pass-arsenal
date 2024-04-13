use password_cracker_rs::Hash::Shabal::*;

#[test]
fn test_shabal192() {
    let plain_text = "ILoveYou";
    let cipher = shabal192(plain_text);
    assert_eq!(cipher, "a908206f3792acbbca59869ba050592a45b60a3398c39fce");
}

#[test]
fn test_shabal224() {
    let plain_text = "ILoveYou";
    let cipher = shabal224(plain_text);
    assert_eq!(
        cipher,
        "4944249ffca7ac4218f06780f1e8f12bba9f47a79beb50703c9d5c74"
    );
}

#[test]
fn test_shabal256() {
    let plain_text = "ILoveYou";
    let cipher = shabal256(plain_text);
    assert_eq!(
        cipher,
        "42e58497c4500461f8d8db3ab25bdf22d8dd2d48481727c4911a088e99da7771"
    );
}

#[test]
fn test_shabal384() {
    let plain_text = "ILoveYou";
    let cipher = shabal384(plain_text);
    assert_eq!(
        cipher,
        "0d0f776cfcfdfac09274812ce28c50c337188ad4152fd627c2de230b6e2816c6f24b75191654afddf9fca292e4c6dc94"
    );
}

#[test]
fn test_shabal512() {
    let plain_text = "ILoveYou";
    let cipher = shabal512(plain_text);
    assert_eq!(cipher, "be2cbd3f1fb63e595e4a1e0509d2fdf6b4057c218549f726c13b9019903496b5c5cca8e3a515f825162ed61cd8a371235f1a05658cae51156a4acc0057779683");
}

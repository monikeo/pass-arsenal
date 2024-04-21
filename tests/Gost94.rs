use pass_arsenal::Hash::Gost94::*;

#[test]
fn test_gost94_crypto_pro() {
    let plain_text = "ILoveYou";
    let cipher = gost94_crypto_pro(plain_text);
    assert_eq!(
        cipher,
        "08b304f98a0abaf265d489364522d86dc97aba1d10eb02980f5c30284abfeed8"
    );
}

#[test]
fn test_gost94_test() {
    let plain_text = "ILoveYou";
    let cipher = gost94_test(plain_text);
    assert_eq!(
        cipher,
        "6c86149193c601c33f73733267a32b13a98bfb6e90ccd6e1b2c5e1dedda03487"
    );
}

#[test]
fn test_gost94_ua() {
    let plain_text = "ILoveYou";
    let cipher = gost94_ua(plain_text);
    assert_eq!(
        cipher,
        "e605adb5174466dd60252564391f9597ce9fc7d79e3063e94efa7a21a9dd2274"
    );
}

#[test]
fn test_gost94_s_2015() {
    let plain_text = "ILoveYou";
    let cipher = gost94_s_2015(plain_text);
    assert_eq!(
        cipher,
        "92f1bdbca243f0237aa6d96ff4c9667c0036acacc8981d9ebd09f2515931c928"
    );
}

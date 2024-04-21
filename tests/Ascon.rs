use pass_arsenal::Hash::Ascon::*;

#[test]
fn test_ascona() {
    let plain_text = "ILoveYou";
    let cipher = ascona(plain_text);
    //println!("{}", cipher);
    assert_eq!(
        cipher,
        "0c2528248905bc83c45e5f568ff581b9ff81b93cce89446166771eb281008fca"
    );
}

#[test]
fn test_ascon() {
    let plain_text = "ILoveYou";
    let cipher = ascon(plain_text);
    //println!("{}", cipher);
    assert_eq!(
        cipher,
        "339fcc7a9696c1193c11034959da40eddaf8f327b15ae136bbeddd23e8f0540f"
    );
}

#[test]
fn test_asconxof() {
    let plain_text = "ILoveYou";
    let cipher = asconxof(plain_text);
    println!("{}", cipher);
    /*
    assert_eq!(
        cipher,
        "e38389276d4b04bfe40bc2b020fffa6a5a2af91647120979796ceb7a867aeeb8"
    );
    */
}

#[test]
fn test_asconaxof() {
    let plain_text = "ILoveYou";
    let cipher = asconaxof(plain_text);
    println!("{}", cipher);
    /*
    assert_eq!(
        cipher,
        "1b900b772be123f468ec3728e4b742fe8b843eb766ac20e991782683f186717d"
    );
    */
}

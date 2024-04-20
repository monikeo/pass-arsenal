use ascon_hash::{
    AsconAHash, AsconAXof, AsconHash, AsconXof, Digest, ExtendableOutput, Update, XofReader,
};

pub fn ascona(plain_text: &str) -> String {
    format!("{:x}", AsconAHash::digest(plain_text.as_bytes()))
}

pub fn ascon(plain_text: &str) -> String {
    format!("{:x}", AsconHash::digest(plain_text.as_bytes()))
}
pub fn asconxof(plain_text: &str) -> String {
    let mut xof = AsconXof::default();
    xof.update(plain_text.as_bytes());
    let mut reader = xof.finalize_xof();
    let mut dst = [0u8; 32];
    reader.read(&mut dst);
    //format!("{:?}", &dst)
    dst.iter()
        .map(|byte| format!("{:x}", byte))
        .collect::<String>()
}

pub fn asconaxof(plain_text: &str) -> String {
    let mut axof = AsconAXof::default();
    axof.update(plain_text.as_bytes());
    let mut reader = axof.finalize_xof();
    let mut dst = [0u8; 32];
    reader.read(&mut dst);
    dst.iter()
        .map(|byte| format!("{:x}", byte))
        .collect::<String>()
    //let text = format!("{:?}", &dst);
    //format!("{:x}", text)
}

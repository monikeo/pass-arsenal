use skein::{consts::U32, Digest, Skein1024, Skein256, Skein512};

pub fn skein256(plain_text: &str) -> String {
    format!("{:x}", Skein256::<U32>::digest(plain_text.as_bytes()))
}

pub fn skein512(plain_text: &str) -> String {
    format!("{:x}", Skein512::<U32>::digest(plain_text.as_bytes()))
}

pub fn skein1024(plain_text: &str) -> String {
    format!("{:x}", Skein1024::<U32>::digest(plain_text.as_bytes()))
}

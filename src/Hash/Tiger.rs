use tiger::{Digest, Tiger, Tiger2};

/*
Tiger is a 192-bit hash function, and was designed by Ross Anderson and Eli Biham in 1995.
It is often used by clients within Gnutella file sharing networks, and does not suffer from known attacks on MD5 and SHA-0/SHA-1.
Tiger2 is an addition, in which the message is padded with a byte of 0x80 (in a similar way to MD4, MD5 and SHA),
whereas in Tiger it is 0x01.
Otherwise the two methods are the same in their operation
 */

pub fn compare_tiger(cipher: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    cipher == hash_fn(plain_text)
}

// tiger192-3
pub fn tiger(plain_text: &str) -> String {
    format!("{:x}", Tiger::digest(plain_text.as_bytes()))
}

pub fn tiger2(plain_text: &str) -> String {
    format!("{:x}", Tiger2::digest(plain_text.as_bytes()))
}

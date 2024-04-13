use sm3::{Digest, Sm3};
/*
 SM3 is cryptographic hash function designed by Xiaoyun Wang, et al.
 The hash is part of the Chinese State Cryptography Administration portfolio

SM3 is used for implementing digital signatures, message authentication codes, and pseudorandom number generators.
The algorithm is public and is considered similar to SHA-256 in security and efficiency.
SM3 is used with Transport Layer Security.
 */

pub fn compare_sm3(cipher: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    cipher == hash_fn(plain_text)
}

pub fn sm3(plain_text: &str) -> String {
    format!("{:x}", Sm3::digest(plain_text.as_bytes()))
}

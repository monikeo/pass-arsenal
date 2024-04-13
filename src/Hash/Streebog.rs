use streebog::{Digest, Streebog256, Streebog512};
/*
 * The Streebog hash family, specified in the GOST R 34.11-2012 specification,
 * is a set of cryptographic hash functions standardized by the Russian government.
 * Streebog is named after the Russian word "Streegog," which means whirlpool or vortex.
 * Streebog should not be confused with any of the GOST hash family, which are described in other documents.
 *
 * The Streebog hash functions are based on the Merkle–Damgård construction with a compression function that utilizes bitwise Boolean operations and modular addition.
 * Streebog has been adopted for various applications, including secure communications, digital signatures, and authentication, particularly in Russian cryptographic systems and protocols.
 * However, its usage outside Russia is relatively limited compared to widely accepted international standards like SHA-2 and SHA-3.
 */

pub fn compare_streebog(cipher: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    cipher == hash_fn(plain_text)
}

pub fn streebog256(plain_text: &str) -> String {
    format!("{:x}", Streebog256::digest(plain_text.as_bytes()))
}

pub fn streebog512(plain_text: &str) -> String {
    format!("{:x}", Streebog512::digest(plain_text.as_bytes()))
}

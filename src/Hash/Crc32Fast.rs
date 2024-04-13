use crc32fast::Hasher;

pub fn compare_crc32(crc32_hash: &str, plain_text: &str, hash_fn: fn(&str) -> String) -> bool {
    crc32_hash == hash_fn(plain_text)
}

/*
pub fn crc32(plain_text: &str) -> u32 {
    let mut crc32_hasher = Hasher::new();
    crc32_hasher.update(plain_text.as_bytes());
    crc32_hasher.finalize()
}
*/

pub fn crc32(plain_text: &str) -> String {
    let mut crc32_hasher = Hasher::new();
    crc32_hasher.update(plain_text.as_bytes());
    format!("{:x}", crc32_hasher.finalize())
}

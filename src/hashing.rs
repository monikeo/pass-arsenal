use crate::Hash::{
    Adler, Blake2, Crc32Fast, Gost94, Groestl, Md2, Md4, Md5, RipeMD, Sha1, Sha2, Sha3, Shabal,
    Skein, Sm3, Streebog, Tiger, Whirlpool,
};
use std::collections::BTreeMap;

pub fn call_hash_function(plain_text: &str, hash_function: fn(&str) -> String) -> String {
    hash_function(plain_text)
}

// Compute hash values for MD hash families
pub fn md_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut md_hash_fn: BTreeMap<&'static str, String> = BTreeMap::new();
    md_hash_fn.insert("md2", Md2::md2(plain_text));
    md_hash_fn.insert("md4", Md4::md4(plain_text));
    md_hash_fn.insert("md5", Md5::md5(plain_text));
    md_hash_fn
}

pub fn print_md_hash(plain_text: &str) {
    let md_hash_collections = md_hash_families(plain_text);
    print_hash_data("MD Hash", &md_hash_collections);
    println!();
}

pub fn sha1_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut sha1_hash_fn = BTreeMap::new();
    sha1_hash_fn.insert("sha1", Sha1::sha1(plain_text));
    sha1_hash_fn
}

pub fn print_sha1_hash(plain_text: &str) {
    let sha1_hash_collections = sha1_hash_families(plain_text);
    print_hash_data("Sha1 Hash", &sha1_hash_collections);
    println!();
}

pub fn sha2_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut sha2_hashs = BTreeMap::new();
    sha2_hashs.insert("sha224", Sha2::sha224(plain_text));
    sha2_hashs.insert("sha256", Sha2::sha256(plain_text));
    sha2_hashs.insert("sha384", Sha2::sha384(plain_text));
    sha2_hashs.insert("sha512", Sha2::sha512(plain_text));
    sha2_hashs.insert("sha512_224", Sha2::sha512_224(plain_text));
    sha2_hashs.insert("sha512_256", Sha2::sha512_256(plain_text));
    sha2_hashs
}

pub fn print_sha2_hash(plain_text: &str) {
    let sha2_hash_collections = sha2_hash_families(plain_text);
    print_hash_data("Sha2 Hash", &sha2_hash_collections);
    println!();
}

pub fn sha3_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut sha3_hashs = BTreeMap::new();
    sha3_hashs.insert("sha3_224", Sha3::sha3_224(plain_text));
    sha3_hashs.insert("sha3_256", Sha3::sha3_256(plain_text));
    sha3_hashs.insert("sha3_384", Sha3::sha3_384(plain_text));
    sha3_hashs.insert("sha3_512", Sha3::sha3_512(plain_text));
    sha3_hashs.insert("keccak224", Sha3::keccak224(plain_text));
    sha3_hashs.insert("keccak256", Sha3::keccak256(plain_text));
    sha3_hashs.insert("keccak256full", Sha3::keccak256full(plain_text));
    sha3_hashs.insert("keccak384", Sha3::keccak384(plain_text));
    sha3_hashs.insert("keccak512", Sha3::keccak512(plain_text));
    sha3_hashs
}

pub fn print_sha3_hash(plain_text: &str) {
    let sha3_hash_collections = sha3_hash_families(plain_text);
    print_hash_data("Sha3 Hash", &sha3_hash_collections);
    println!();
}

pub fn blake2_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut blake2_hashs = BTreeMap::new();
    blake2_hashs.insert("blake2s256", Blake2::blake2s256(plain_text));
    blake2_hashs.insert("blake2b512", Blake2::blake2b512(plain_text));
    blake2_hashs
}

pub fn print_blake2_hash(plain_text: &str) {
    let blake2_hash_collections = blake2_hash_families(plain_text);
    print_hash_data("Blake2 Hash", &blake2_hash_collections);
    println!();
}

pub fn skein_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut skein_hashs = BTreeMap::new();
    skein_hashs.insert("skein256", Skein::skein256(plain_text));
    skein_hashs.insert("skein512", Skein::skein512(plain_text));
    skein_hashs.insert("skein1024", Skein::skein1024(plain_text));
    skein_hashs
}

pub fn print_skein_hash_families(plain_text: &str) {
    let skein_hash_collections = skein_hash_families(plain_text);
    print_hash_data("Skein Hash", &skein_hash_collections);
    println!();
}

pub fn ripemd_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut ripemd_hashs = BTreeMap::new();
    ripemd_hashs.insert("ripemd128", RipeMD::ripemd128(plain_text));
    ripemd_hashs.insert("ripemd160", RipeMD::ripemd160(plain_text));
    ripemd_hashs.insert("ripemd256", RipeMD::ripemd256(plain_text));
    ripemd_hashs.insert("ripemd320", RipeMD::ripemd320(plain_text));
    ripemd_hashs
}

pub fn print_ripemd_hash(plain_text: &str) {
    let ripemd_hash_collections = ripemd_hash_families(plain_text);
    print_hash_data("RipeMD Hash", &ripemd_hash_collections);
    println!();
}

pub fn tiger_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut tiger_hashs = BTreeMap::new();
    tiger_hashs.insert("tiger", Tiger::tiger(plain_text));
    tiger_hashs.insert("tiger2", Tiger::tiger2(plain_text));
    tiger_hashs
}

pub fn print_tiger_hash(plain_text: &str) {
    let tiger_hash_collections = tiger_hash_families(plain_text);
    print_hash_data("Tiger Hash", &tiger_hash_collections);
    println!();
}

pub fn shabal_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut shabal_hashs = BTreeMap::new();
    shabal_hashs.insert("shabal192", Shabal::shabal192(plain_text));
    shabal_hashs.insert("shabal224", Shabal::shabal224(plain_text));
    shabal_hashs.insert("shabal256", Shabal::shabal256(plain_text));
    shabal_hashs.insert("shabal384", Shabal::shabal384(plain_text));
    shabal_hashs.insert("shabal512", Shabal::shabal512(plain_text));
    shabal_hashs
}

pub fn print_shabal_hash(plain_text: &str) {
    let shabal_hash_collections = shabal_hash_families(plain_text);
    print_hash_data("Shabal Hash", &shabal_hash_collections);
    println!();
}

pub fn gost94_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut gost94_hashs = BTreeMap::new();
    gost94_hashs.insert("gost94_crypto_pro", Gost94::gost94_crypto_pro(plain_text));
    gost94_hashs.insert("gost94_test", Gost94::gost94_test(plain_text));
    gost94_hashs.insert("gost94_ua", Gost94::gost94_ua(plain_text));
    gost94_hashs.insert("gost94_s_2015", Gost94::gost94_s_2015(plain_text));
    gost94_hashs
}

pub fn print_gost94_hash(plain_text: &str) {
    let gost94_hash_collections = gost94_hash_families(plain_text);
    print_hash_data("Gost94 Hash", &gost94_hash_collections);
    println!();
}

pub fn groestl_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut groestl_hashs = BTreeMap::new();
    groestl_hashs.insert("groestl224", Groestl::groestl224(plain_text));
    groestl_hashs.insert("groestl256", Groestl::groestl256(plain_text));
    groestl_hashs.insert("groestl384", Groestl::groestl384(plain_text));
    groestl_hashs.insert("groestl512", Groestl::groestl512(plain_text));
    groestl_hashs
}

pub fn print_groestl_hash(plain_text: &str) {
    let groestl_hash_collections = groestl_hash_families(plain_text);
    print_hash_data("Groestl Hash", &groestl_hash_collections);
    println!();
}

pub fn streebog_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut streebog_hashs = BTreeMap::new();
    streebog_hashs.insert("streebog256", Streebog::streebog256(plain_text));
    streebog_hashs.insert("streebog512", Streebog::streebog512(plain_text));
    streebog_hashs
}

pub fn print_streebog_hash_families(plain_text: &str) {
    let streebog_hash_collections = streebog_hash_families(plain_text);
    print_hash_data("Streebog Hash", &streebog_hash_collections);
    println!();
}

pub fn sm3_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut sm3_hashs = BTreeMap::new();
    sm3_hashs.insert("sm3", Sm3::sm3(plain_text));
    sm3_hashs
}

pub fn print_sm3_hash_families(plain_text: &str) {
    let sm3_hash_collections = sm3_hash_families(plain_text);
    print_hash_data("Sm3", &sm3_hash_collections);
    println!();
}

pub fn adler_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut adler_hashs = BTreeMap::new();
    adler_hashs.insert("adler32", Adler::adler32(plain_text));
    adler_hashs
}

pub fn print_adler_hash_families(plain_text: &str) {
    let adler_hash_collections = adler_hash_families(plain_text);
    print_hash_data("Adler32", &adler_hash_collections);
    println!();
}

pub fn crc32_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut crc_hashs = BTreeMap::new();
    crc_hashs.insert("crc32", Crc32Fast::crc32(plain_text));
    crc_hashs
}

pub fn print_crc32_hash_families(plain_text: &str) {
    let crc32_hash_collections = crc32_hash_families(plain_text);
    print_hash_data("Crc32", &crc32_hash_collections);
    println!();
}

pub fn whirlpool_hash_families(plain_text: &str) -> BTreeMap<&'static str, String> {
    let mut whirlpool_hashs = BTreeMap::new();
    whirlpool_hashs.insert("whirlpool", Whirlpool::whirlpool(plain_text));
    whirlpool_hashs
}

pub fn print_whirlpool_hash_families(plain_text: &str) {
    let whirlpool_hash_collections = whirlpool_hash_families(plain_text);
    print_hash_data("Whirlpool", &whirlpool_hash_collections);
    println!();
}

pub fn print_ultimate_hash(plain_text: &str) {
    print_md_hash(plain_text);
    print_sha1_hash(plain_text);
    print_sha2_hash(plain_text);
    print_sha3_hash(plain_text);
    print_blake2_hash(plain_text);
    print_skein_hash_families(plain_text);
    print_ripemd_hash(plain_text);
    print_tiger_hash(plain_text);
    print_shabal_hash(plain_text);
    print_gost94_hash(plain_text);
    print_groestl_hash(plain_text);
    print_streebog_hash_families(plain_text);
    print_sm3_hash_families(plain_text);
    print_adler_hash_families(plain_text);
    print_crc32_hash_families(plain_text);
    print_whirlpool_hash_families(plain_text);
}

pub fn print_hash_data(title: &str, data: &BTreeMap<&'static str, String>) {
    println!("{}", title.to_uppercase());
    for (key, value) in data.iter() {
        println!("{} :\t {}", key, value);
    }
}

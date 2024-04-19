use crate::Hash::*;
use rayon::prelude::*;

pub fn dictionary_attack(
    victim_password: &str,
    algorithm_hash_name: &str,
    list_passwords: &[String],
) -> Option<String> {
    let hash_algorithm = match algorithm_hash_name {
        "md2" => Md2::md2,
        "md4" => Md4::md4,
        "md5" => Md5::md5,
        "sha1" => Sha1::sha1,
        "sha224" => Sha2::sha224,
        "sha256" => Sha2::sha256,
        "sha384" => Sha2::sha384,
        "sha512" => Sha2::sha512,
        "sha512_224" => Sha2::sha512_224,
        "sha512_256" => Sha2::sha512_256,
        "sha3_224" => Sha3::sha3_224,
        "sha3_256" => Sha3::sha3_256,
        "sha3_384" => Sha3::sha3_384,
        "sha3_512" => Sha3::sha3_512,
        "keccak224" => Sha3::keccak224,
        "keccak256" => Sha3::keccak256,
        "keccak256full" => Sha3::keccak256full,
        "keccak384" => Sha3::keccak384,
        "keccak512" => Sha3::keccak512,
        "blake2s256" => Blake2::blake2s256,
        "blake2sb512" => Blake2::blake2b512,
        "ripemd128" => RipeMD::ripemd128,
        "ripemd160" => RipeMD::ripemd160,
        "ripemd256" => RipeMD::ripemd256,
        "ripemd320" => RipeMD::ripemd320,
        "tiger" => Tiger::tiger,
        "tiger2" => Tiger::tiger2,
        "shabal192" => Shabal::shabal192,
        "shabal224" => Shabal::shabal224,
        "shabal256" => Shabal::shabal256,
        "shabal384" => Shabal::shabal384,
        "shabal512" => Shabal::shabal512,
        "gost94_crypto_pro" => Gost94::gost94_crypto_pro,
        "gost94_test" => Gost94::gost94_test,
        "gost94_ua" => Gost94::gost94_ua,
        "gost94_s_2015" => Gost94::gost94_s_2015,
        "groestl224" => Groestl::groestl224,
        "groestl256" => Groestl::groestl256,
        "groestl384" => Groestl::groestl384,
        "groestl512" => Groestl::groestl512,
        "streebog256" => Streebog::streebog256,
        "streebog512" => Streebog::streebog512,
        "sm3" => Sm3::sm3,
        "adler32" | "adler" => Adler::adler32,
        "crc32" | "crc" => Crc32Fast::crc32,
        "whirlpool" => Whirlpool::whirlpool,
        _ => return None,
    };
    let mut attempts = 1;
    println!(
        "Attempting to crack: \"{}\" in {}!\n",
        victim_password, algorithm_hash_name
    );
    /*
    for (index, password) in list_passwords.iter().enumerate() {
        let password = password.trim();
        let password_hash = hash_algorithm(password)
        println!("[{}] {} == {}", attempts, password, password_hash,);
        if victim_password == password_hash {
            return Some(password.to_string());
        }
        attempts += 1;
    }
    */

    let test = list_passwords
        .par_chunks(1000)
        .find_any(|chunk| {
            chunk
                .iter()
                .any(|password| victim_password == hash_algorithm(password.trim()))
        })
        .and_then(|chunk| {
            chunk
                .iter()
                .find(|password| victim_password == hash_algorithm(password.trim()))
        })
        .map(|s| s.to_owned());
    let test1 = list_passwords
        .par_iter()
        .find_first(|password| victim_password == hash_algorithm(password.trim()))
        .map(|s| s.to_owned());
    test1
}

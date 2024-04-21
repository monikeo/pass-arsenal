use clap::Parser;
use pass_arsenal::{
    Cli::{Args, SubCommands},
    Encode::{Base64, CaesarCipher, MorseCode, Url},
    Encryption,
};

use pass_arsenal::hashing::*;
use pass_arsenal::PasswordStrength::PasswordStrengthCriteria::password_strength_criteria::*;
fn test() {
    let password = "ILoveYou";
    let flag = pwned_password_exposure(password);
    if flag {
        println!("FOund");
    } else {
        println!("nono");
    }
}

fn ultimate_encode(plain_text: &str) {
    todo!();
}

fn ultimate_encryption(plain_text: &str) {
    todo!();
}

fn run() {
    let args = Args::parse();

    if let Some(all) = args.all() {
        println!("execute change file name: {}", all);
        print_ultimate_hash(all);
    }
}

fn main() {
    test();
    //run();
}

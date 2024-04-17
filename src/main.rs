use clap::Parser;
use ultimate_password_tool::{
    Cli::{Args, SubCommands},
    Encode::{Base64, CaesarCipher, MorseCode, Url},
    Encryption,
};

use ultimate_password_tool::hashing::*;
use ultimate_password_tool::PasswordStrength::PasswordStrengthCriteria::password_strength_criteria::*;
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

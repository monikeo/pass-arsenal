use adler::Adler32;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, Read};

pub fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}

/*
pub fn adler32(plain_text: &str) -> u32 {
    let mut sum = Adler32::new();
    sum.write_slice(plain_text.as_bytes());
    sum.checksum()
}
*/
pub fn adler32(plain_text: &str) -> String {
    let mut sum = Adler32::new();
    sum.write_slice(plain_text.as_bytes());
    format!("{:x}", sum.checksum())
}

/*
pub fn adler32_file(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut buffer = Vec::new();
    buf_reader.read_to_end(&mut buffer)?;
    Ok(adler32(&String::from_utf8_lossy(&buffer)))
}
*/

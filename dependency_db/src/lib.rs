use std::fs;
use std::io::{self, Read, Write};

pub fn load(path: &str) -> Result<Vec<u8>, io::Error> {
    let bytes = fs::read(path)?;
    Ok(bytes)
}

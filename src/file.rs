extern crate failure;


use std::fs::OpenOptions;
use std::io::prelude::*;
use std::{fs, io};

pub fn load_string(path : &str) -> io::Result<String> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn write_string(path : &str, string : &str) -> Result<(), failure::Error> {
    let mut file = match OpenOptions::new().write(true).open(path) {
        Ok(f) => f,
        Err(e) => {
            println!("File Open to Write Error: {:?}", e);
            fs::File::create(path)?
        }
    };
    
    file.write_all(string.as_bytes())?;

    Ok(())
}

pub fn load_codes(path : &str, code_length : usize) -> io::Result<Vec<String>> {
    let contents = load_string(path)?;

    let v : Vec<String> = contents
        .split("\n")
        .filter(|x| x.len() == code_length)
        .map(|s| s.to_owned())
        .collect();

    Ok(v)
}
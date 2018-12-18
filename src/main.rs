extern crate dlcode_generator;
use dlcode_generator::{file, code};

fn main() {
    // Input and Output file name
    let filename = "sample.txt";
    // The nubmer of new codes to generate
    let gen_code_num = 30;
    let code_length = 16;

    let prev_codes : Vec<_> = 
        file::load_codes(&filename, code_length)
        .unwrap_or_default()
        .into_iter().collect();
    
    let chars : Vec<char> = "ABCDEFGHJKLMNPRSTUVWXYZ23456789".chars().collect();

    let codes = code::gen_codes(&prev_codes, &chars, code_length, gen_code_num);

    match file::write_string(
        &filename, 
        &format!("{}{}", if prev_codes.len() == 0 {""} else {"\n"}, codes.join("\n")), 
        true) {
        Ok(_) => println!("Write Codes to {}", &filename),
        Err(e) => println!("{:?}", e),
    }
    
}

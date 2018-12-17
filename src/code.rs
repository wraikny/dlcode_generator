extern crate rayon;
extern crate rand;

use rayon::prelude::*;
use std::collections::{HashSet};
use rand::{Rng, thread_rng};

fn gen_code(chars : &Vec<char>, char_num : usize) -> String {
    let mut rng = thread_rng();
    let code : String = 
        (0..char_num)
        .map(|_| (rng.gen::<f32>() * ((chars.len() - 1) as f32)) as usize)
        .map(|i| chars[i])
        .collect();
    
    code
}

pub fn gen_codes(prev_codes : &Vec<String>, chars : &Vec<char>, char_num : usize, code_num : usize) -> Vec<String> {
    let codes : Vec<_> = (0..code_num).into_par_iter().map(|_|{
        gen_code(chars, char_num)
    }).collect();

    let mut prev_codes_mut = prev_codes.clone();
    let mut codes_appended = codes.clone();
    codes_appended.append(&mut prev_codes_mut);
    
    let codes_appended_hashset : HashSet<_> = codes_appended.clone().into_iter().collect();

    let generated_codes_num = codes_appended_hashset.len() - prev_codes.len();

    if generated_codes_num == code_num {
        codes.into_iter().collect()
    } else {
        let mut codes_fill = gen_codes(&codes_appended, chars, char_num, code_num - generated_codes_num);
        let mut codes = codes;
        codes.append(&mut codes_fill);
        codes
    }
}
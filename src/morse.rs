pub mod code;

use crate::morse::code::*;
use heapless::String;

pub fn codes_to_string(codes: &[[u8; 5]; 64]) -> String<64> {
    let mut string = String::new();

    for code in codes {
        string.push(Code::code_to_char(code)).unwrap();
    }

    string
}

pub fn string_to_codes(string: &String<64>) -> [Code; 64] {
    let mut codes = [Code::None; 64];

    for (index, mut char) in string.chars().enumerate() {
        char.make_ascii_lowercase();
        codes[index] = Code::char_to_code(&char)
    }

    codes
}

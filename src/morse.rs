pub mod code;

use crate::morse::code::*;
use crate::BUFFER_LENGTH;
use heapless::String;

pub fn codes_to_string(codes: &[[u8; 5]; BUFFER_LENGTH]) -> String<BUFFER_LENGTH> {
    let mut string = String::new();

    for code in codes {
        string.push(Code::Some(*code).to_char()).unwrap();
    }

    string
}

pub fn string_to_codes(string: &String<BUFFER_LENGTH>) -> [Code; BUFFER_LENGTH] {
    let mut codes = [Code::None; BUFFER_LENGTH];

    for (index, mut char) in string.chars().enumerate() {
        char.make_ascii_lowercase();
        codes[index] = Code::char_to_code(&char)
    }

    codes
}

pub mod code;

use crate::morse::code::*;
use crate::BUFFER_LENGTH;
use heapless::String;

pub fn codes_to_string(codes: &[Code; BUFFER_LENGTH]) -> String<BUFFER_LENGTH> {
    let mut string = String::new();

    for code in codes {
        match code {
            Code::Some(_) => {
                string.push(code.to_char()).unwrap();
            }
            Code::Space => {
                string.push(' ').unwrap();
            }
            Code::Error => {
                string.push('%').unwrap();
            }
            Code::None => {}
        }
    }

    string
}

pub fn to_marks(codes: &[Code; BUFFER_LENGTH]) -> String<BUFFER_LENGTH> {
    let mut string = String::<BUFFER_LENGTH>::new();

    for code in codes {
        match code {
            Code::Some(code) => {
                for byte in code {
                    if *byte == 1 {
                        string.push('.').unwrap();
                    } else if *byte == 2 {
                        string.push('-').unwrap();
                    }
                }
                string.push(' ').unwrap();
            }
            Code::Space => {
                string.push(' ').unwrap();
                string.push(' ').unwrap();
            }
            Code::Error => {
                string.push('%').unwrap();
            }
            Code::None => {}
        }
    }

    String::from(string.trim())
}

pub fn string_to_codes(string: &String<BUFFER_LENGTH>) -> [Code; BUFFER_LENGTH] {
    let mut codes = [Code::None; BUFFER_LENGTH];

    for (index, mut char) in string.chars().enumerate() {
        char.make_ascii_lowercase();
        codes[index] = Code::char_to_code(&char)
    }

    codes
}

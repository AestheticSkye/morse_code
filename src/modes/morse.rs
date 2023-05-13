pub mod code;

use heapless::String;

use crate::{
	modes::morse::code::{Code, Mark},
	BUFFER_LENGTH,
};

/// Converts a morse code array to a string
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

/// Converts a morse code array to a string of marks
pub fn to_marks(codes: &[Code; BUFFER_LENGTH]) -> String<BUFFER_LENGTH> {
	let mut string = String::<BUFFER_LENGTH>::new();

	for code in codes {
		match code {
			Code::Some(code) => {
				for byte in code {
					match byte {
						Mark::Dot => {
							string.push('.').unwrap();
						}
						Mark::Dash => {
							string.push('-').unwrap();
						}
						Mark::None => {}
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

/// Converts a string to a morse code array
pub fn string_to_codes(string: &String<BUFFER_LENGTH>) -> [Code; BUFFER_LENGTH] {
	let mut codes = [Code::None; BUFFER_LENGTH];

	for (index, mut char) in string.chars().enumerate() {
		char.make_ascii_lowercase();
		codes[index] = Code::char_to_code(char);
	}

	codes
}

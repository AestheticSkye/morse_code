use heapless::{String, Vec};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mark {
	Dot,
	Dash,
	None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Code {
	Letter([Mark; 5]),
	Space,
	Error,
	None,
}

const CODES: [(char, Code); 37] = [
	(' ', Code::Space),
	(
		'a',
		Code::Letter([Mark::Dot, Mark::Dash, Mark::None, Mark::None, Mark::None]),
	),
	(
		'b',
		Code::Letter([Mark::Dash, Mark::Dot, Mark::Dot, Mark::Dot, Mark::None]),
	),
	(
		'c',
		Code::Letter([Mark::Dash, Mark::Dot, Mark::Dash, Mark::Dot, Mark::None]),
	),
	(
		'd',
		Code::Letter([Mark::Dash, Mark::Dot, Mark::Dot, Mark::None, Mark::None]),
	),
	(
		'e',
		Code::Letter([Mark::Dot, Mark::None, Mark::None, Mark::None, Mark::None]),
	),
	(
		'f',
		Code::Letter([Mark::Dot, Mark::Dot, Mark::Dash, Mark::Dot, Mark::None]),
	),
	(
		'g',
		Code::Letter([Mark::Dash, Mark::Dash, Mark::Dot, Mark::None, Mark::None]),
	),
	(
		'h',
		Code::Letter([Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dot, Mark::None]),
	),
	(
		'i',
		Code::Letter([Mark::Dot, Mark::Dot, Mark::None, Mark::None, Mark::None]),
	),
	(
		'j',
		Code::Letter([Mark::Dot, Mark::Dash, Mark::Dash, Mark::Dash, Mark::None]),
	),
	(
		'k',
		Code::Letter([Mark::Dash, Mark::Dot, Mark::Dash, Mark::None, Mark::None]),
	),
	(
		'l',
		Code::Letter([Mark::Dot, Mark::Dash, Mark::Dot, Mark::Dot, Mark::None]),
	),
	(
		'm',
		Code::Letter([Mark::Dash, Mark::Dash, Mark::None, Mark::None, Mark::None]),
	),
	(
		'n',
		Code::Letter([Mark::Dash, Mark::Dot, Mark::None, Mark::None, Mark::None]),
	),
	(
		'o',
		Code::Letter([Mark::Dash, Mark::Dash, Mark::Dash, Mark::None, Mark::None]),
	),
	(
		'p',
		Code::Letter([Mark::Dot, Mark::Dash, Mark::Dash, Mark::Dot, Mark::None]),
	),
	(
		'q',
		Code::Letter([Mark::Dash, Mark::Dash, Mark::Dot, Mark::Dash, Mark::None]),
	),
	(
		'r',
		Code::Letter([Mark::Dot, Mark::Dash, Mark::Dot, Mark::None, Mark::None]),
	),
	(
		's',
		Code::Letter([Mark::Dot, Mark::Dot, Mark::Dot, Mark::None, Mark::None]),
	),
	(
		't',
		Code::Letter([Mark::Dash, Mark::None, Mark::None, Mark::None, Mark::None]),
	),
	(
		'u',
		Code::Letter([Mark::Dot, Mark::Dot, Mark::Dash, Mark::None, Mark::None]),
	),
	(
		'v',
		Code::Letter([Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dash, Mark::None]),
	),
	(
		'w',
		Code::Letter([Mark::Dot, Mark::Dash, Mark::Dash, Mark::None, Mark::None]),
	),
	(
		'x',
		Code::Letter([Mark::Dash, Mark::Dot, Mark::Dot, Mark::Dash, Mark::None]),
	),
	(
		'y',
		Code::Letter([Mark::Dash, Mark::Dot, Mark::Dash, Mark::Dash, Mark::None]),
	),
	(
		'z',
		Code::Letter([Mark::Dash, Mark::Dash, Mark::Dot, Mark::Dot, Mark::None]),
	),
	(
		'1',
		Code::Letter([Mark::Dot, Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dash]),
	),
	(
		'2',
		Code::Letter([Mark::Dot, Mark::Dot, Mark::Dash, Mark::Dash, Mark::Dash]),
	),
	(
		'3',
		Code::Letter([Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dash, Mark::Dash]),
	),
	(
		'4',
		Code::Letter([Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dash]),
	),
	(
		'5',
		Code::Letter([Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dot]),
	),
	(
		'6',
		Code::Letter([Mark::Dash, Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dot]),
	),
	(
		'7',
		Code::Letter([Mark::Dash, Mark::Dash, Mark::Dot, Mark::Dot, Mark::Dot]),
	),
	(
		'8',
		Code::Letter([Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dot, Mark::Dot]),
	),
	(
		'9',
		Code::Letter([Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dot]),
	),
	(
		'0',
		Code::Letter([Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dash]),
	),
];

impl Code {
	/// Converts a morse code to a character
	pub fn to_char(self) -> char {
		if self == Self::Space {
			return ' ';
		}

		let Self::Letter(marks) = self else {
			return '%'
		};

		for code_set in CODES {
			if let Self::Letter(cmp_marks) = code_set.1 {
				if cmp_marks == Vec::<Mark, 5>::from_slice(&marks).unwrap() {
					return code_set.0;
				}
			}
		}

		'%'
	}

	/// Converts a character to a morse code
	pub fn char_to_code(character: char) -> Self {
		for code_set in CODES {
			if code_set.0 == character {
				return code_set.1;
			}
		}
		Self::Error
	}
	/// Converts a morse code array to a string of marks
	pub fn to_marks(self) -> String<20> {
		let mut string = String::<20>::new();

		match self {
			Self::Letter(code) => {
				for mark in code {
					match mark {
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
			Self::Space => {
				string.push(' ').unwrap();
				string.push(' ').unwrap();
			}
			Self::Error => {
				string.push('%').unwrap();
			}
			Self::None => {}
		}

		String::from(string.trim())
	}
}

use heapless::{String, Vec};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mark {
	Dot,
	Dash,
	None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Code {
	Some([Mark; 5]),
	Space,
	Error,
	None,
}

const CODES: [(char, Code); 37] = [
	(' ', Code::Space),
	(
		'a',
		Code::Some([Mark::Dot, Mark::Dash, Mark::None, Mark::None, Mark::None]),
	),
	(
		'b',
		Code::Some([Mark::Dash, Mark::Dot, Mark::Dot, Mark::Dot, Mark::None]),
	),
	(
		'c',
		Code::Some([Mark::Dash, Mark::Dot, Mark::Dash, Mark::Dot, Mark::None]),
	),
	(
		'd',
		Code::Some([Mark::Dash, Mark::Dot, Mark::Dot, Mark::None, Mark::None]),
	),
	(
		'e',
		Code::Some([Mark::Dot, Mark::None, Mark::None, Mark::None, Mark::None]),
	),
	(
		'f',
		Code::Some([Mark::Dot, Mark::Dot, Mark::Dash, Mark::Dot, Mark::None]),
	),
	(
		'g',
		Code::Some([Mark::Dash, Mark::Dash, Mark::Dot, Mark::None, Mark::None]),
	),
	(
		'h',
		Code::Some([Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dot, Mark::None]),
	),
	(
		'i',
		Code::Some([Mark::Dot, Mark::Dot, Mark::None, Mark::None, Mark::None]),
	),
	(
		'j',
		Code::Some([Mark::Dot, Mark::Dash, Mark::Dash, Mark::Dash, Mark::None]),
	),
	(
		'k',
		Code::Some([Mark::Dash, Mark::Dot, Mark::Dash, Mark::None, Mark::None]),
	),
	(
		'l',
		Code::Some([Mark::Dot, Mark::Dash, Mark::Dot, Mark::Dot, Mark::None]),
	),
	(
		'm',
		Code::Some([Mark::Dash, Mark::Dash, Mark::None, Mark::None, Mark::None]),
	),
	(
		'n',
		Code::Some([Mark::Dash, Mark::Dot, Mark::None, Mark::None, Mark::None]),
	),
	(
		'o',
		Code::Some([Mark::Dash, Mark::Dash, Mark::Dash, Mark::None, Mark::None]),
	),
	(
		'p',
		Code::Some([Mark::Dot, Mark::Dash, Mark::Dash, Mark::Dot, Mark::None]),
	),
	(
		'q',
		Code::Some([Mark::Dash, Mark::Dash, Mark::Dot, Mark::Dash, Mark::None]),
	),
	(
		'r',
		Code::Some([Mark::Dot, Mark::Dash, Mark::Dot, Mark::None, Mark::None]),
	),
	(
		's',
		Code::Some([Mark::Dot, Mark::Dot, Mark::Dot, Mark::None, Mark::None]),
	),
	(
		't',
		Code::Some([Mark::Dash, Mark::None, Mark::None, Mark::None, Mark::None]),
	),
	(
		'u',
		Code::Some([Mark::Dot, Mark::Dot, Mark::Dash, Mark::None, Mark::None]),
	),
	(
		'v',
		Code::Some([Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dash, Mark::None]),
	),
	(
		'w',
		Code::Some([Mark::Dot, Mark::Dash, Mark::Dash, Mark::None, Mark::None]),
	),
	(
		'x',
		Code::Some([Mark::Dash, Mark::Dot, Mark::Dot, Mark::Dash, Mark::None]),
	),
	(
		'y',
		Code::Some([Mark::Dash, Mark::Dot, Mark::Dash, Mark::Dash, Mark::None]),
	),
	(
		'z',
		Code::Some([Mark::Dash, Mark::Dash, Mark::Dot, Mark::Dot, Mark::None]),
	),
	(
		'1',
		Code::Some([Mark::Dot, Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dash]),
	),
	(
		'2',
		Code::Some([Mark::Dot, Mark::Dot, Mark::Dash, Mark::Dash, Mark::Dash]),
	),
	(
		'3',
		Code::Some([Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dash, Mark::Dash]),
	),
	(
		'4',
		Code::Some([Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dash]),
	),
	(
		'5',
		Code::Some([Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dot]),
	),
	(
		'6',
		Code::Some([Mark::Dash, Mark::Dot, Mark::Dot, Mark::Dot, Mark::Dot]),
	),
	(
		'7',
		Code::Some([Mark::Dash, Mark::Dash, Mark::Dot, Mark::Dot, Mark::Dot]),
	),
	(
		'8',
		Code::Some([Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dot, Mark::Dot]),
	),
	(
		'9',
		Code::Some([Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dot]),
	),
	(
		'0',
		Code::Some([Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dash, Mark::Dash]),
	),
];

impl Code {
	/// Converts a morse code to a character
	pub fn to_char(self) -> char {
		if self == Self::Space {
			return ' ';
		}

		let Self::Some(marks) = self else {
			return '%'
		};

		for code_set in CODES {
			if let Self::Some(cmp_marks) = code_set.1 {
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
	pub fn to_marks(&self) -> String<5> {
		let mut string = String::<5>::new();

		match self {
			Code::Some(code) => {
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
			Code::Space => {
				string.push(' ').unwrap();
				string.push(' ').unwrap();
			}
			Code::Error => {
				string.push('%').unwrap();
			}
			Code::None => {}
		}

		String::from(string.trim())
	}
}

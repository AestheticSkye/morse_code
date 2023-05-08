use heapless::String;

#[derive(Copy, Clone)]
pub enum Code {
    Some([u8; 5]),
    Space,
    Error,
    None,
}

const CODES: [(char, Code); 36] = [
    ('a', Code::Some([1, 2, 0, 0, 0])),
    ('b', Code::Some([2, 1, 1, 1, 0])),
    ('c', Code::Some([2, 1, 2, 1, 0])),
    ('d', Code::Some([2, 1, 1, 0, 0])),
    ('e', Code::Some([1, 0, 0, 0, 0])),
    ('f', Code::Some([1, 1, 2, 1, 0])),
    ('g', Code::Some([2, 2, 1, 0, 0])),
    ('h', Code::Some([1, 1, 1, 1, 0])),
    ('i', Code::Some([1, 1, 0, 0, 0])),
    ('j', Code::Some([1, 2, 2, 2, 0])),
    ('k', Code::Some([2, 1, 2, 0, 0])),
    ('l', Code::Some([1, 2, 1, 1, 0])),
    ('m', Code::Some([2, 2, 0, 0, 0])),
    ('n', Code::Some([2, 1, 0, 0, 0])),
    ('o', Code::Some([2, 2, 2, 0, 0])),
    ('p', Code::Some([1, 2, 2, 1, 0])),
    ('q', Code::Some([2, 2, 1, 2, 0])),
    ('r', Code::Some([1, 2, 1, 0, 0])),
    ('s', Code::Some([1, 1, 1, 0, 0])),
    ('t', Code::Some([2, 0, 0, 0, 0])),
    ('u', Code::Some([1, 1, 2, 0, 0])),
    ('v', Code::Some([1, 1, 1, 2, 0])),
    ('w', Code::Some([1, 2, 2, 0, 0])),
    ('x', Code::Some([2, 1, 1, 2, 0])),
    ('y', Code::Some([2, 1, 2, 2, 0])),
    ('z', Code::Some([2, 2, 1, 1, 0])),
    ('1', Code::Some([1, 2, 2, 2, 2])),
    ('2', Code::Some([1, 1, 2, 2, 2])),
    ('3', Code::Some([1, 1, 1, 2, 2])),
    ('4', Code::Some([1, 1, 1, 1, 2])),
    ('5', Code::Some([1, 1, 1, 1, 1])),
    ('6', Code::Some([2, 1, 1, 1, 1])),
    ('7', Code::Some([2, 2, 1, 1, 1])),
    ('8', Code::Some([2, 2, 2, 1, 1])),
    ('9', Code::Some([2, 2, 2, 2, 1])),
    ('0', Code::Some([2, 2, 2, 2, 2])),
];

impl Code {
    pub fn code_to_char(code: &[u8; 5]) -> char {
        for code_set in CODES {
            if let Code::Some(code_arr) = code_set.1 {
                if code_arr == *code {
                    return code_set.0;
                }
            }
        }
        '%'
    }

    pub fn char_to_code(character: &char) -> Code {
        if character == &' ' {
            return Code::Space;
        }
        for code_set in CODES {
            if let Code::Some(_) = code_set.1 {
                if code_set.0 == *character {
                    return code_set.1;
                }
            }
        }
        Code::Error
    }
}

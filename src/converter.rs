struct ByteMask {
    mask: &'static str,
    first_code_point: u32,
    last_code_point: u32,
}

const ENCODINGS: [ByteMask; 4] = [
    ByteMask {
        mask: "0XXXXXXX",
        first_code_point: 0x0,
        last_code_point: 0x007F,
    },
    ByteMask {
        mask: "110XXXXX10XXXXXX",
        first_code_point: 0x0080,
        last_code_point: 0x07FF,
    },
    ByteMask {
        mask: "1110XXXX10XXXXXX10XXXXXX",
        first_code_point: 0x0800,
        last_code_point: 0xFFFF,
    },
    ByteMask {
        mask: "11110XXX10XXXXXX10XXXXXX10XXXXXX",
        first_code_point: 0x10000,
        last_code_point: 0x10FFFF,
    },
];

pub fn convert_unicode_to_utf8_bytes(unicode: u32) -> Vec<u8> {
    let number_of_bytes = ENCODINGS
        .iter()
        .position(|mask| unicode >= mask.first_code_point && unicode <= mask.last_code_point)
        .unwrap();
    let base2: Vec<char> = format!("{:b}", unicode).chars().rev().collect();

    let starting_value = ENCODINGS[number_of_bytes as usize].mask;
    let mut base2_index = 0;

    let utf8_as_binary: String = starting_value
        .chars()
        .rev()
        .map(|c| match c {
            'X' => match base2.get(base2_index) {
                Some(v) => {
                    base2_index += 1;
                    *v
                }
                None => '0',
            },
            _ => c,
        })
        .collect();
    let utf8_as_binary = utf8_as_binary.chars().rev().collect::<String>();
    let utf8 = u32::from_str_radix(&utf8_as_binary, 2).unwrap();

    let mut bytes = vec![];

    for i in 1..=number_of_bytes + 1 {
        let byte = (utf8 >> ((number_of_bytes + 1 - i) * 8)) & 0xff;
        bytes.push(byte as u8);
    }

    bytes
}

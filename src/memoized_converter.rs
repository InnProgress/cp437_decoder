use crate::converter;
use std::collections::HashMap;

pub struct MemoizedConverter {
    values: HashMap<u32, Vec<u8>>,
}
impl MemoizedConverter {
    pub fn new() -> MemoizedConverter {
        MemoizedConverter {
            values: HashMap::new(),
        }
    }
    pub fn run(&mut self, unicode: u32) -> Vec<u8> {
        match self.values.get(&unicode) {
            Some(value) => value.to_owned(),
            None => {
                let new_value = converter::convert_unicode_to_utf8_bytes(unicode);
                self.values.insert(unicode, new_value.to_owned());
                new_value
            }
        }
    }
}

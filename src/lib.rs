use memoized_converter::MemoizedConverter;
use std::{collections::HashMap, fs, io::prelude::*, process, str};

pub mod converter;
pub mod memoized_converter;

pub type CodePage = HashMap<u8, u32>;

pub fn load_codepage(file_name: &'static str) -> CodePage {
    let rules_string = fs::read_to_string(file_name).unwrap();
    let mut codepage: CodePage = HashMap::new();
    for line in rules_string.lines() {
        let splitted_line: Vec<&str> = line.split(":").collect();
        if let Ok(v) = splitted_line[0].parse() {
            codepage.insert(v, u32::from_str_radix(splitted_line[1], 16).unwrap());
        }
    }
    codepage
}

pub fn read_file(file_name: &'static str) -> Vec<u8> {
    match fs::read(file_name) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("Problem with reading file {}", file_name);
            process::exit(0);
        }
    }
}

pub fn write_output(file_name: &'static str, output: Vec<u8>) {
    match fs::File::create(file_name) {
        Ok(mut file) => match file.write_all(&output) {
            Ok(_) => println!("File converted into {} file", file_name),
            Err(_) => println!("Problem with writing into {} file", file_name),
        },
        Err(_) => println!("Problem with creating {} file", file_name),
    };
}

pub fn convert_bytes(input_bytes: Vec<u8>, codepage: CodePage) -> Vec<u8> {
    let mut output_bytes: Vec<u8> = vec![];
    let mut converter = MemoizedConverter::new();
    input_bytes.into_iter().for_each(|byte| {
        match codepage.get(&byte) {
            Some(v) => {
                let bytes = converter.run(*v);
                output_bytes.extend_from_slice(&bytes);

                // let v16: Vec<u16> = vec![*v as u16];
                // let string = String::from_utf16(v16.as_slice()).unwrap();
                // let bytes = string.as_bytes();
            }
            None => (),
        };
    });
    output_bytes
}

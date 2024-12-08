use std::fs::File;
use std::io::{self, Read};

pub fn read_file_contents(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}
use std::{fs::File, io::Read};

pub fn read_file(file: &str) -> String {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents
}

pub fn read_file_lines(file: &str) -> Vec<String> {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents.lines().map(|s| s.to_string()).collect()
}

pub fn read_file_tokens(file: &str) -> Vec<String> {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents.split_whitespace().map(|s| s.to_string()).collect()
}

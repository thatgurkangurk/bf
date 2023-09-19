use std::{fs::File, io::Read};

pub fn read_file_contents(file: &str) -> std::string::String {
    let mut file = File::open(file).expect("that file does not exist");
    let mut src = String::new();
    file.read_to_string(&mut src)
        .expect("failed to read the file");

    src
}

use std::{fs::File, io::BufReader};

pub fn read_file(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).expect("Unable to find file");

    BufReader::new(file)
}

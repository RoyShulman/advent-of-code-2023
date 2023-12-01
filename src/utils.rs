use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_lines<P: AsRef<Path>>(path: P) -> std::io::Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

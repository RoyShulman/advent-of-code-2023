use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_lines<P: AsRef<Path>>(path: P) -> impl IntoIterator<Item = String> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().map(|x| x.unwrap())
}

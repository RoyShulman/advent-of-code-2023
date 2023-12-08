use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
    str::FromStr,
};

pub fn read_lines<P: AsRef<Path>>(path: P) -> impl IntoIterator<Item = String> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
        .lines()
        .map(std::result::Result::unwrap)
}

pub fn parse_input_lines<P, T>(path: P) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
    T::Err: Debug,
{
    read_lines(path)
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn parse_input<P, T>(path: P) -> T
where
    P: AsRef<Path>,
    T: FromStr,
    T::Err: Debug,
{
    let mut content = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    content.parse().unwrap()
}

#[cfg(test)]
pub fn get_day_test_input(day: &str) -> PathBuf {
    let mut path = PathBuf::from("input");
    path.push(day);
    path.push("test.txt");
    path
}

#[cfg(test)]
pub fn get_day_extra_test_input(day: &str, extra_test: usize) -> PathBuf {
    let mut path = PathBuf::from("input");
    path.push(day);
    path.push(format!("test_{}.txt", extra_test));
    path
}

pub fn get_day_input(day: &str) -> PathBuf {
    let mut path = PathBuf::from("input");
    path.push(day);
    path.push("actual.txt");
    path
}

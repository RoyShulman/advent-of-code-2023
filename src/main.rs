use utils::{get_day_input, parse_input_lines};

use crate::utils::parse_input;

// mod day1;
// pub mod day2;
// pub mod day3;
// pub mod day4;
pub mod day5;
mod utils;

fn main() {
    let almanac = parse_input(get_day_input("day5"));
    println!("{}", day5::part1(&almanac));
    println!("{}", day5::part2(&almanac));
}

use utils::{get_day_input, parse_input_lines};

use crate::utils::parse_input;

// mod day1;
// pub mod day2;
// pub mod day3;
// pub mod day4;
// pub mod day5;
pub mod day6;
mod utils;

fn main() {
    let races = parse_input(get_day_input("day6"));
    println!("{}", day6::part1(&races));
    println!("{}", day6::part2(&races));
}

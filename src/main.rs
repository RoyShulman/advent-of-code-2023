use utils::{get_day_input, parse_input};

// mod day1;
// pub mod day2;
pub mod day3;
mod utils;

fn main() {
    let engine_lines = parse_input(get_day_input("day3"));

    println!("{}", day3::part1(&engine_lines));
    println!("{}", day3::part2(&engine_lines));
}

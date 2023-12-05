use utils::{get_day_input, parse_input};

// mod day1;
// pub mod day2;
// pub mod day3;
pub mod day4;
mod utils;

fn main() {
    let scratch_cards = parse_input(get_day_input("day4"));
    println!("{}", day4::part1(&scratch_cards));
    println!("{}", day4::part2(&scratch_cards));
}

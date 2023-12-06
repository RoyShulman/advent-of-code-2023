use aoc::utils::{get_day_input, parse_input_lines};

use aoc::day6;
use aoc::utils::parse_input;

fn main() {
    let races = parse_input(get_day_input("day6"));
    println!("{}", day6::part1(&races));
    println!("{}", day6::part2(&races));
}

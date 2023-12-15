use std::fs::read_to_string;

use aoc::utils::{get_day_input, parse_input};

use aoc::day15;

fn main() {
    let input = get_day_input("day15");
    let input = read_to_string(&input).unwrap();
    println!("{}", day15::part1(&input));
    println!("{}", day15::part2(&input));
}

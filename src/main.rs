use std::fs::read_to_string;

use aoc::utils::{get_day_input, parse_input};

use aoc::day16;

fn main() {
    let contraption = parse_input(get_day_input("day16"));
    println!("{}", day16::part1(&contraption));
    println!("{}", day16::part2(&contraption));
}

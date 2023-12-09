use aoc::utils::{get_day_input, parse_input_lines};

use aoc::day9;

fn main() {
    let history = parse_input_lines(get_day_input("day9"));
    println!("{}", day9::part1(&history));
    println!("{}", day9::part2(&history));
}

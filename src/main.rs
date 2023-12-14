use aoc::utils::{get_day_input, parse_input};

use aoc::day13;

fn main() {
    let grid_patterns = parse_input(get_day_input("day13"));
    println!("{}", day13::part1(&grid_patterns));
    println!("{}", day13::part2(&grid_patterns));
}

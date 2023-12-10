use aoc::utils::{get_day_input, parse_input, parse_input_lines};

use aoc::day10;

fn main() {
    let grid = parse_input(get_day_input("day10"));
    println!("{}", day10::part1(&grid));
    // println!("{}", day10::part2(&history));
}

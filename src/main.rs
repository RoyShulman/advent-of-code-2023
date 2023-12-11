use aoc::utils::{get_day_input, parse_input, parse_input_lines};

use aoc::day11;

fn main() {
    let image = parse_input(get_day_input("day11"));
    println!("{}", day11::part1_and_part2(&image));
    // println!("{}", day10::part2(&history));
}

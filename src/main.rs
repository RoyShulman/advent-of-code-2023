use aoc::utils::get_day_input;

use aoc::day8;
use aoc::utils::parse_input;

fn main() {
    let map = parse_input(get_day_input("day8"));
    println!("{}", day8::part1(&map));
    println!("{}", day8::part2(&map));
}

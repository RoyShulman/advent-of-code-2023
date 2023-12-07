use aoc::utils::get_day_input;

use aoc::day7;
use aoc::utils::parse_input;

fn main() {
    let hand_set = parse_input(get_day_input("day7"));
    println!("{}", day7::part2(&hand_set));
}

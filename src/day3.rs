use std::{ops::Range, str::FromStr};

use anyhow::Context;
use itertools::Itertools;

#[derive(Debug)]
pub struct AdjacencyRange {
    inner: Range<usize>,
}

impl AdjacencyRange {
    pub fn new(range: &Range<usize>) -> Self {
        let start = match range.start {
            0 => range.start,
            _ => range.start - 1,
        };
        let end = range.end + 1;
        Self { inner: start..end }
    }

    pub fn contains(&self, index: &usize) -> bool {
        self.inner.contains(index)
    }
}

#[derive(Debug)]
pub struct PossiblePartNumber {
    number: u32,
    location_range: AdjacencyRange,
}

#[derive(Debug)]
pub struct EngineLine {
    possible_part_numbers: Vec<PossiblePartNumber>,
    symbol_indexes: Vec<usize>,
    possible_gears: Vec<usize>,
}

fn parse_possible_part_number(
    s: &str,
    possible_start: &mut Option<usize>,
    current_index: usize,
) -> anyhow::Result<Option<PossiblePartNumber>> {
    let Some(number_start_index) = possible_start.take() else {
        return Ok(None);
    };
    let number_substring = &s[number_start_index..current_index];
    let number = u32::from_str(number_substring)
        .with_context(|| format!("failed to parse string as u32: {number_substring}"))?;

    Ok(Some(PossiblePartNumber {
        number,
        location_range: AdjacencyRange::new(&(number_start_index..current_index)),
    }))
}

impl FromStr for EngineLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbol_indexes = Vec::new();
        let mut possible_part_numbers = Vec::new();
        let mut possible_gears = Vec::new();
        let mut parsing_number_start = None;
        for (index, c) in s.chars().enumerate() {
            if c.is_ascii_digit() {
                if parsing_number_start.is_none() {
                    parsing_number_start = Some(index);
                }
                continue;
            }

            if let Some(possible_part_number) =
                parse_possible_part_number(s, &mut parsing_number_start, index)?
            {
                possible_part_numbers.push(possible_part_number);
            }

            if c == '.' {
                continue;
            }

            symbol_indexes.push(index);

            if c == '*' {
                possible_gears.push(index);
            }
        }

        // if the line ends in a digit we mustn't forget to parse the last number
        if let Some(possible_part_number) =
            parse_possible_part_number(s, &mut parsing_number_start, s.len())?
        {
            possible_part_numbers.push(possible_part_number);
        }

        Ok(Self {
            possible_part_numbers,
            symbol_indexes,
            possible_gears,
        })
    }
}

fn get_part_numbers_sum(part_numbers: &[PossiblePartNumber], symbol_indexes: &[usize]) -> u32 {
    let mut sum = 0;
    for part_number in part_numbers {
        for index in symbol_indexes {
            if part_number.location_range.contains(index) {
                sum += part_number.number;
            }
        }
    }

    sum
}

pub fn part1(engine_lines: &[EngineLine]) -> u32 {
    let mut sum = 0;
    // Iterate same line adjacency
    for line in engine_lines {
        sum += get_part_numbers_sum(&line.possible_part_numbers, &line.symbol_indexes);
    }

    // interline adjacencies

    for (line_above, line_below) in engine_lines.iter().tuple_windows() {
        sum += get_part_numbers_sum(
            &line_above.possible_part_numbers,
            &line_below.symbol_indexes,
        );
        sum += get_part_numbers_sum(
            &line_below.possible_part_numbers,
            &line_above.symbol_indexes,
        );
    }

    sum
}

fn add_adjecent_part_number(
    gear: &usize,
    part_numbers: &[PossiblePartNumber],
    adjecent: &mut Vec<u32>,
) {
    for part_number in part_numbers {
        if part_number.location_range.contains(gear) {
            adjecent.push(part_number.number);
        }
    }
}

fn get_gear_product_sum(
    part_numbers_above: &[PossiblePartNumber],
    part_numbers_current: &[PossiblePartNumber],
    part_numbers_below: &[PossiblePartNumber],
    gears: &[usize],
) -> u32 {
    let mut sum = 0;
    for gear in gears {
        let mut adjecent = Vec::new();
        add_adjecent_part_number(gear, part_numbers_above, &mut adjecent);
        add_adjecent_part_number(gear, part_numbers_current, &mut adjecent);
        add_adjecent_part_number(gear, part_numbers_below, &mut adjecent);

        if adjecent.len() == 2 {
            sum += adjecent.into_iter().product::<u32>();
        }
    }

    sum
}

pub fn part2(engine_lines: &[EngineLine]) -> u32 {
    let mut sum = 0;

    for (line_above, current_line, line_below) in engine_lines.iter().tuple_windows() {
        // this is cheating a bit, but the first and last lines don't have gears. Otherwise we would also have to sum the first and last line of gears by having the above and below lines empty accordingly
        sum += get_gear_product_sum(
            &line_above.possible_part_numbers,
            &current_line.possible_part_numbers,
            &line_below.possible_part_numbers,
            &current_line.possible_gears,
        );
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{get_day_test_input, parse_input};

    #[test]
    fn test_day3_part1() {
        let engine_lines = parse_input(get_day_test_input("day3"));
        assert_eq!(part1(&engine_lines), 4361);
    }

    #[test]
    fn test_day3_part2() {
        let engine_lines = parse_input(get_day_test_input("day3"));
        assert_eq!(part2(&engine_lines), 467835);
    }
}

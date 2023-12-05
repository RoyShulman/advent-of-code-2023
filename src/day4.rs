use std::{collections::HashSet, str::FromStr};

use anyhow::Context;

pub struct ScratchCard {
    chosen: HashSet<u32>,
    winning: HashSet<u32>,
}

impl ScratchCard {
    pub fn get_count_chosen_in_winning(&self) -> usize {
        self.chosen
            .iter()
            .filter(|chosen| self.winning.contains(chosen))
            .count()
    }
}

impl FromStr for ScratchCard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split(": ")
            .last()
            .with_context(|| format!("invalid format: {s}"))?;
        let mut numbers_it = numbers.split("|").map(|numbers_list_str| {
            numbers_list_str
                .split_whitespace()
                .map(|num| {
                    u32::from_str(num).with_context(|| format!("failed to parse as u32: {num}"))
                })
                .collect::<Result<HashSet<_>, _>>()
        });
        let chosen = numbers_it
            .next()
            .with_context(|| format!("missing chosen numbers in: {s}"))??;
        let winning = numbers_it
            .next()
            .with_context(|| format!("missing winning numbers in: {s}"))??;
        Ok(Self { chosen, winning })
    }
}

pub fn part1(scratch_cards: &[ScratchCard]) -> u32 {
    scratch_cards
        .iter()
        .filter_map(|x| match x.get_count_chosen_in_winning() {
            0 => None,
            x => Some(2_u32.pow((x - 1) as u32)),
        })
        .sum()
}

pub fn part2(scratch_cards: &[ScratchCard]) -> u32 {
    let num_winners_in_each_card: Vec<usize> = scratch_cards
        .iter()
        .map(|x| x.get_count_chosen_in_winning())
        .collect();
    // we know they are sorted and there are no skips, and at least one line for each
    let mut num_cards_of_each = vec![1; num_winners_in_each_card.len()];
    for (index, winners) in num_winners_in_each_card.iter().enumerate() {
        for _ in 0..(num_cards_of_each[index] as usize) {
            for current_to_add in index + 1..index + winners + 1 {
                num_cards_of_each[current_to_add] += 1;
            }
        }
    }

    num_cards_of_each.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::utils::{get_day_test_input, parse_input};

    use super::*;

    #[test]
    fn test_part1() {
        let scratch_cards = parse_input(get_day_test_input("day4"));
        assert_eq!(part1(&scratch_cards), 13);
    }

    #[test]
    fn test_part2() {
        let scratch_cards = parse_input(get_day_test_input("day4"));
        assert_eq!(part2(&scratch_cards), 30);
    }
}

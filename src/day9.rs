use std::str::FromStr;

use anyhow::Context;
use itertools::Itertools;

pub struct History {
    values: Vec<i32>,
}

impl FromStr for History {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: anyhow::Result<_> = s
            .split_whitespace()
            .map(|x| {
                x.parse()
                    .with_context(|| format!("failed to parse {x} as u32"))
            })
            .collect();

        Ok(Self {
            values: result.context("failed to parse history")?,
        })
    }
}

fn get_diffs(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .tuple_windows()
        .map(|(before, after)| after - before)
        .collect()
}

impl History {
    fn get_all_intermidiate_results(&self) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        results.push(self.values.clone());

        let mut diffs = get_diffs(&self.values);
        while !diffs.iter().all(|x| *x == 0) {
            results.push(diffs.clone()); // todo: avoid clone
            diffs = get_diffs(&diffs);
        }

        results
    }

    pub fn extrapolate_last_value(&self) -> anyhow::Result<i32> {
        let results = self.get_all_intermidiate_results();
        anyhow::ensure!(results.iter().all(|x| x.len() > 0));

        Ok(results.into_iter().rev().fold(0, |previous_diff, current| {
            previous_diff
                + current
                    .last()
                    .expect("we checked before every vec contains at least one value")
        }))
    }

    pub fn extrapolate_first_value(&self) -> anyhow::Result<i32> {
        let results = self.get_all_intermidiate_results();
        anyhow::ensure!(results.iter().all(|x| x.len() > 0));

        Ok(results.into_iter().rev().fold(0, |previous_diff, current| {
            current
                .first()
                .expect("we checked before every vec contains at least one value")
                - previous_diff
        }))
    }
}

pub fn part1(history: &[History]) -> i32 {
    history
        .iter()
        .map(|x| x.extrapolate_last_value().unwrap())
        .sum()
}

pub fn part2(history: &[History]) -> i32 {
    history
        .iter()
        .map(|x| x.extrapolate_first_value().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::utils::{get_day_test_input, parse_input_lines};

    use super::*;

    #[test]
    fn test_part1() {
        let history = parse_input_lines(get_day_test_input("day9"));
        assert_eq!(part1(&history), 114);
    }

    #[test]
    fn test_part2() {
        let history = parse_input_lines(get_day_test_input("day9"));
        assert_eq!(part2(&history), 2);
    }
}

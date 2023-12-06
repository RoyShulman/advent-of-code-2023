use std::str::FromStr;

use anyhow::Context;
use itertools::Itertools;

#[derive(Debug)]
pub struct Race {
    race_time: u64,
    record_distance: u64,
}

impl Race {
    fn num_ways_to_win_brute_force(&self) -> Option<u64> {
        let mut num_ways = None;
        for hold_time in 1..self.race_time {
            let distance = hold_time * (self.race_time - hold_time);
            if distance > self.record_distance {
                num_ways = Some(num_ways.unwrap_or(0) + 1);
            }
        }

        num_ways
    }

    fn num_ways_to_win(&self) -> Option<u64> {
        // S < (t-w)*w
        // w - wait time, t - race time, s = record time
        // w**2 - t*w + s < 0

        // maybe losses precision? think about this
        let t = -(self.race_time as f64);
        let s = self.record_distance as f64;

        let discriminant = t.powi(2) - 4. * s;
        if discriminant < 0. {
            return None;
        }

        let sqrt = discriminant.sqrt();
        let first_root = (-t + sqrt) / 2.;
        let second_root = (-t - sqrt) / 2.;

        let num_integers_between = first_root.ceil() as u64 - (second_root.floor() as u64 + 1);
        Some(num_integers_between)
    }
}

#[derive(Debug)]
pub struct Races {
    races: Vec<Race>,
    single_race: Race,
}

fn day6_line_to_u64_vec(line: &str) -> anyhow::Result<Vec<u64>> {
    let numbers = line
        .split(":")
        .skip(1)
        .next()
        .context("missing the actual numbers")?;
    numbers
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse::<u64>().context("fails to parse number"))
        .collect()
}

fn day6_parse_part2_single_race(line: &str) -> anyhow::Result<u64> {
    let numbers = line
        .split(":")
        .skip(1)
        .next()
        .context("missing the actual numbers")?;
    let single_number = numbers.split_whitespace().join("");
    single_number
        .parse()
        .context("failed to parse single number")
}

impl FromStr for Races {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Time:      7  15   30
        // Distance:  9  40  200

        let mut lines = s.lines();
        let line = lines.next().context("missing times line")?;

        let times = day6_line_to_u64_vec(line).context("failed to parse times")?;
        let single_race_time =
            day6_parse_part2_single_race(line).context("failed to parse single time race")?;

        let line = lines.next().context("missing distances line")?;
        let distances = day6_line_to_u64_vec(line).context("failed to parse distances line")?;
        let single_race_distance =
            day6_parse_part2_single_race(line).context("failed t parse single race distance")?;

        if times.len() != distances.len() {
            anyhow::bail!("times and distances length differ");
        }

        let races = times
            .into_iter()
            .zip(distances.into_iter())
            .map(|(time, distance)| Race {
                race_time: time,
                record_distance: distance,
            })
            .collect();

        Ok(Self {
            races,
            single_race: Race {
                race_time: single_race_time,
                record_distance: single_race_distance,
            },
        })
    }
}

impl Races {
    fn product_of_num_ways_to_win(&self) -> u64 {
        self.races
            .iter()
            .filter_map(|x| x.num_ways_to_win())
            .product()
    }

    fn single_race_ways_to_win(&self) -> u64 {
        self.single_race.num_ways_to_win().unwrap_or(0)
    }
}

pub fn part1(races: &Races) -> u64 {
    races.product_of_num_ways_to_win()
}

pub fn part2(races: &Races) -> u64 {
    races.single_race_ways_to_win()
}

#[cfg(test)]
mod tests {

    use crate::utils::{get_day_test_input, parse_input};

    use super::*;

    #[test]
    fn test_part1() {
        let races = parse_input(get_day_test_input("day6"));
        assert_eq!(part1(&races), 288);
    }

    #[test]
    fn test_part2() {
        let races = parse_input(get_day_test_input("day6"));
        assert_eq!(part2(&races), 71503);
    }
}

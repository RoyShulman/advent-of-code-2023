use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
    str::{FromStr, Lines},
};

use anyhow::{Context, Ok};
use itertools::Itertools;

#[derive(Debug)]
struct SeedConversionLine {
    source: Range<u64>,
    destination: Range<u64>,
}

impl SeedConversionLine {}

fn parse_whitespace_seperated_numbers(s: &str) -> anyhow::Result<Vec<u64>> {
    s.split_whitespace()
        .into_iter()
        .map(|x| {
            x.parse()
                .with_context(|| format!("failed to parse as u64: {x}"))
        })
        .collect()
}

impl FromStr for SeedConversionLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 50 98 2
        let numbers = parse_whitespace_seperated_numbers(s)?;

        let mut it = numbers.into_iter();
        let destination = it.next().context("no destination")?;
        let source = it.next().context("no source")?;
        let length = it.next().context("no length")?;

        Ok(SeedConversionLine {
            source: source..source + length,
            destination: destination..destination + length,
        })
    }
}

fn parse_map_block(lines: &mut Lines) -> anyhow::Result<Vec<SeedConversionLine>> {
    lines.next().context("no title line found")?; // skip the first title line
    let mut line = lines.next().context("no first line found")?;
    let mut conversions = Vec::new();
    while !line.trim().is_empty() {
        conversions.push(
            line.parse()
                .with_context(|| format!("failed to parse map block line: {line}"))?,
        );
        line = match lines.next() {
            Some(line) => line,
            None => break, // end of input
        }
    }

    Ok(conversions)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MappingType {
    Seed,
    Soil,
    Fertilizier,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct SeedConversion {
    // maybe btreemap?
    mappings: Vec<SeedConversionLine>,
}

impl SeedConversion {
    pub fn get_dest_number(&self, source_num: u64) -> u64 {
        for mapping in &self.mappings {
            if mapping.source.contains(&source_num) {
                let diff_from_start = source_num - mapping.source.start;
                return mapping.destination.start + diff_from_start;
            }
        }

        // If no mapping, it's means it's 1 to 1
        source_num
    }
}

impl From<Vec<SeedConversionLine>> for SeedConversion {
    fn from(value: Vec<SeedConversionLine>) -> Self {
        Self { mappings: value }
    }
}

#[derive(Debug)]
pub struct MappingTo {
    conversion: SeedConversion,
    to: MappingType,
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    mappings: HashMap<MappingType, MappingTo>,
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // first line is the seeds
        let mut lines = s.lines().into_iter();
        let seeds = lines
            .next()
            .context("no seeds line")?
            .split(':')
            .skip(1)
            .next()
            .context("missing numbers part of seeds")?;
        let seeds = parse_whitespace_seperated_numbers(seeds)?;
        lines.next(); // skip the next blank line

        let seed_to_soil = parse_map_block(&mut lines).context("failed to parse seed to soil")?;
        let seed_to_fertilizer =
            parse_map_block(&mut lines).context("failed to parse seed_to_fertilizer")?;
        let fertilizer_to_water =
            parse_map_block(&mut lines).context("failed to parse parse_map_block")?;
        let water_to_light =
            parse_map_block(&mut lines).context("failed to parse water_to_light")?;
        let light_to_temperature =
            parse_map_block(&mut lines).context("failed to parse light_to_temperature")?;
        let temperature_to_humidity =
            parse_map_block(&mut lines).context("failed to parse temperature_to_humidity")?;
        let humidity_to_location =
            parse_map_block(&mut lines).context("failed to parse humidity_to_location")?;

        let mut mappings = HashMap::new();

        mappings.insert(
            MappingType::Seed,
            MappingTo {
                conversion: seed_to_soil.into(),
                to: MappingType::Soil,
            },
        );
        mappings.insert(
            MappingType::Soil,
            MappingTo {
                conversion: seed_to_fertilizer.into(),
                to: MappingType::Fertilizier,
            },
        );

        mappings.insert(
            MappingType::Fertilizier,
            MappingTo {
                conversion: fertilizer_to_water.into(),
                to: MappingType::Water,
            },
        );

        mappings.insert(
            MappingType::Water,
            MappingTo {
                conversion: water_to_light.into(),
                to: MappingType::Light,
            },
        );

        mappings.insert(
            MappingType::Light,
            MappingTo {
                conversion: light_to_temperature.into(),
                to: MappingType::Temperature,
            },
        );

        mappings.insert(
            MappingType::Temperature,
            MappingTo {
                conversion: temperature_to_humidity.into(),
                to: MappingType::Humidity,
            },
        );

        mappings.insert(
            MappingType::Humidity,
            MappingTo {
                conversion: humidity_to_location.into(),
                to: MappingType::Location,
            },
        );

        Ok(Self { seeds, mappings })
    }
}

impl Almanac {
    fn follow_mapping_from_util(
        &self,
        source_type: MappingType,
        destination_type: MappingType,
        number: u64,
    ) -> anyhow::Result<u64> {
        if source_type == destination_type {
            return Ok(number);
        }

        let mut current_mapping = self.mappings.get(&source_type).context("no source found")?;
        let mut current_number = current_mapping.conversion.get_dest_number(number);

        // pray for no infinite loop
        while current_mapping.to != destination_type {
            current_mapping = self
                .mappings
                .get(&current_mapping.to)
                .context("failed lookup in chain")?;
            current_number = current_mapping.conversion.get_dest_number(current_number);
        }

        Ok(current_number)
    }

    fn get_location_for_seeds(&self) -> anyhow::Result<Vec<u64>> {
        let mut locations = Vec::new();
        for seed in &self.seeds {
            locations.push(
                self.follow_mapping_from_util(MappingType::Seed, MappingType::Location, *seed)
                    .context("failed to follow mapping for seed")?,
            );
        }

        Ok(locations)
    }

    pub fn get_location_for_seed_pairs(&self) -> anyhow::Result<Vec<u64>> {
        let mut locations = Vec::new();
        let seed_tuples = self.seeds.iter().tuples();

        for (seed_start, length) in seed_tuples {
            for seed in *seed_start..(seed_start + length) {
                locations.push(
                    self.follow_mapping_from_util(MappingType::Seed, MappingType::Location, seed)
                        .context("failed to follow mapping for seed")?,
                );
            }
        }
        Ok(locations)
    }
}

pub fn part1(almanac: &Almanac) -> u64 {
    almanac
        .get_location_for_seeds()
        .unwrap()
        .into_iter()
        .min()
        .unwrap()
}

pub fn part2(almanac: &Almanac) -> u64 {
    almanac
        .get_location_for_seed_pairs()
        .unwrap()
        .into_iter()
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::utils::{get_day_test_input, parse_input};

    use super::*;

    #[test]
    fn test_part1() {
        let almanac = parse_input(get_day_test_input("day5"));
        assert_eq!(part1(&almanac), 35);
    }

    #[test]
    fn test_part2() {
        let almanac = parse_input(get_day_test_input("day5"));
        assert_eq!(part2(&almanac), 46);
    }
}

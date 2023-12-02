use std::{collections::HashMap, path::Path, str::FromStr};

use anyhow::Context;

use crate::utils::read_lines;

struct Game {
    id: u32,
    revealed_cubes_list: RevealedCubesList,
}

impl Game {
    pub fn is_game_posssible(&self, constraints: &HashMap<Color, u32>) -> bool {
        // A game is possibe if no revealed cubes are above the constraints
        for subset in self.revealed_cubes_list.iter() {
            for (color, constraint_count) in constraints.iter() {
                match subset.colors_count.get(&color) {
                    Some(count) => {
                        if *constraint_count < *count {
                            return false;
                        }
                    }
                    None => continue,
                }
            }
        }

        true
    }

    pub fn get_fewest_for_all_color(&self) -> HashMap<Color, u32> {
        let mut fewest_for_color: HashMap<Color, u32> = HashMap::new();

        for subset in self.revealed_cubes_list.iter() {
            for (color, count) in subset.colors_count.iter() {
                fewest_for_color
                    .entry(*color)
                    .and_modify(|x| *x = *count.max(x))
                    .or_insert(*count);
            }
        }

        fewest_for_color
    }
}

struct RevealedCubesList {
    revealed_cubes: Vec<RevealedCubes>,
}

impl RevealedCubesList {
    pub fn iter(&self) -> std::slice::Iter<'_, RevealedCubes> {
        self.revealed_cubes.iter()
    }
}

struct RevealedCubes {
    pub colors_count: HashMap<Color, u32>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => anyhow::bail!("unknown color: {}", s),
        };

        Ok(color)
    }
}

impl RevealedCubes {
    pub fn new() -> Self {
        Self {
            colors_count: HashMap::new(),
        }
    }
}

// We don't care about matching on the specific error, and only really care about displaying a trace so using anyhow is good

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let mut it = s.split(":");
        let game = it
            .next()
            .with_context(|| format!("missing first part before : for {}", s))?;
        let id = parse_game_id(game).context("failed to parse game id")?;
        let revealed_cubes_list = it
            .next()
            .with_context(|| format!("missing revealed part in: {}", s))?
            .parse()
            .context("failed to parse revealed cubes list")?;

        Ok(Game {
            id,
            revealed_cubes_list,
        })
    }
}

impl FromStr for RevealedCubesList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let revealed_cubes: anyhow::Result<Vec<RevealedCubes>> = s
            .split(";")
            .into_iter()
            .map(|x| {
                x.parse()
                    .with_context(|| format!("failed to parse to revealed cubes: {}", x))
            })
            .collect();

        Ok(Self {
            revealed_cubes: revealed_cubes?,
        })
    }
}

impl FromStr for RevealedCubes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 3 blue, 4 red
        let it = s.split(",");
        let mut revelead_cubes = Self::new();

        for single_cube_str in it {
            let (color, count) = parse_single_cube_str(single_cube_str)?;

            revelead_cubes.colors_count.insert(color, count);
        }

        Ok(revelead_cubes)
    }
}

fn parse_single_cube_str(single_cube_str: &str) -> anyhow::Result<(Color, u32)> {
    let mut single_cube_it = single_cube_str.split_whitespace();
    let count = single_cube_it
        .next()
        .with_context(|| format!("missing num part in: {}", single_cube_str))?;
    let color = single_cube_it
        .next()
        .with_context(|| format!("missing color part in: {}", single_cube_str))?;

    let count = u32::from_str(count)
        .with_context(|| format!("failed to parse for cube count: {}", count))?;

    let color = color.parse()?;
    Ok((color, count))
}

fn parse_game_id(s: &str) -> anyhow::Result<u32> {
    let str_id = s
        .split_whitespace()
        .skip(1)
        .next()
        .with_context(|| format!("invalid input to parse game id: {}", s))?;
    u32::from_str(str_id).with_context(|| format!("failed to parse to u32: {}", str_id))
}

fn get_games(path: &Path) -> Vec<Game> {
    let games: anyhow::Result<Vec<Game>> = read_lines(path)
        .into_iter()
        .map(|line| line.parse())
        .collect();

    games.unwrap()
}

pub fn day2_part1<P: AsRef<Path>>(path: P) -> u32 {
    let games = get_games(path.as_ref());
    let constraints: HashMap<Color, u32> =
        HashMap::from_iter([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    games
        .iter()
        .filter_map(|game| match game.is_game_posssible(&constraints) {
            true => Some(game.id),
            false => None,
        })
        .sum()
}

pub fn day2_part2<P: AsRef<Path>>(path: P) -> u32 {
    let games = get_games(path.as_ref());
    games
        .into_iter()
        .map(|x| x.get_fewest_for_all_color().into_values().product::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        let path = "input/day2/test.txt";
        assert_eq!(day2_part1(path), 8);
    }

    #[test]
    fn test_day2_part2() {
        let path = "input/day2/test.txt";
        assert_eq!(day2_part2(path), 2286);
    }
}

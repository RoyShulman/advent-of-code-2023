use std::{collections::HashSet, str::FromStr};

use anyhow::Context;

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct GrindIndex {
    x: usize,
    y: usize,
}

impl GrindIndex {
    pub fn north(&self) -> Option<Self> {
        if self.y == 0 {
            return None;
        }

        Some(Self {
            x: self.x,
            y: self.y - 1,
        })
    }

    pub fn south(&self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y + 1,
        })
    }

    pub fn east(&self) -> Option<Self> {
        Some(Self {
            x: self.x + 1,
            y: self.y,
        })
    }

    pub fn west(&self) -> Option<Self> {
        if self.x == 0 {
            return None;
        }

        Some(Self {
            x: self.x - 1,
            y: self.y,
        })
    }
}

impl Tile {
    pub fn get_possible_next(&self, current: &GrindIndex) -> Vec<GrindIndex> {
        let options = match self {
            Tile::Vertical => vec![current.north(), current.south()],
            Tile::Horizontal => vec![current.east(), current.west()],
            Tile::NorthEast => vec![current.north(), current.east()],
            Tile::NorthWest => vec![current.north(), current.west()],
            Tile::SouthWest => vec![current.south(), current.west()],
            Tile::SouthEast => vec![current.south(), current.east()],
            Tile::Ground => vec![],
            Tile::Start => vec![
                current.north(),
                current.south(),
                current.east(),
                current.west(),
            ],
        };

        options.into_iter().filter_map(|x| x).collect()
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let result = match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => anyhow::bail!("invalid value for tile {value}"),
        };
        Ok(result)
    }
}

#[derive(Debug)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn get_start(&self) -> Option<GrindIndex> {
        for (y, line) in self.tiles.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                match tile {
                    Tile::Start => return Some(GrindIndex { x, y }),
                    _ => continue,
                }
            }
        }

        None
    }

    fn get_loop_length(&self, start: GrindIndex) -> Option<u32> {
        let mut stack = Vec::new();
        let mut discovered = HashSet::new();
        let mut first = true;

        stack.push((None, start, 0));
        while !stack.is_empty() {
            let (prev, current, depth) = stack.pop().expect("we iterate while it's not empty");
            let Some(tile) = self.get_tile(&current) else {
                continue;
            };
            if first {
                first = false
            } else {
                if tile == &Tile::Start {
                    return Some(depth / 2);
                }
            }

            if discovered.contains(&current) {
                continue;
            }

            for possible_next in tile.get_possible_next(&current) {
                if let Some(prev) = prev {
                    if prev != possible_next {
                        stack.push((Some(current), possible_next, depth + 1));
                    }
                } else {
                    stack.push((Some(current), possible_next, depth + 1));
                }
            }
            discovered.insert(current);
        }

        None
    }

    fn get_tile(&self, index: &GrindIndex) -> Option<&Tile> {
        self.tiles.get(index.y).map(|line| line.get(index.x))?
    }

    pub fn get_num_furthest_from_start(&self) -> Option<u32> {
        let start = self.get_start()?;
        self.get_loop_length(start)
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: anyhow::Result<Vec<Vec<Tile>>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(|c| c.try_into().context("failed to parse char as tile"))
                    .collect()
            })
            .collect();

        Ok(Self {
            tiles: tiles.context("failed to parse grid")?,
        })
    }
}

pub fn part1(grid: &Grid) -> u32 {
    grid.get_num_furthest_from_start().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::utils::{get_day_test_input, parse_input};

    use super::*;

    #[test]
    fn test_part1() {
        let grid = parse_input(get_day_test_input("day10"));
        assert_eq!(part1(&grid), 8);
    }
}

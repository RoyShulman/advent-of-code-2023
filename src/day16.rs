use std::{collections::HashSet, str::FromStr};

use anyhow::Context;

#[derive(Debug, PartialEq, Eq)]
pub enum GridElement {
    EmptySpace,
    LeftToRightMirror,
    RightToLeftMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl TryFrom<char> for GridElement {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::EmptySpace),
            '/' => Ok(Self::LeftToRightMirror),
            '\\' => Ok(Self::RightToLeftMirror),
            '|' => Ok(Self::VerticalSplitter),
            '-' => Ok(Self::HorizontalSplitter),
            c => anyhow::bail!("invalid value for grid element: {c}"),
        }
    }
}

impl std::fmt::Display for GridElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            GridElement::EmptySpace => '.',
            GridElement::LeftToRightMirror => '/',
            GridElement::RightToLeftMirror => '\\',
            GridElement::VerticalSplitter => '|',
            GridElement::HorizontalSplitter => '-',
        };
        write!(f, "{}", c)
    }
}

impl GridElement {
    fn get_next_direction(&self, direction: Direction) -> (Direction, Option<Direction>) {
        match self {
            GridElement::EmptySpace => (direction, None),
            GridElement::LeftToRightMirror => {
                let next_direction = match direction {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::East => Direction::North,
                    Direction::West => Direction::South,
                };
                (next_direction, None)
            }
            GridElement::RightToLeftMirror => {
                let next_direction = match direction {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::East => Direction::South,
                    Direction::West => Direction::North,
                };
                (next_direction, None)
            }
            GridElement::VerticalSplitter => match direction {
                Direction::North => (direction, None),
                Direction::South => (direction, None),
                Direction::East => (Direction::North, Some(Direction::South)),
                Direction::West => (Direction::North, Some(Direction::South)),
            },
            GridElement::HorizontalSplitter => match direction {
                Direction::North => (Direction::East, Some(Direction::West)),
                Direction::South => (Direction::East, Some(Direction::West)),
                Direction::East => (direction, None),
                Direction::West => (direction, None),
            },
        }
    }
}

pub struct Contraption {
    grid: Vec<Vec<GridElement>>,
}

impl FromStr for Contraption {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        for line in s.lines() {
            let grid_line: anyhow::Result<Vec<GridElement>> = line
                .chars()
                .into_iter()
                .map(|x| x.try_into().context("failed to parse grid element"))
                .collect();
            grid.push(grid_line.context("failed to parse grid line")?)
        }

        Ok(Self { grid })
    }
}

impl Contraption {
    fn get(&self, index: (usize, usize)) -> Option<&GridElement> {
        self.grid.get(index.1).map(|line| line.get(index.0))?
    }

    #[allow(dead_code)]
    fn draw_energized(&self, energized: &HashSet<(usize, usize)>) {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if energized.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("")
        }
    }

    fn num_rows(&self) -> usize {
        self.grid.len()
    }

    fn num_columns(&self) -> usize {
        self.grid.get(0).map(|x| x.len()).unwrap_or(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct MovingBeam {
    current: (usize, usize),
    direction: Direction,
}

impl MovingBeam {
    fn get_next_location<'a>(
        &mut self,
        contraption: &'a Contraption,
    ) -> (Option<(usize, usize)>, Option<Self>) {
        self.current = match self.direction {
            Direction::North => {
                if self.current.1 == 0 {
                    return (None, None);
                }
                (self.current.0, self.current.1 - 1)
            }
            Direction::South => (self.current.0, self.current.1 + 1),
            Direction::East => (self.current.0 + 1, self.current.1),
            Direction::West => {
                if self.current.0 == 0 {
                    return (None, None);
                }
                (self.current.0 - 1, self.current.1)
            }
        };

        let Some(element) = contraption.get(self.current) else {
            return (None, None);
        };

        let (next_direction, split_beam_direction) = element.get_next_direction(self.direction);

        self.direction = next_direction;
        let next_beam = match split_beam_direction {
            Some(direction) => Some(Self {
                current: self.current,
                direction,
            }),
            None => None,
        };

        (Some(self.current), next_beam)
    }
}

struct Beams<'a> {
    contraption: &'a Contraption,
    beams: Vec<MovingBeam>,
    energized: HashSet<(usize, usize)>,
    previous_steps: HashSet<MovingBeam>,
}

impl<'a> Beams<'a> {
    fn new(contraption: &'a Contraption) -> Self {
        let current = (0, 0);
        let element = contraption.get(current).expect("must start at (0,0)");
        let (direction, next_beam) = element.get_next_direction(Direction::East);
        assert!(next_beam.is_none()); // pls no

        Self {
            contraption,
            beams: vec![MovingBeam { current, direction }],
            energized: HashSet::from_iter([(0, 0)]),
            previous_steps: HashSet::from_iter([MovingBeam { current, direction }]),
        }
    }

    fn with_start_beam(
        contraption: &'a Contraption,
        start_beam: MovingBeam,
    ) -> anyhow::Result<Self> {
        let start_index = start_beam.current;
        let element = contraption
            .get(start_index)
            .with_context(|| format!("invalid start index: {:?}", start_beam))?;
        let (direction, next_beam) = element.get_next_direction(start_beam.direction);

        let start_beam = MovingBeam {
            current: start_beam.current,
            direction,
        };

        let energized = HashSet::from_iter([start_index]);
        let mut previous_steps = HashSet::from_iter([start_beam]);
        let mut beams = vec![start_beam];

        if let Some(direction) = next_beam {
            let next_beam = MovingBeam {
                current: start_index,
                direction,
            };

            beams.push(next_beam);
            previous_steps.insert(next_beam);
        }

        Ok(Self {
            contraption,
            beams,
            energized,
            previous_steps,
        })
    }

    fn next_bounce(&mut self) -> bool {
        let mut beams_to_add = Vec::new();
        let mut locations_to_add = HashSet::new();
        self.beams.retain_mut(|beam| {
            let (next_location, extra_beam) = beam.get_next_location(self.contraption);
            if let Some(extra_beam) = extra_beam {
                beams_to_add.push(extra_beam);
            };

            match next_location {
                Some(location) => {
                    if self.previous_steps.contains(&beam) {
                        return false;
                    }

                    locations_to_add.insert(location);
                    self.previous_steps.insert(*beam);
                    true
                }
                None => false,
            }
        });

        // eprintln!("{:?}", self.beams);
        // std::thread::sleep(std::time::Duration::from_millis(500));
        self.energized.extend(locations_to_add);
        self.beams.extend(beams_to_add);

        !self.beams.is_empty()
    }
}

pub fn part1(contraption: &Contraption) -> usize {
    let mut beams = Beams::new(contraption);
    while beams.next_bounce() {
        // contraption.draw_energized(&beams.energized);
        // println!("");
    }
    beams.energized.len()
}

fn get_num_energized(beams: &mut Beams<'_>) -> usize {
    while beams.next_bounce() {}
    beams.energized.len()
}

pub fn part2(contraption: &Contraption) -> usize {
    let mut energized = 0;

    let mut start_beams = Vec::new();
    for y in 0..contraption.num_rows() {
        start_beams.push(MovingBeam {
            current: (0, y),
            direction: Direction::East,
        });

        start_beams.push(MovingBeam {
            current: (contraption.num_columns() - 1, y),
            direction: Direction::West,
        });
    }

    for x in 0..contraption.num_columns() {
        start_beams.push(MovingBeam {
            current: (x, 0),
            direction: Direction::South,
        });

        start_beams.push(MovingBeam {
            current: (x, contraption.num_rows() - 1),
            direction: Direction::North,
        });
    }

    for start_beam in start_beams {
        let mut beams = Beams::with_start_beam(contraption, start_beam).unwrap();

        energized = energized.max(get_num_energized(&mut beams));
    }

    energized
}

#[cfg(test)]
mod tests {
    use crate::utils::{get_day_test_input, parse_input};

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(get_day_test_input("day16"));
        assert_eq!(part1(&input), 46);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(get_day_test_input("day16"));
        assert_eq!(part2(&input), 51);
    }
}

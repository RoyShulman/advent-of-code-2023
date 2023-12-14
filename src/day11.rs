use std::{iter::FusedIterator, str::FromStr};

use anyhow::Context;

#[derive(Debug)]
pub struct DriftedGridIndex {
    x: usize,
    y: usize,
}

impl DriftedGridIndex {
    fn distance(&self, other: &DriftedGridIndex) -> u64 {
        let self_x = self.x as i64;
        let self_y = self.y as i64;
        let other_x = other.x as i64;
        let other_y = other.y as i64;
        (self_x - other_x).unsigned_abs() + (self_y - other_y).unsigned_abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageData {
    Galaxy,
    Empty,
}

impl TryFrom<char> for ImageData {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Galaxy),
            '.' => Ok(Self::Empty),
            _ => anyhow::bail!("invalid value for image data: {value}"),
        }
    }
}

#[derive(Debug)]
pub struct Image {
    // The location after taking into account the empty rows and columns
    drifted_galaxies: Vec<DriftedGridIndex>,
    // empty_rows: Vec<usize>,
}

struct Grid {
    inner: Vec<ImageData>,
    num_rows: usize,
    num_columns: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: anyhow::Result<Vec<ImageData>> = s
            .lines()
            .into_iter()
            .flat_map(|line| {
                line.chars()
                    .into_iter()
                    .map(|c| c.try_into().context("failed to parse image data"))
            })
            .collect();

        let num_rows = s.lines().count();
        let first_row = s.lines().next();
        let num_columns = match first_row {
            Some(row) => row.len(),
            None => 0,
        };

        Ok(Self {
            inner: grid.context("failed to parse grid")?,
            num_rows,
            num_columns,
        })
    }
}

impl Grid {
    fn get(&self, row: usize, column: usize) -> Option<&ImageData> {
        let index = row * self.num_rows + column;
        self.inner.get(index)
    }

    fn iter_columns(&self) -> ColumnIterator<'_> {
        ColumnIterator {
            grid: self,
            current_column: 0,
        }
    }

    fn iter_rows(&self) -> RowIterator<'_> {
        RowIterator {
            grid: self,
            current_row: 0,
        }
    }
}

struct ColumnIterator<'a> {
    grid: &'a Grid,
    current_column: usize,
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = Vec<ImageData>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_column > self.grid.num_columns {
            return None;
        }

        let mut column = Vec::with_capacity(self.grid.num_rows);

        for row in 0..self.grid.num_rows {
            column.push(*self.grid.get(row, self.current_column)?);
        }

        self.current_column += 1;

        Some(column)
    }
}

struct RowIterator<'a> {
    grid: &'a Grid,
    current_row: usize,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = Vec<ImageData>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row > self.grid.num_rows {
            return None;
        }

        let mut row = Vec::with_capacity(self.grid.num_rows);

        for column in 0..self.grid.num_columns {
            row.push(*self.grid.get(self.current_row, column)?);
        }

        self.current_row += 1;

        Some(row)
    }
}

const EMPTY_SIZE: usize = 1_000_000;

impl<'a> FusedIterator for ColumnIterator<'a> {}

impl FromStr for Image {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_empty_rows = 0;
        let mut empty_columns = Vec::new();
        let mut drifted_galaxies = Vec::new();

        let grid: Grid = s.parse().context("failed to parse grid")?;

        for (column_index, column) in grid.iter_columns().enumerate() {
            if column.into_iter().all(|x| x == ImageData::Empty) {
                empty_columns.push(column_index);
            }
        }

        for (row_index, row) in grid.iter_rows().enumerate() {
            if row.iter().all(|&x| x == ImageData::Empty) {
                num_empty_rows += 1;
                continue;
            }

            for (column_index, data) in row.into_iter().enumerate() {
                let column_after_drift = match data {
                    ImageData::Galaxy => {
                        let column = column_index
                            + (empty_columns
                                .iter()
                                .filter(|&&empty_column_index| column_index > empty_column_index)
                                .count()
                                * EMPTY_SIZE);
                        column
                            - empty_columns
                                .iter()
                                .filter(|&&empty_column_index| column_index > empty_column_index)
                                .count()
                    }
                    ImageData::Empty => continue,
                };

                drifted_galaxies.push(DriftedGridIndex {
                    x: column_after_drift,
                    y: row_index + (num_empty_rows * EMPTY_SIZE) - num_empty_rows,
                })
            }
        }

        Ok(Self { drifted_galaxies })
    }
}

impl Image {
    fn get_shortest_path_between_all_pairs(&self) -> Vec<u64> {
        let num_pairs = (self.drifted_galaxies.len() * (self.drifted_galaxies.len() + 1)) / 2;
        let mut distances = Vec::with_capacity(num_pairs);
        for (index, side_a) in self.drifted_galaxies.iter().enumerate() {
            for side_b in self.drifted_galaxies.iter().skip(index + 1) {
                distances.push(side_a.distance(side_b));
            }
        }

        distances
    }
}

// TODO: find a way to split part1 and part2, because they are kind of the same just different EMPTY_SIZE
pub fn part1_and_part2(image: &Image) -> u64 {
    image
        .get_shortest_path_between_all_pairs()
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::utils::{get_day_test_input, parse_input};

    use super::*;

    // #[test]
    // fn test_part1() {
    //     let image = parse_input(get_day_test_input("day11"));
    //     assert_eq!(part1(&image), 374);
    // }

    // #[test]
    // fn test_part2() {
    //     let image = parse_input(get_day_test_input("day11"));
    //     assert_eq!(part1_and_part2(&image), 8410);
    // }
}

use std::{iter::FusedIterator, str::FromStr};

#[derive(Debug)]
pub struct GridPattern {
    inner: Vec<char>,
    rows: usize,
    columns: usize,
}

trait EqualExceptOne: PartialEq {
    fn equal_except_one(&self, other: &Self) -> bool;
}

impl<'a> EqualExceptOne for &'a [char] {
    fn equal_except_one(&self, other: &Self) -> bool {
        let mut found_unequal = false;
        for (a, b) in self.iter().zip(other.iter()) {
            if a != b {
                if found_unequal {
                    return false;
                } else {
                    found_unequal = true;
                }
            }
        }

        found_unequal
    }
}

impl<'a> EqualExceptOne for Vec<char> {
    fn equal_except_one(&self, other: &Self) -> bool {
        let mut found_unequal = false;
        for (a, b) in self.iter().zip(other.iter()) {
            if a != b {
                if found_unequal {
                    return false;
                } else {
                    found_unequal = true;
                }
            }
        }

        found_unequal
    }
}

///
/// Create 2 iterators going backward and forward from a given line (can be row or column)
/// and simultaneously checking if they are equal. If not, we return false because
/// it's not reflected using this line.
///
/// Assume `total_lines` >= `reflection_line`
///
/// Maybe it was better for T not to be clone and just call it twice with the same it?
///
fn is_reflected<T>(it: T, reflection_line: usize, total_lines: usize) -> bool
where
    T: Iterator + DoubleEndedIterator + Clone,
    T::Item: PartialEq,
{
    let mut equal = true;
    let mut forward_it = it.clone().skip(reflection_line);
    let mut back_it = it.rev().skip(total_lines - reflection_line);
    while let (Some(forward), Some(back)) = (forward_it.next(), back_it.next()) {
        if forward != back {
            equal = false;
            break;
        }
    }

    equal
}

fn is_reflected_with_smudge<T>(it: T, reflection_line: usize, total_lines: usize) -> bool
where
    T: Iterator + DoubleEndedIterator + Clone,
    T::Item: EqualExceptOne,
{
    let mut forward_it = it.clone().skip(reflection_line);
    let mut back_it = it.rev().skip(total_lines - reflection_line);
    let mut found_single_smudge = false;
    while let (Some(forward), Some(back)) = (forward_it.next(), back_it.next()) {
        if forward.equal_except_one(&back) {
            if found_single_smudge {
                // second like that is almost equal
                return false;
            } else {
                found_single_smudge = true;
            }
        } else if forward != back {
            return false;
        }
    }

    found_single_smudge
}

impl GridPattern {
    pub fn from_str_lines(lines: &[&str]) -> Self {
        let rows = lines.len();
        let columns = match lines.get(0) {
            Some(c) => c.len(),
            None => {
                return Self {
                    inner: Vec::new(),
                    rows: 0,
                    columns: 0,
                }
            }
        };

        let inner = lines.iter().flat_map(|line| line.chars()).collect();
        Self {
            inner,
            rows,
            columns,
        }
    }

    fn row_iter(&self) -> RowIterator<'_> {
        RowIterator::new(self)
    }

    fn column_iter(&self) -> ColumnIterator<'_> {
        ColumnIterator::new(self)
    }

    fn find_horizontal_reflection_line(&self, with_smudge: bool) -> Option<usize> {
        for reflection_row in 1..self.rows {
            let is_reflected = match with_smudge {
                false => is_reflected(self.row_iter(), reflection_row, self.rows),
                true => is_reflected_with_smudge(self.row_iter(), reflection_row, self.rows),
            };

            if is_reflected {
                return Some(reflection_row);
            }
        }

        None
    }

    fn find_vertical_reflection_line(&self, with_smudge: bool) -> Option<usize> {
        for reflection_column in 1..self.columns {
            let is_reflected = match with_smudge {
                false => is_reflected(self.column_iter(), reflection_column, self.columns),
                true => {
                    is_reflected_with_smudge(self.column_iter(), reflection_column, self.columns)
                }
            };

            if is_reflected {
                return Some(reflection_column);
            }
        }

        None
    }

    fn find_reflection(&self, with_smudge: bool) -> Option<usize> {
        let horizontal = self.find_horizontal_reflection_line(with_smudge);
        if let Some(horizontal) = horizontal {
            return Some((horizontal) * 100);
        }

        let vertical = self.find_vertical_reflection_line(with_smudge);
        if let Some(vertical) = vertical {
            return Some(vertical);
        }

        None
    }
}

// I think iterators shouldn't be copy
#[derive(Debug, Clone)]
struct RowIterator<'a> {
    grid: &'a GridPattern,
    current_row_from_start: usize,
    current_row_from_end: usize,
}

impl<'a> RowIterator<'a> {
    fn new(grid_pattern: &'a GridPattern) -> Self {
        Self {
            grid: grid_pattern,
            current_row_from_start: 0,
            current_row_from_end: grid_pattern.rows,
        }
    }
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        // row end is initialized at max rows so no need to check if current is above max
        if self.current_row_from_start >= self.current_row_from_end {
            return None;
        }

        let start_index = self.current_row_from_start * self.grid.columns;
        let end_index = start_index + self.grid.columns;
        let to_yield = self
            .grid
            .inner
            .get(start_index..end_index)
            .expect("We checked the slice is valid");
        self.current_row_from_start += 1;
        Some(to_yield)
    }
}

impl<'a> DoubleEndedIterator for RowIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_row_from_end <= self.current_row_from_start {
            return None;
        }
        self.current_row_from_end -= 1;
        let start_index = self.current_row_from_end * self.grid.columns;
        let end_index = start_index + self.grid.columns;
        let to_yield = self
            .grid
            .inner
            .get(start_index..end_index)
            .expect("We checked the slice is valid");
        Some(to_yield)
    }
}

#[derive(Debug, Clone)]
struct ColumnIterator<'a> {
    grid: &'a GridPattern,
    current_column_from_start: usize,
    current_column_from_end: usize,
}

impl<'a> ColumnIterator<'a> {
    fn new(grid_pattern: &'a GridPattern) -> Self {
        Self {
            grid: grid_pattern,
            current_column_from_start: 0,
            current_column_from_end: grid_pattern.columns,
        }
    }
}

impl<'a> FusedIterator for RowIterator<'a> {}

impl<'a> Iterator for ColumnIterator<'a> {
    // columns aren't continous in memory so can't return a slice
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_column_from_start >= self.current_column_from_end {
            return None;
        }

        let mut column_data = Vec::with_capacity(self.grid.rows);
        for row in 0..self.grid.rows {
            let index = self.current_column_from_start + row * self.grid.columns;
            column_data.push(
                *self
                    .grid
                    .inner
                    .get(index)
                    .expect("the index is always in the bounds"),
            )
        }

        self.current_column_from_start += 1;
        Some(column_data)
    }
}

impl<'a> DoubleEndedIterator for ColumnIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_column_from_end <= self.current_column_from_start {
            return None;
        }
        self.current_column_from_end -= 1;

        let mut column_data = Vec::with_capacity(self.grid.rows);
        for row in 0..self.grid.rows {
            let index = self.current_column_from_end + row * self.grid.columns;
            column_data.push(
                *self
                    .grid
                    .inner
                    .get(index)
                    .expect("the index is always in the bounds"),
            )
        }
        Some(column_data)
    }
}

impl<'a> FusedIterator for ColumnIterator<'a> {}

#[derive(Debug)]
pub struct GridPatterns {
    patterns: Vec<GridPattern>,
}

impl FromStr for GridPatterns {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().into_iter();
        let mut current_pattern_lines = vec![];
        let mut patterns = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                // reached the end of a pattern
                patterns.push(GridPattern::from_str_lines(&current_pattern_lines));
                current_pattern_lines = vec![];
            } else {
                current_pattern_lines.push(line);
            }
        }

        patterns.push(GridPattern::from_str_lines(&current_pattern_lines));
        Ok(Self { patterns })
    }
}

fn find_reflection(grid_patterns: &GridPatterns, with_smudge: bool) -> usize {
    grid_patterns
        .patterns
        .iter()
        .map(|x| {
            x.find_reflection(with_smudge)
                .expect("question must be solvable")
        })
        .sum()
}

pub fn part1(grid_patterns: &GridPatterns) -> usize {
    find_reflection(grid_patterns, false)
}

pub fn part2(grid_patterns: &GridPatterns) -> usize {
    find_reflection(grid_patterns, true)
}

#[cfg(test)]
mod tests {
    use crate::utils::{get_day_test_input, parse_input};

    use super::*;

    #[test]
    fn test_part1() {
        let grid_patterns = parse_input(get_day_test_input("day13"));
        assert_eq!(part1(&grid_patterns), 405);
    }

    #[test]
    fn test_part2() {
        let grid_patterns = parse_input(get_day_test_input("day13"));
        assert_eq!(part2(&grid_patterns), 400);
    }
}

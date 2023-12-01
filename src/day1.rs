use std::{collections::HashMap, iter::FusedIterator, path::Path, str::Chars};

use crate::utils::read_lines;

// part 1 and 2 can be the same because test doesn't contain named digits
pub fn day1<P: AsRef<Path>>(filename: P) -> u32 {
    read_lines(filename)
        .into_iter()
        .map(|x| {
            get_first_and_last_digit(&x.unwrap())
                .unwrap()
                .as_two_digit_num() as u32
        })
        .sum()
}

struct FirstAndLastDigit {
    pub first: u8,
    pub last: u8,
}

impl FirstAndLastDigit {
    pub fn as_two_digit_num(&self) -> u8 {
        self.first * 10 + self.last
    }
}

///
/// Find the first and last digit from a given string. Also this assumes the digits are ascii.
///
fn get_first_and_last_digit(haystack: &str) -> Option<FirstAndLastDigit> {
    let mut it = DigitOrNamedDigit::new(haystack).into_iter().fuse(); // fuse because we call next twice without checking if the first one returned None

    let first = it.next();
    let last = it.rev().next();

    match (first, last) {
        (Some(first), Some(last)) => Some(FirstAndLastDigit {
            first: first as u8,
            last: last as u8,
        }),
        (Some(first), None) => Some(FirstAndLastDigit {
            first: first as u8,
            last: first as u8,
        }),
        _ => None,
    }
}

struct DigitOrNamedDigit<'a> {
    buffer: &'a str,
    index: usize,
    back_index: usize,
    named_to_digit: HashMap<&'static str, u8>,
}

impl<'a> DigitOrNamedDigit<'a> {
    pub fn new(haystack: &'a str) -> Self {
        let named_to_digit = HashMap::from_iter([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);

        let back_index = if haystack.len() == 0 {
            0
        } else {
            haystack.len()
        };
        Self {
            buffer: haystack,
            index: 0,
            back_index,
            named_to_digit,
        }
    }
}

impl<'a> Iterator for DigitOrNamedDigit<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index > self.buffer.len() || self.index > self.back_index {
                return None;
            }

            let mut chars = self.buffer.chars();

            const RADIX: u32 = 10;

            if let Some(c) = chars.nth(self.index) {
                if let Some(digit) = c.to_digit(RADIX) {
                    self.index += 1;
                    return Some(digit as u8);
                }
            }

            for (named_digit, digit) in self.named_to_digit.iter() {
                if let Some(in_buffer) = self.buffer.get(self.index..self.index + named_digit.len())
                {
                    if &in_buffer == named_digit {
                        self.index += named_digit.len();
                        return Some(*digit);
                    }
                }
            }

            self.index += 1;
        }
    }
}

impl<'a> DoubleEndedIterator for DigitOrNamedDigit<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if self.back_index < self.index {
                return None;
            }

            const RADIX: u32 = 10;

            let mut chars = self.buffer.chars();
            if let Some(Some(digit)) = chars.nth(self.back_index).map(|c| c.to_digit(RADIX)) {
                self.back_index = 0.min(self.back_index - 1);
                return Some(digit as u8);
            }

            for (named_digit, digit) in self.named_to_digit.iter() {
                if self.back_index < named_digit.len() {
                    continue;
                }
                if let Some(in_buffer) = self
                    .buffer
                    .get(self.back_index - named_digit.len()..self.back_index)
                {
                    if &in_buffer == named_digit {
                        self.back_index -= named_digit.len();
                        return Some(*digit);
                    }
                }
            }

            if self.back_index == 0 {
                return None;
            }
            self.back_index -= 1;
        }
    }
}

///
/// We continue returning None even after the first None is returned
impl<'a> FusedIterator for DigitOrNamedDigit<'a> {}

#[cfg(test)]
mod tests {
    use super::day1;

    #[test]
    fn test_day() {
        let result = day1("input/day1/test.txt");
        assert_eq!(result, 142);
    }
}

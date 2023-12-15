use std::str::FromStr;

use anyhow::Context;
use itertools::Itertools;

// assume all characters are ascii
fn hash_char(c: char, current_value: u32) -> u32 {
    (((c as u8) as u32 + current_value) * 17) % 256
}

fn hash_str(step: &str) -> u32 {
    step.chars()
        .into_iter()
        .fold(0, |current_value, c| hash_char(c, current_value))
}

pub fn part1(input: &str) -> u32 {
    input.trim().split(",").map(hash_str).sum()
}

#[derive(Debug)]
struct BoxContent {
    label: String,
    focal_length: u32,
}

enum SequenceOperation {
    AddLens {
        box_content: BoxContent,
        box_index: usize,
    },
    RemoveLens {
        label: String,
        box_index: usize,
    },
}

impl FromStr for SequenceOperation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, rest) = match s.chars().last().context("got empty string")? {
            '-' => {
                anyhow::ensure!(s.len() > 1);
                s.split_at(s.len() - 1)
            }
            d if d.is_ascii_digit() => {
                anyhow::ensure!(s.len() > 2);
                s.split_at(s.len() - 2)
            }
            _ => anyhow::bail!("invalid sequence: {s}"),
        };
        let box_index = hash_str(label) as usize;
        let label = label.to_string();

        let mut rest = rest.chars();
        match rest.next().context("invalid char sequence")? {
            '=' => Ok(Self::AddLens {
                box_content: BoxContent {
                    label,
                    focal_length: rest
                        .next()
                        .context("missing focal length")?
                        .to_digit(10)
                        .context("failed to convert to digit")?,
                },
                box_index,
            }),
            '-' => Ok(Self::RemoveLens { label, box_index }),
            _ => anyhow::bail!("invalid operation: {}", s),
        }
    }
}

fn build_lens_hashmap(
    operations: Vec<SequenceOperation>,
) -> anyhow::Result<[Vec<BoxContent>; 256]> {
    let mut boxes = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    let mut boxes: [Vec<BoxContent>; 256] = boxes.try_into().expect("number of boxes matches");

    for operation in operations {
        match operation {
            SequenceOperation::AddLens {
                box_content,
                box_index,
            } => {
                let wanted_box = boxes
                    .get_mut(box_index)
                    .context("invalid box index for add")?;

                let mut replaced = false;
                for content in wanted_box.iter_mut() {
                    if content.label == box_content.label {
                        content.focal_length = box_content.focal_length;
                        replaced = true;
                        break;
                    }
                }

                if !replaced {
                    wanted_box.push(box_content);
                }
            }
            SequenceOperation::RemoveLens { label, box_index } => boxes
                .get_mut(box_index)
                .context("invalid box index for remove")?
                .retain(|x| x.label != label),
        }
    }

    Ok(boxes)
}

fn get_focusing_power(indexed_box: (usize, Vec<BoxContent>)) -> usize {
    let (box_index, box_content_vec) = indexed_box;
    box_content_vec
        .into_iter()
        .enumerate()
        .map(|(lens_index, content)| {
            (box_index + 1) * (lens_index + 1) * content.focal_length as usize
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let operations = input
        .trim()
        .split(",")
        .map(|x| {
            x.parse::<SequenceOperation>()
                .expect("sequence can always be parsed")
        })
        .collect_vec();

    let hashmap = build_lens_hashmap(operations).unwrap();
    hashmap
        .into_iter()
        .enumerate()
        .map(get_focusing_power)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::utils::get_day_test_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = get_day_test_input("day15");
        let input = read_to_string(&input).unwrap();
        assert_eq!(part1(&input), 1320);
    }

    #[test]
    fn test_part2() {
        let input = get_day_test_input("day15");
        let input = read_to_string(&input).unwrap();
        assert_eq!(part2(&input), 145);
    }
}

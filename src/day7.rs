use std::{cmp::Ordering, str::FromStr};

use anyhow::Context;
use itertools::Itertools;

// Order is important here because we derive PartialOrd
// not sure if giving them value is good
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Card {
    J = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    T = 9,
    Q = 10,
    K = 11,
    A = 12,
}

// yes can use macro but didn't care for adding another dependency now
const NUM_CARDS: usize = 13;

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let result = match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => anyhow::bail!("failed to parse card: {value}"),
        };
        Ok(result)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandKind {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl HandKind {
    fn upgrade_by_j_count(&self, j_count: usize) -> anyhow::Result<Self> {
        if j_count == 0 {
            return Ok(*self);
        }

        let result = match self {
            HandKind::HighCard => match j_count {
                1 => HandKind::OnePair,
                _ => anyhow::bail!("invalid count: {j_count}"),
            },
            HandKind::OnePair => match j_count {
                1 | 2 => HandKind::ThreeOfAKind,
                _ => anyhow::bail!("invalid count: {j_count}"),
            },
            HandKind::TwoPair => match j_count {
                1 => HandKind::FullHouse,
                2 => HandKind::FourOfAKind,
                _ => anyhow::bail!("invalid count: {j_count}"),
            },
            HandKind::ThreeOfAKind => match j_count {
                1 | 3 => HandKind::FourOfAKind,
                _ => anyhow::bail!("invalid count: {j_count}"),
            },
            HandKind::FullHouse => match j_count {
                2 | 3 => HandKind::FiveOfAKind,
                _ => anyhow::bail!("invalid count: {j_count}"),
            },
            HandKind::FourOfAKind => match j_count {
                1 | 4 => HandKind::FiveOfAKind,
                _ => anyhow::bail!("invalid count: {j_count}"),
            },
            HandKind::FiveOfAKind => match j_count {
                5 => HandKind::FiveOfAKind,
                _ => anyhow::bail!("invalid count: {j_count}"),
            },
        };
        Ok(result)
    }
}

// They can only be equal if the cards are equal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
}

fn hand_kind_exluding_special_j(non_zero: &[i32]) -> HandKind {
    if non_zero.len() == 1 {
        return HandKind::FiveOfAKind;
    }

    if non_zero.len() == 4 {
        return HandKind::OnePair;
    }

    if non_zero.len() == 2 {
        if non_zero[0] == 4 || non_zero[1] == 4 {
            return HandKind::FourOfAKind;
        }
        return HandKind::FullHouse;
    }

    if non_zero.len() == 3 {
        if non_zero.iter().any(|x| *x == 3) {
            return HandKind::ThreeOfAKind;
        }
        return HandKind::TwoPair;
    }

    // We assume all hands are of some type
    return HandKind::HighCard;
}

impl Hand {
    fn get_hand_kind(&self) -> HandKind {
        let mut card_count = [0; NUM_CARDS];
        let mut num_j = 0;
        for card in &self.cards {
            if *card == Card::J {
                num_j += 1;
                // ignore j cards
            }
            card_count[*card as usize] += 1;
        }

        let non_zero = card_count.into_iter().filter(|x| *x != 0).collect_vec();
        let hand_kind = hand_kind_exluding_special_j(&non_zero);
        eprintln!("{:?} {}", self.cards, num_j);
        hand_kind.upgrade_by_j_count(num_j).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_order = self.get_hand_kind().cmp(&other.get_hand_kind());
        if hand_order != Ordering::Equal {
            return Some(hand_order);
        }

        // Same hand kind, need to start looking at the cards in order

        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            let card_order = self_card.cmp(other_card);
            if card_order == Ordering::Equal {
                continue;
            }
            return Some(card_order);
        }

        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("partial cmp is never None")
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 32T3K
        let cards: anyhow::Result<Vec<Card>> = s
            .chars()
            .into_iter()
            .map(|x| x.try_into().context("failed to parse single card"))
            .collect();
        let parsed_cards = cards?;
        let cards = parsed_cards.try_into().map_err(|original_vec: Vec<Card>| {
            anyhow::anyhow!("vec has size: {} which is invalid", original_vec.len())
        })?;

        Ok(Self { cards })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HandBid {
    hand: Hand,
    bid: u32,
}

impl PartialEq for HandBid {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for HandBid {}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl Ord for HandBid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("hand bid partial eq is never None")
    }
}

impl FromStr for HandBid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 32T3K 765
        let mut it = s.split_whitespace();
        let hand = it
            .next()
            .context("missing hand")?
            .parse()
            .context("failed to parse hand")?;
        let bid = it
            .next()
            .context("missing bid")?
            .parse()
            .context("failed to parse bid")?;
        Ok(Self { hand, bid })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandSet {
    hand_bids: Vec<HandBid>,
}

impl FromStr for HandSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand_bids: anyhow::Result<Vec<HandBid>> = s
            .lines()
            .map(|x| x.parse().context("failed to parse hand bid line"))
            .collect();
        Ok(Self {
            hand_bids: hand_bids?,
        })
    }
}

pub fn part2(hand_set: &HandSet) -> u32 {
    let mut sorted_hand = hand_set.hand_bids.iter().map(|x| x).collect_vec();
    eprintln!(
        "{:?}",
        hand_set
            .hand_bids
            .iter()
            .map(|x| x.hand.get_hand_kind())
            .collect_vec()
    );
    sorted_hand.sort();

    sorted_hand
        .into_iter()
        .enumerate()
        .map(|(index, hand_bid)| ((index + 1) as u32) * hand_bid.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::utils::{get_day_test_input, parse_input};

    use super::*;

    // #[test]
    // fn test_part1() {
    //     let hand_set = parse_input(get_day_test_input("day7"));
    //     assert_eq!(part1(&hand_set), 6440);
    // }

    #[test]
    fn test_part2() {
        let hand_set = parse_input(get_day_test_input("day7"));
        assert_eq!(part2(&hand_set), 5905);
    }
}

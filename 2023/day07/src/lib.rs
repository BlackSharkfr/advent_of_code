use std::str::FromStr;

use aoc::Aoc;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 7;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let mut lines = input
            .lines()
            .map(|line| {
                let (_, (cards, bid)) = parsers::part1(line)
                    .unwrap_or_else(|e| panic!("Parse failed {e:?} at line {line}"));
                let score = cards_value_part1(cards);
                (score, bid)
            })
            .collect::<Vec<_>>();

        lines.par_sort_unstable_by_key(|(score, _)| *score);

        lines
            .into_iter()
            .enumerate()
            .map(|(index, (_, bid))| (index as u32 + 1) * bid)
            .sum()
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let mut lines = input
            .lines()
            .map(|line| {
                let (_, (cards, bid)) = parsers::part2(line)
                    .unwrap_or_else(|e| panic!("Parse failed {e:?} at line {line}"));
                let score = cards_value_part2(cards);
                (score, bid)
            })
            .collect::<Vec<_>>();

        lines.par_sort_unstable_by_key(|(score, _)| *score);

        lines
            .into_par_iter()
            .enumerate()
            .map(|(index, (_value, bid))| (index as u32 + 1) * bid)
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardPart1 {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    Jack,
    Queen,
    King,
    Ace,
}
impl FromStr for CardPart1 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Self::C2),
            "3" => Ok(Self::C3),
            "4" => Ok(Self::C4),
            "5" => Ok(Self::C5),
            "6" => Ok(Self::C6),
            "7" => Ok(Self::C7),
            "8" => Ok(Self::C8),
            "9" => Ok(Self::C9),
            "T" => Ok(Self::C10),
            "J" => Ok(Self::Jack),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardPart2 {
    Joker,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    Queen,
    King,
    Ace,
}
impl FromStr for CardPart2 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "J" => Ok(Self::Joker),
            "2" => Ok(Self::C2),
            "3" => Ok(Self::C3),
            "4" => Ok(Self::C4),
            "5" => Ok(Self::C5),
            "6" => Ok(Self::C6),
            "7" => Ok(Self::C7),
            "8" => Ok(Self::C8),
            "9" => Ok(Self::C9),
            "T" => Ok(Self::C10),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err(()),
        }
    }
}

fn cards_value_part1(cards: [CardPart1; 5]) -> u32 {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (*card as u32) << (4 * i))
        .sum::<u32>()
        + ((Hand::from(cards) as u32) << (4 * 5))
}

fn cards_value_part2(cards: [CardPart2; 5]) -> u32 {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (*card as u32) << (4 * i))
        .sum::<u32>()
        + ((Hand::from(cards) as u32) << (4 * 5))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}
impl From<[CardPart1; 5]> for Hand {
    fn from(cards: [CardPart1; 5]) -> Self {
        let mut counts = [0; 13];
        for card in cards {
            counts[card as usize] += 1;
        }
        counts.sort();
        Hand::from_highest_counts(counts[12], counts[11])
    }
}
impl From<[CardPart2; 5]> for Hand {
    fn from(cards: [CardPart2; 5]) -> Self {
        let mut counts = [0; 13];
        for card in cards {
            counts[card as usize] += 1;
        }
        let jokers = std::mem::take(&mut counts[0]);
        counts.sort();
        counts[12] += jokers;
        Hand::from_highest_counts(counts[12], counts[11])
    }
}
impl Hand {
    fn from_highest_counts(highest: u16, second: u16) -> Hand {
        match (highest, second) {
            (5, _) => Hand::FiveOfKind,
            (4, _) => Hand::FourOfKind,
            (3, 2) => Hand::FullHouse,
            (3, _) => Hand::ThreeOfKind,
            (2, 2) => Hand::TwoPair,
            (2, _) => Hand::OnePair,
            _ => Hand::HighCard,
        }
    }
}

mod parsers {
    use nom::{
        bytes::complete::take,
        character::complete::{space1, u32},
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use super::*;

    pub fn part1(input: &str) -> IResult<&str, ([CardPart1; 5], u32)> {
        let (input, cards) = five_cards_part1(input)?;
        let (input, _) = space1(input)?;
        let (input, bid) = u32(input)?;
        Ok((input, (cards, bid)))
    }
    pub fn part2(input: &str) -> IResult<&str, ([CardPart2; 5], u32)> {
        let (input, cards) = five_cards_part2(input)?;
        let (input, _) = space1(input)?;
        let (input, bid) = u32(input)?;
        Ok((input, (cards, bid)))
    }

    fn card_part1(input: &str) -> IResult<&str, CardPart1> {
        take(1_usize).parse_from_str().parse(input)
    }

    fn card_part2(input: &str) -> IResult<&str, CardPart2> {
        take(1_usize).parse_from_str().parse(input)
    }

    fn five_cards_part1(input: &str) -> IResult<&str, [CardPart1; 5]> {
        let (input, c0) = card_part1(input)?;
        let (input, c1) = card_part1(input)?;
        let (input, c2) = card_part1(input)?;
        let (input, c3) = card_part1(input)?;
        let (input, c4) = card_part1(input)?;
        Ok((input, [c0, c1, c2, c3, c4]))
    }
    fn five_cards_part2(input: &str) -> IResult<&str, [CardPart2; 5]> {
        let (input, c0) = card_part2(input)?;
        let (input, c1) = card_part2(input)?;
        let (input, c2) = card_part2(input)?;
        let (input, c3) = card_part2(input)?;
        let (input, c4) = card_part2(input)?;
        Ok((input, [c0, c1, c2, c3, c4]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(6440)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(5905)
    }
}

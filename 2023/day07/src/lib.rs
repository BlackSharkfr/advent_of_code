use std::str::FromStr;

use aoc::Aoc;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 0;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let (_, mut lines) =
            parsers::part1(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));

        lines.par_sort_by_cached_key(|(cards, _)| cards_value_part1(cards));

        lines
            .into_iter()
            .enumerate()
            .map(|(index, (_cards, bid))| (index as u32 + 1) * bid)
            .sum()
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let (_, mut lines) =
            parsers::part2(input).unwrap_or_else(|e| panic!("Parser Failed {e:?}"));

        lines.par_sort_by_cached_key(|(cards, _)| cards_value_part2(cards));

        lines
            .into_par_iter()
            .enumerate()
            .map(|(index, (_value, bid))| (index as u32 + 1) * bid)
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
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
impl CardPart1 {
    fn to_part2(&self) -> CardPart2 {
        match self {
            CardPart1::C2 => CardPart2::C2,
            CardPart1::C3 => CardPart2::C3,
            CardPart1::C4 => CardPart2::C4,
            CardPart1::C5 => CardPart2::C5,
            CardPart1::C6 => CardPart2::C6,
            CardPart1::C7 => CardPart2::C7,
            CardPart1::C8 => CardPart2::C8,
            CardPart1::C9 => CardPart2::C9,
            CardPart1::C10 => CardPart2::C10,
            CardPart1::Jack => CardPart2::Joker,
            CardPart1::Queen => CardPart2::Queen,
            CardPart1::King => CardPart2::King,
            CardPart1::Ace => CardPart2::Ace,
        }
    }
    fn value(&self) -> u32 {
        match self {
            CardPart1::C2 => 0,
            CardPart1::C3 => 1,
            CardPart1::C4 => 2,
            CardPart1::C5 => 3,
            CardPart1::C6 => 4,
            CardPart1::C7 => 5,
            CardPart1::C8 => 6,
            CardPart1::C9 => 7,
            CardPart1::C10 => 8,
            CardPart1::Jack => 9,
            CardPart1::Queen => 10,
            CardPart1::King => 11,
            CardPart1::Ace => 12,
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
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
impl CardPart2 {
    fn value(&self) -> u32 {
        match self {
            CardPart2::Joker => 0,
            CardPart2::C2 => 1,
            CardPart2::C3 => 2,
            CardPart2::C4 => 3,
            CardPart2::C5 => 4,
            CardPart2::C6 => 5,
            CardPart2::C7 => 6,
            CardPart2::C8 => 7,
            CardPart2::C9 => 8,
            CardPart2::C10 => 9,
            CardPart2::Queen => 10,
            CardPart2::King => 11,
            CardPart2::Ace => 12,
        }
    }
}

fn cards_value_part1(cards: &[CardPart1; 5]) -> u32 {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| card.value() << (4 * i))
        .sum::<u32>()
        + (Hand::from(cards).value() << (4 * 5))
}

fn cards_value_part2(cards: &[CardPart2; 5]) -> u32 {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| card.value() << (4 * i))
        .sum::<u32>()
        + (Hand::from(cards).value() << (4 * 5))
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

impl Hand {
    fn value(&self) -> u32 {
        match self {
            Hand::HighCard => 0,
            Hand::OnePair => 1,
            Hand::TwoPair => 2,
            Hand::ThreeOfKind => 3,
            Hand::FullHouse => 4,
            Hand::FourOfKind => 5,
            Hand::FiveOfKind => 6,
        }
    }
}
impl From<&[CardPart1; 5]> for Hand {
    fn from(cards: &[CardPart1; 5]) -> Self {
        let mut cards = cards.clone();
        cards.sort();

        let mut counts = [1, 0, 0, 0, 0];
        let mut insert = 0;
        for i in 0..cards.len() - 1 {
            if cards[i] == cards[i + 1] {
                counts[insert] += 1;
            } else {
                insert += 1;
                counts[insert] = 1;
            }
        }
        counts.sort();

        match (counts[4], counts[3]) {
            (5, _) => Hand::FiveOfKind,
            (4, 1) => Hand::FourOfKind,
            (3, 2) => Hand::FullHouse,
            (3, 1) => Hand::ThreeOfKind,
            (2, 2) => Hand::TwoPair,
            (2, _) => Hand::OnePair,
            _ => Hand::HighCard,
        }
    }
}
impl From<&[CardPart2; 5]> for Hand {
    fn from(cards: &[CardPart2; 5]) -> Self {
        let mut cards = cards.clone();
        cards.sort();

        let (mut jokers, mut counts) = if cards[0] == CardPart2::Joker {
            (1, [0, 0, 0, 0, 0])
        } else {
            (0, [1, 0, 0, 0, 0])
        };
        let mut insert = 0;
        for i in 0..cards.len() - 1 {
            if cards[i] == CardPart2::Joker {
                jokers += 1;
                continue;
            }
            if cards[i] == cards[i + 1] {
                counts[insert] += 1;
            } else {
                insert += 1;
                counts[insert] = 1;
            }
        }
        counts.sort();
        counts[4] += jokers;

        match (counts[4], counts[3]) {
            (5, _) => Hand::FiveOfKind,
            (4, 1) => Hand::FourOfKind,
            (3, 2) => Hand::FullHouse,
            (3, 1) => Hand::ThreeOfKind,
            (2, 2) => Hand::TwoPair,
            (2, _) => Hand::OnePair,
            _ => Hand::HighCard,
        }
    }
}

mod parsers {
    use nom::{
        bytes::complete::take,
        character::complete::{line_ending, space1, u32},
        multi::separated_list1,
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use super::*;

    pub fn part1(input: &str) -> IResult<&str, Vec<([CardPart1; 5], u32)>> {
        separated_list1(line_ending, line_part1)(input)
    }

    pub fn part2(input: &str) -> IResult<&str, Vec<([CardPart2; 5], u32)>> {
        separated_list1(line_ending, line_part2)(input)
    }

    pub fn line_part1(input: &str) -> IResult<&str, ([CardPart1; 5], u32)> {
        let (input, cards) = five_cards(input)?;
        let (input, _) = space1(input)?;
        let (input, bid) = u32(input)?;
        Ok((input, (cards, bid)))
    }
    pub fn line_part2(input: &str) -> IResult<&str, ([CardPart2; 5], u32)> {
        let (input, cards) = five_cards_part2(input)?;
        let (input, _) = space1(input)?;
        let (input, bid) = u32(input)?;
        Ok((input, (cards, bid)))
    }

    fn card(input: &str) -> IResult<&str, CardPart1> {
        take(1_usize).parse_from_str().parse(input)
    }

    fn card_part2(input: &str) -> IResult<&str, CardPart2> {
        take(1_usize)
            .parse_from_str()
            .map(|card: CardPart1| card.to_part2())
            .parse(input)
    }

    fn five_cards(input: &str) -> IResult<&str, [CardPart1; 5]> {
        let (input, c0) = card(input)?;
        let (input, c1) = card(input)?;
        let (input, c2) = card(input)?;
        let (input, c3) = card(input)?;
        let (input, c4) = card(input)?;
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

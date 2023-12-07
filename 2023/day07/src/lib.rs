use std::str::FromStr;

use aoc::Aoc;
use itertools::Itertools;

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
        lines.sort_by(|(a, _), (b, _)| compare_part1(a, b));

        lines
            .into_iter()
            .enumerate()
            .map(|(index, (_cards, bid))| {
                // println!(
                //     "rank {:>4}, bid {:>4}, hand {:?}, cards {:?}",
                //     index + 1,
                //     bid,
                //     Hand::from_part1(&cards),
                //     cards
                // );
                (index as u32 + 1) * bid
            })
            .sum()
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let (_, mut lines) =
            parsers::part2(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));
        lines.sort_by(|(a, _), (b, _)| compare_part2(a, b));

        lines
            .into_iter()
            .enumerate()
            .map(|(index, (_cards, bid))| {
                // println!(
                //     "rank {:>4}, bid {:>4}, hand {:?}, cards {:?}",
                //     index + 1,
                //     bid,
                //     Hand::from_part2(&cards),
                //     cards
                // );
                (index as u32 + 1) * bid
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Card {
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
impl FromStr for Card {
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
impl Card {
    fn to_part2(&self) -> CardPart2 {
        match self {
            Card::C2 => CardPart2::C2,
            Card::C3 => CardPart2::C3,
            Card::C4 => CardPart2::C4,
            Card::C5 => CardPart2::C5,
            Card::C6 => CardPart2::C6,
            Card::C7 => CardPart2::C7,
            Card::C8 => CardPart2::C8,
            Card::C9 => CardPart2::C9,
            Card::C10 => CardPart2::C10,
            Card::Jack => CardPart2::Joker,
            Card::Queen => CardPart2::Queen,
            Card::King => CardPart2::King,
            Card::Ace => CardPart2::Ace,
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

fn compare_part1(a: &[Card; 5], b: &[Card; 5]) -> std::cmp::Ordering {
    match Hand::from_part1(a).cmp(&Hand::from_part1(b)) {
        std::cmp::Ordering::Equal => {
            for (card_a, card_b) in a.iter().zip(b) {
                match card_a.cmp(card_b) {
                    std::cmp::Ordering::Equal => (),
                    other => return other,
                }
            }
            panic!("Hands are equal : {:?}", a);
        }
        other => other,
    }
}
fn compare_part2(a: &[CardPart2; 5], b: &[CardPart2; 5]) -> std::cmp::Ordering {
    match Hand::from_part2(a).cmp(&Hand::from_part2(b)) {
        std::cmp::Ordering::Equal => {
            for (card_a, card_b) in a.iter().zip(b) {
                match card_a.cmp(&card_b) {
                    std::cmp::Ordering::Equal => (),
                    other => return other,
                }
            }
            panic!("Hands are equal : {:?}", a);
        }
        other => other,
    }
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
    fn from_part1(cards: &[Card; 5]) -> Self {
        let mut counts = cards.iter().counts().into_values().collect_vec();
        counts.sort();

        match (counts.pop(), counts.pop()) {
            (Some(5), _) => Hand::FiveOfKind,
            (Some(4), Some(1)) => Hand::FourOfKind,
            (Some(3), Some(2)) => Hand::FullHouse,
            (Some(3), Some(1)) => Hand::ThreeOfKind,
            (Some(2), Some(2)) => Hand::TwoPair,
            (Some(2), _) => Hand::OnePair,
            _ => Hand::HighCard,
        }
    }
    fn from_part2(cards: &[CardPart2; 5]) -> Self {
        let mut bins = cards.iter().counts();
        let jokers = bins.remove(&CardPart2::Joker);
        let mut counts = bins.into_values().collect_vec();
        counts.sort();
        match (counts.last_mut(), jokers) {
            (None, _) => return Hand::FiveOfKind,
            (Some(highest), Some(jokers)) => *highest += jokers,
            _ => (),
        }
        match (counts.pop(), counts.pop()) {
            (Some(5), _) => Hand::FiveOfKind,
            (Some(4), Some(1)) => Hand::FourOfKind,
            (Some(3), Some(2)) => Hand::FullHouse,
            (Some(3), Some(1)) => Hand::ThreeOfKind,
            (Some(2), Some(2)) => Hand::TwoPair,
            (Some(2), _) => Hand::OnePair,
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

    pub fn part1(input: &str) -> IResult<&str, Vec<([Card; 5], u32)>> {
        separated_list1(line_ending, line_part1)(input)
    }
    pub fn part2(input: &str) -> IResult<&str, Vec<([CardPart2; 5], u32)>> {
        separated_list1(line_ending, line_part2)(input)
    }

    fn line_part1(input: &str) -> IResult<&str, ([Card; 5], u32)> {
        let (input, cards) = five_cards(input)?;
        let (input, _) = space1(input)?;
        let (input, bid) = u32(input)?;
        Ok((input, (cards, bid)))
    }
    fn line_part2(input: &str) -> IResult<&str, ([CardPart2; 5], u32)> {
        let (input, cards) = five_cards_part2(input)?;
        let (input, _) = space1(input)?;
        let (input, bid) = u32(input)?;
        Ok((input, (cards, bid)))
    }

    fn card(input: &str) -> IResult<&str, Card> {
        take(1_usize).parse_from_str().parse(input)
    }

    fn card_part2(input: &str) -> IResult<&str, CardPart2> {
        take(1_usize)
            .parse_from_str()
            .map(|card: Card| card.to_part2())
            .parse(input)
    }

    fn five_cards(input: &str) -> IResult<&str, [Card; 5]> {
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

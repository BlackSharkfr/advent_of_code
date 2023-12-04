use std::collections::VecDeque;

use aoc::Aoc;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 4;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                let (_, points) = parsers::part1(line)
                    .unwrap_or_else(|e| panic!("Parser failed on line {line}: {e}"));
                points
            })
            .sum()
    }

    fn part2(mut input: &str) -> u32 {
        let mut cards = 0;
        let mut next_cards = VecDeque::<u32>::new();
        while let Ok((remain, points)) = parsers::part2(input) {
            let amount_of_cards = next_cards.pop_front().unwrap_or(1);
            for index in 0..points {
                match next_cards.get_mut(index as usize) {
                    Some(n) => *n += amount_of_cards,
                    None => next_cards.push_back(1 + amount_of_cards),
                }
            }
            cards += amount_of_cards;
            input = remain;
        }
        cards
    }
}

pub fn part1_rayon(input: &str) -> u32 {
    input
        .par_lines()
        .map(|line| {
            let (_, points) = parsers::part1(line)
                .unwrap_or_else(|e| panic!("Parser failed on line {line}: {e}"));
            points
        })
        .sum()
}

pub fn part2_rayon(input: &str) -> u32 {
    let points = input
        .par_lines()
        .map(|line| {
            let (_, points) = parsers::part2(line)
                .unwrap_or_else(|e| panic!("Parser failed on line {line}: {e}"));
            points
        })
        .collect::<Vec<_>>();

    let mut cards = 0;
    let mut next_cards = VecDeque::<u32>::new();
    for points in points {
        let amount_of_cards = next_cards.pop_front().unwrap_or(1);
        for index in 0..points {
            match next_cards.get_mut(index as usize) {
                Some(n) => *n += amount_of_cards,
                None => next_cards.push_back(1 + amount_of_cards),
            }
        }
        cards += amount_of_cards;
    }
    cards
}

mod parsers {
    use nom::{
        bytes::complete::take_until,
        character::complete::{char, space1, u8},
        sequence::preceded,
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    pub fn part1(input: &str) -> IResult<&str, u32> {
        let (input, _) = take_until(":")(input)?;
        let input = &input[1..];
        let (input, _) = space1(input)?;

        // YOU SHALL NOT ALLOCATE !
        // let's replace this perfectly fine vector...
        // let (input, winners) = many1(u8.terminated(space1))(input)?;

        // ...with this silly array (variable size between test sample and input)
        let (input, winners) = winners(input)?;

        let (mut input, _) = char('|')(input)?;

        let mut score = 0;
        while let Ok((remain, number)) = preceded(space1::<&str, ()>, u8)(input) {
            if winners.contains(&number) {
                score += 1
            }
            input = remain
        }
        let score = match score {
            0 => 0,
            1 => 1,
            _ => 2_u32.pow(score - 1),
        };

        Ok((input, score))
    }

    pub fn part2(input: &str) -> IResult<&str, u8> {
        let (input, _) = take_until(":")(input)?;
        let input = &input[1..];
        let (input, _) = space1(input)?;

        // YOU SHALL NOT ALLOCATE !
        // let's replace this perfectly fine vector...
        // let (input, winners) = many1(u8.terminated(space1))(input)?;

        // ...with this silly array (variable size between test sample and input)
        let (input, winners) = winners(input)?;

        let (mut input, _) = char('|')(input)?;
        let mut score = 0;
        while let Ok((remain, number)) = preceded(space1::<&str, ()>, u8)(input) {
            if winners.contains(&number) {
                score += 1;
            }
            input = remain
        }

        Ok((input, score))
    }

    #[cfg(test)]
    fn winners(input: &str) -> IResult<&str, [u8; 5]> {
        let (input, w0) = u8.terminated(space1).parse(input)?;
        let (input, w1) = u8.terminated(space1).parse(input)?;
        let (input, w2) = u8.terminated(space1).parse(input)?;
        let (input, w3) = u8.terminated(space1).parse(input)?;
        let (input, w4) = u8.terminated(space1).parse(input)?;
        let winners = [w0, w1, w2, w3, w4];
        Ok((input, winners))
    }

    #[cfg(not(test))]
    fn winners(input: &str) -> IResult<&str, [u8; 10]> {
        let (input, w0) = u8.terminated(space1).parse(input)?;
        let (input, w1) = u8.terminated(space1).parse(input)?;
        let (input, w2) = u8.terminated(space1).parse(input)?;
        let (input, w3) = u8.terminated(space1).parse(input)?;
        let (input, w4) = u8.terminated(space1).parse(input)?;
        let (input, w5) = u8.terminated(space1).parse(input)?;
        let (input, w6) = u8.terminated(space1).parse(input)?;
        let (input, w7) = u8.terminated(space1).parse(input)?;
        let (input, w8) = u8.terminated(space1).parse(input)?;
        let (input, w9) = u8.terminated(space1).parse(input)?;
        let winners = [w0, w1, w2, w3, w4, w5, w6, w7, w8, w9];
        Ok((input, winners))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(13)
    }

    #[test]
    fn test_part1_rayon() {
        let input = Day::SAMPLE_PART1;
        assert_eq!(13, part1_rayon(input));
    }

    #[test]
    fn test_part2() {
        Day::test_part2(30)
    }

    #[test]
    fn test_part2_rayon() {
        let input = Day::SAMPLE_PART2;
        assert_eq!(30, part2_rayon(input));
    }
}

use std::collections::VecDeque;

use aoc::Aoc;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 4;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(mut input: &str) -> u32 {
        let mut sum = 0;
        while let Ok((remain, points)) = parsers::part1(input) {
            sum += points;
            input = remain;
        }
        sum
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

mod parsers {
    use nom::{
        bytes::complete::take_until,
        character::complete::{char, line_ending, space1, u8},
        multi::many1,
        sequence::preceded,
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    pub fn part1(input: &str) -> IResult<&str, u32> {
        let (input, _) = take_until(":")(input)?;
        let input = &input[1..];
        let (input, _) = space1(input)?;
        let (input, winners) = many1(u8.terminated(space1))(input)?;
        let (mut input, _) = char('|')(input)?;

        let mut score = 0;
        while let Ok((remain, number)) = preceded(space1::<&str, ()>, u8)(input) {
            if winners.contains(&number) {
                match score {
                    0 => score = 1,
                    _ => score *= 2,
                }
            }
            input = remain
        }

        let (input, _) = line_ending(input)?;

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
        let (input, w0) = u8.terminated(space1).parse(input)?;
        let (input, w1) = u8.terminated(space1).parse(input)?;
        let (input, w2) = u8.terminated(space1).parse(input)?;
        let (input, w3) = u8.terminated(space1).parse(input)?;
        let (input, w4) = u8.terminated(space1).parse(input)?;
        #[cfg(test)]
        let winners = [w0, w1, w2, w3, w4];
        #[cfg(not(test))]
        let (input, winners) = {
            let (input, w5) = u8.terminated(space1).parse(input)?;
            let (input, w6) = u8.terminated(space1).parse(input)?;
            let (input, w7) = u8.terminated(space1).parse(input)?;
            let (input, w8) = u8.terminated(space1).parse(input)?;
            let (input, w9) = u8.terminated(space1).parse(input)?;
            let winners = [w0, w1, w2, w3, w4, w5, w6, w7, w8, w9];
            (input, winners)
        };

        let (mut input, _) = char('|')(input)?;
        let mut score = 0;
        while let Ok((remain, number)) = preceded(space1::<&str, ()>, u8)(input) {
            if winners.contains(&number) {
                score += 1;
            }
            input = remain
        }
        let (input, _) = line_ending(input)?;

        Ok((input, score))
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
    fn test_part2() {
        Day::test_part2(30)
    }
}

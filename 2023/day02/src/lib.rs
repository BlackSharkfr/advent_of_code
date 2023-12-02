use std::str::FromStr;

use aoc::Aoc;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 2;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> u32 {
        input
            .lines()
            .filter_map(|line| {
                let (_, game) =
                    parsers::game(line).unwrap_or_else(|e| panic!("Parser failed {e} : {line}"));
                game.possible()
            })
            .sum()
    }

    fn part2(input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                let (_, game) =
                    parsers::game(line).unwrap_or_else(|e| panic!("Parser failed {e} : {line}"));
                game.power()
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Default)]
struct Game {
    id: u8,
    red: u8,
    green: u8,
    blue: u8,
}
impl Game {
    fn new(id: u8) -> Self {
        Game {
            id,
            ..Default::default()
        }
    }
    fn insert_max_cubes(&mut self, cubes: Cubes) {
        match cubes.color {
            Color::Red => self.red = self.red.max(cubes.amount),
            Color::Green => self.green = self.green.max(cubes.amount),
            Color::Blue => self.blue = self.blue.max(cubes.amount),
        }
    }

    fn possible(&self) -> Option<u32> {
        if self.red > 12 || self.green > 13 || self.blue > 14 {
            return None;
        }
        Some(self.id as u32)
    }

    fn power(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

#[derive(Debug, PartialEq)]
struct Cubes {
    amount: u8,
    color: Color,
}

#[derive(Debug, PartialEq, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}
impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}

mod parsers {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, u8},
        IResult, Parser,
    };
    use nom_supreme::ParserExt;
    fn color(input: &str) -> IResult<&str, Color> {
        alpha1.parse_from_str().parse(input)
    }
    fn cubes(input: &str) -> IResult<&str, Cubes> {
        let (input, amount) = u8(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, color) = color(input)?;
        let reveal = Cubes { amount, color };
        Ok((input, reveal))
    }

    pub fn game(input: &str) -> IResult<&str, Game> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = u8(input)?;
        let (mut input, _) = tag(": ")(input)?;

        let mut game = Game::new(id);
        // let mut reveal = cubes.terminated(alt((tag(", "), tag("; "))).opt());
        let mut reveal = |str| {
            let (input, cubes) = cubes(str)?;
            let (input, _) = alt((tag(", "), tag("; "))).opt().parse(input)?;
            Ok((input, cubes))
        };
        while let Ok((remain, cubes)) = reveal.parse(input) {
            game.insert_max_cubes(cubes);
            input = remain;
        }

        Ok((input, game))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(8)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(2286)
    }
}

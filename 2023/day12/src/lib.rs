use std::collections::VecDeque;

use aoc::Aoc;
use itertools::Itertools;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = usize;
    const DAY_NUMBER: u8 = 0;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        input
            .par_lines()
            .map(|line| {
                let (_, data) = parsers::part1(line)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
                permutations(data)
            })
            .sum()
    }

    fn part2(input: &str) -> Self::OUTPUT {
        input
            .par_lines()
            .map(|line| {
                let (_, data) = parsers::part1(line)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
                let mut springs = data.0.clone();
                for _ in 0..4 {
                    springs.push(Spring::Unknown);
                    springs.extend_from_slice(&data.0);
                }

                let pattern = (0..5).flat_map(|_| data.1.iter().cloned()).collect_vec();
                permutations((springs, pattern))
            })
            .sum()
    }
}

fn permutations((springs, pattern): (Vec<Spring>, Vec<u8>)) -> usize {
    // println!("Initial springs : {springs:?}, pattern : {pattern:?}");
    let mut count = 0;
    let mut arrangements = VecDeque::from([springs]);
    while let Some(mut springs) = arrangements.pop_front() {
        let mut broken_count = 0;
        let mut broken_index = 0;
        for spring_index in 0..springs.len() {
            match springs[spring_index] {
                Spring::Unknown => {
                    let mut copy = springs.clone();
                    copy[spring_index] = Spring::Operational;
                    arrangements.push_back(copy);
                    springs[spring_index] = Spring::Broken;
                    broken_count += 1;
                }
                Spring::Broken => {
                    broken_count += 1;
                }
                Spring::Operational => {
                    if broken_count == 0 {
                        continue;
                    }
                    if broken_index == pattern.len() || broken_count != pattern[broken_index] {
                        break;
                    }
                    broken_index += 1;
                    broken_count = 0;
                }
            }
        }

        if broken_count != 0 {
            if broken_index == pattern.len() || broken_count != pattern[broken_index] {
                continue;
            }
            broken_index += 1;
        }

        if broken_index != pattern.len() {
            continue;
        }
        count += 1
    }
    // println!("Count: {count}");
    count
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Operational,
    Broken,
    Unknown,
}
impl TryFrom<char> for Spring {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '?' => Ok(Spring::Unknown),
            '#' => Ok(Spring::Broken),
            '.' => Ok(Spring::Operational),
            _ => Err(()),
        }
    }
}

mod parsers {
    use nom::{
        character::complete::{anychar, char, space1, u8},
        multi::{many1, separated_list1},
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use super::*;

    pub fn part1(input: &str) -> IResult<&str, (Vec<Spring>, Vec<u8>)> {
        let (input, springs) = many1(spring)(input)?;
        let (input, _) = space1(input)?;
        let (input, damaged) = separated_list1(char(','), u8)(input)?;
        Ok((input, (springs, damaged)))
    }

    fn spring(input: &str) -> IResult<&str, Spring> {
        anychar.map_res(Spring::try_from).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(21)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(525152)
    }
}

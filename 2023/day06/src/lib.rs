use aoc::Aoc;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = usize;
    const DAY_NUMBER: u8 = 6;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let (_, (times, distances)) =
            parsers::part1(input).unwrap_or_else(|e| panic!("Parser failed: {e:?}"));

        (0..times.len())
            .map(|i| brute_force(times[i] as u64, distances[i] as u64))
            .product()
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let (_, (time, distance)) =
            parsers::part2(input).unwrap_or_else(|e| panic!("Parser failed: {e:?}"));
        brute_force(time, distance)
    }
}

pub fn brute_force(time: u64, distance: u64) -> usize {
    (0..time)
        .into_par_iter()
        .filter(|&speed| (speed * (time - speed)) > distance)
        .count()
}

mod parsers {
    use nom::{bytes::complete::tag, character::complete::u32};
    use nom::{
        character::complete::{digit1, line_ending, space1},
        multi::separated_list1,
        IResult,
    };
    use nom_supreme::ParserExt;

    pub fn part1(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
        let (input, _) = tag("Time:")(input)?;
        let (input, _) = space1(input)?;
        let (input, times) = numbers_part1(input)?;
        let (input, _) = line_ending(input)?;
        let (input, _) = tag("Distance:")(input)?;
        let (input, _) = space1(input)?;
        let (input, distances) = numbers_part1(input)?;

        Ok((input, (times, distances)))
    }

    pub fn part2(input: &str) -> IResult<&str, (u64, u64)> {
        let (input, _) = tag("Time:")(input)?;
        let (input, _) = space1(input)?;
        let (input, time) = numbers_part2(input)?;
        let (input, _) = line_ending(input)?;
        let (input, _) = tag("Distance:")(input)?;
        let (input, _) = space1(input)?;
        let (input, distance) = numbers_part2(input)?;

        Ok((input, (time, distance)))
    }

    fn numbers_part1(input: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(space1, u32)(input)
    }

    fn numbers_part2(input: &str) -> IResult<&str, u64> {
        let (input, numbers) = separated_list1(
            space1,
            digit1.map_res(|digits: &str| {
                let Ok(number) = digits.parse::<u32>() else {
                    return Err(());
                };
                Ok((number, digits.len() as u32))
            }),
        )(input)?;
        let total = numbers
            .into_iter()
            .fold(0_u64, |acc, (num, len)| acc * 10_u64.pow(len) + num as u64);
        Ok((input, total))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(288)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(71503)
    }
}

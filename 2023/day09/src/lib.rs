use aoc::Aoc;
use itertools::Itertools;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = i32;
    const DAY_NUMBER: u8 = 9;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        input
            .par_lines()
            .map(|line| {
                let (_, values) = parsers::values(line)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
                predict_next(&values)
            })
            .sum()
    }

    fn part2(input: &str) -> Self::OUTPUT {
        input
            .par_lines()
            .map(|line| {
                let (_, values) = parsers::values(line)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
                predict_previous(&values)
            })
            .sum()
    }
}

fn predict_next(values: &[i32]) -> i32 {
    let differences = values
        .windows(2)
        .map(|numbers| numbers[1] - numbers[0])
        .collect_vec();
    match differences.iter().all(|number| *number == 0) {
        true => values.last().cloned(),
        false => values.last().map(|n| *n + predict_next(&differences)),
    }
    .expect("Failed to predict next number : recursion reached an empty series")
}

fn predict_previous(values: &[i32]) -> i32 {
    let differences = values
        .windows(2)
        .map(|numbers| numbers[1] - numbers[0])
        .collect_vec();
    match differences.iter().all(|number| *number == 0) {
        true => values.first().cloned(),
        false => values.first().map(|n| *n - predict_previous(&differences)),
    }
    .expect("Failed to predict previous number : recursion reached an empty series")
}

mod parsers {
    use nom::{
        character::complete::{i32, space1},
        multi::separated_list1,
        IResult,
    };

    pub fn values(input: &str) -> IResult<&str, Vec<i32>> {
        separated_list1(space1, i32)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(114)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(2)
    }
}

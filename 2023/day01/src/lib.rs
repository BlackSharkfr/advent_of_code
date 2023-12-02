use aoc::Aoc;
// use itertools::Itertools;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 1;
    const INPUT: &'static str = include_str!("../inputs/day01_input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/day01_sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/day01_sample2.txt");

    fn part1(input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                let mut digits = line.chars().filter_map(to_ascii_digit);
                let first = digits
                    .next()
                    .expect("Line should contain at least one number");
                let last = digits.next_back().unwrap_or(first);
                (10 * first) + last
            })
            .sum()
    }

    fn part2(input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                let mut digits = (0..line.len())
                    .map(|i| &line[i..])
                    .filter_map(to_digit_part2);
                let first = digits
                    .next()
                    .expect("Line should contain at least one number");
                let last = digits.next_back().unwrap_or(first);
                (first * 10) + last
            })
            .sum()
    }
}

fn to_ascii_digit(c: char) -> Option<u32> {
    c.to_digit(10)
}

fn to_digit_part2(input: &str) -> Option<u32> {
    if let Some(n) = input.chars().next().and_then(to_ascii_digit) {
        return Some(n);
    }
    let numbers = [
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ];
    for (value, str) in numbers {
        if input.starts_with(str) {
            return Some(value);
        }
    }
    None
}

pub fn part2_rayon(input: &str) -> u32 {
    input
        .par_lines()
        .map(|line| {
            let mut digits = (0..line.len())
                .map(|i| &line[i..])
                .filter_map(to_digit_part2);
            let first = digits
                .next()
                .expect("Line should contain at least one number");
            let last = digits.next_back().unwrap_or(first);
            (first * 10) + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(142)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(281)
    }
}

use aoc::Aoc;
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
            .lines()
            .map(|line| {
                let (_, values) = parsers::values(line)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
                predict_next::recursive_vec(&values)
            })
            .sum()
    }

    fn part2(input: &str) -> Self::OUTPUT {
        input
            .par_lines()
            .map(|line| {
                let (_, mut values) = parsers::values(line)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
                values.reverse();
                predict_next::recursive_vec(&values)
            })
            .sum()
    }
}

pub fn part1_inplace(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (_, values) = parsers::values(line)
                .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
            predict_next::inplace(values)
        })
        .sum()
}
pub fn part2_inplace(input: &str) -> i32 {
    input
        .par_lines()
        .map(|line| {
            let (_, mut values) = parsers::values(line)
                .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
            values.reverse();
            predict_next::inplace(values)
        })
        .sum()
}
pub fn part1_recursive_inplace(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (_, mut values) = parsers::values(line)
                .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
            predict_next::recursive_inplace(&mut values)
        })
        .sum()
}
pub fn part2_inplace_recursive(input: &str) -> i32 {
    input
        .par_lines()
        .map(|line| {
            let (_, mut values) = parsers::values(line)
                .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
            values.reverse();
            predict_next::recursive_inplace(&mut values)
        })
        .sum()
}

/// Different algorithms to solve the puzzle
mod predict_next {

    /// First naive attempt : stores the differences in a new vector and then recurse
    pub fn recursive_vec(values: &[i32]) -> i32 {
        let Some(last) = values.last().cloned() else {
            panic!("Failed to predict next number : diffs do not converge to zero")
        };

        let differences = values
            .windows(2)
            .map(|numbers| numbers[1] - numbers[0])
            .collect::<Vec<_>>();

        match differences.iter().all(|number| *number == 0) {
            true => last,
            false => last + recursive_vec(&differences),
        }
    }

    /// Reuses the same vector
    ///
    /// On each loop :
    /// - the last item is preserved
    /// - the n-1 items are replaced with their diff
    ///
    /// Once the diffs are all 0,
    /// - walk up the vector to add the successive diffs
    /// - the final answer is in the last item
    pub fn inplace(mut values: Vec<i32>) -> i32 {
        for last in (0..values.len() - 1).rev() {
            for i in 0..=last {
                values[i] = values[i + 1] - values[i];
            }

            if values[0..last].iter().all(|value| *value == 0) {
                return values[last..].iter().cloned().sum();
            }
        }
        panic!("Failed to predict next number : diffs do not converge to zero")
    }

    /// Reuses the same vector : recursive version
    ///
    /// On each recursion :
    /// - the last item is preserved
    /// - the n-1 items are replaced with their diff
    ///
    /// Once the diffs are all 0,
    /// - return the last item up the recursion chain
    /// - add it to the last item of the previous recursion and return it
    pub fn recursive_inplace(values: &mut [i32]) -> i32 {
        if values.iter().all(|n| *n == 0) {
            if values.is_empty() {
                panic!("Failed to predict next number : diffs do not converge to zero")
            }
            return 0;
        }

        let end = values.len() - 1;
        for i in 0..end {
            values[i] = values[i + 1] - values[i];
        }
        values[end] + recursive_inplace(&mut values[0..end])
    }
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
    fn test_in_place() {
        let input = Day::SAMPLE_PART1;
        assert_eq!(114, part1_inplace(input));

        let input = Day::INPUT;
        assert_eq!(1992273652, part1_inplace(input));

        let input = Day::SAMPLE_PART2;
        assert_eq!(2, part2_inplace(input));

        let input = Day::INPUT;
        assert_eq!(1012, part2_inplace(input));
    }

    #[test]
    fn test_in_place_recursive() {
        let input = Day::SAMPLE_PART1;
        assert_eq!(114, part1_recursive_inplace(input));

        let input = Day::INPUT;
        assert_eq!(1992273652, part1_recursive_inplace(input));

        let input = Day::SAMPLE_PART2;
        assert_eq!(2, part2_inplace_recursive(input));

        let input = Day::INPUT;
        assert_eq!(1012, part2_inplace_recursive(input));
    }

    #[test]
    #[should_panic]
    fn empty_in_place_recursive() {
        predict_next::recursive_inplace(&mut Vec::new());
    }

    #[test]
    fn test_part2() {
        Day::test_part2(2)
    }
}

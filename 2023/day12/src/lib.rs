use std::collections::{HashMap, VecDeque};

use aoc::Aoc;
use itertools::Itertools;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = usize;
    const DAY_NUMBER: u8 = 12;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        input
            .par_lines()
            .map(|line| {
                let (_, (springs, pattern)) = parsers::part1(line)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
                cached_permutations(&springs, &pattern, 0, &mut HashMap::new())
            })
            .sum()
    }

    fn part2(input: &str) -> Self::OUTPUT {
        input
            .par_lines()
            .map(|line| {
                let (_, (springs, pattern)) = parsers::part1(line)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));

                let springs = std::iter::repeat(springs).take(5);
                let springs = Itertools::intersperse(springs, vec![Spring::Unknown])
                    .flatten()
                    .collect_vec();

                let pattern = std::iter::repeat(pattern).take(5).flatten().collect_vec();

                cached_permutations(&springs, &pattern, 0, &mut HashMap::new())
            })
            .sum()
    }
}

/// For benching purposes
pub fn part1_brute_force(input: &str) -> usize {
    input
        .par_lines()
        .map(|line| {
            let (_, (springs, pattern)) = parsers::part1(line)
                .unwrap_or_else(|e| panic!("Parser failed {e:?} on line {line}"));
            brute_force_permutations(springs, &pattern)
        })
        .sum()
}

#[allow(dead_code)]
/// Works for Part1, but Part2 takes forever !
fn brute_force_permutations(springs: Vec<Spring>, pattern: &[u8]) -> usize {
    let mut count = 0;
    let mut arrangements = VecDeque::from([springs]);
    while let Some(mut springs) = arrangements.pop_front() {
        let mut broken_count = 0;
        let mut broken_index = 0;
        for spring_index in 0..springs.len() {
            if springs[spring_index] == Spring::Unknown {
                let mut copy = springs.clone();
                copy[spring_index] = Spring::Working;
                arrangements.push_front(copy);
                springs[spring_index] = Spring::Broken;
            }
            if springs[spring_index] == Spring::Broken {
                broken_count += 1;
                if broken_index == pattern.len() {
                    break;
                }
                if broken_count > pattern[broken_index] {
                    break;
                }
            }
            if springs[spring_index] == Spring::Working && broken_count != 0 {
                if broken_count != pattern[broken_index] {
                    break;
                }
                broken_index += 1;
                broken_count = 0;
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
    count
}

type Cache = HashMap<(usize, usize, u8), usize>;

/// Computes permutations recursively and stores intermediate values in a cache.
fn cached_permutations(springs: &[Spring], pattern: &[u8], count: u8, cache: &mut Cache) -> usize {
    if let Some(cached) = cache.get(&(springs.len(), pattern.len(), count)) {
        return *cached;
    }

    let permutations = match springs.first() {
        Some(Spring::Working) => case_working(springs, pattern, count, cache),
        Some(Spring::Broken) => case_broken(springs, pattern, count, cache),
        Some(Spring::Unknown) => {
            case_working(springs, pattern, count, cache)
                + case_broken(springs, pattern, count, cache)
        }
        // Finished the last spring with a Spring::Working
        None if pattern.is_empty() => 1,
        // Finished the last spring with a Spring::Broken
        None if pattern == [count] => 1,
        // Finished the last spring, but the pattern was not followed
        None => 0,
    };

    cache.insert((springs.len(), pattern.len(), count), permutations);
    permutations
}

fn case_broken(springs: &[Spring], pattern: &[u8], count: u8, cache: &mut Cache) -> usize {
    // Broken springs outside of allowed pattern
    let Some(max_count) = pattern.first() else {
        return 0;
    };
    // More broken springs than allowed in the pattern
    if *max_count <= count {
        return 0;
    }
    cached_permutations(&springs[1..], pattern, count + 1, cache)
}

fn case_working(springs: &[Spring], pattern: &[u8], count: u8, cache: &mut Cache) -> usize {
    // First working spring after `count` broken ones
    if pattern.first() == Some(&count) {
        return cached_permutations(&springs[1..], &pattern[1..], 0, cache);
    }
    // Multiple working springs in a row, or working springs at the start
    if count == 0 {
        return cached_permutations(&springs[1..], pattern, 0, cache);
    }
    return 0;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Working,
    Broken,
    Unknown,
}
impl TryFrom<char> for Spring {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '?' => Ok(Spring::Unknown),
            '#' => Ok(Spring::Broken),
            '.' => Ok(Spring::Working),
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
        let input = "???.### 1,1,3";
        assert_eq!(1, Day::part1(input), "1: {input}");

        let input = ".??..??...?##. 1,1,3";
        assert_eq!(4, Day::part1(input), "2: {input}");

        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(1, Day::part1(input), "3: {input}");

        let input = "????.#...#... 4,1,1";
        assert_eq!(1, Day::part1(input), "4: {input}");

        let input = "????.######..#####. 1,6,5";
        assert_eq!(4, Day::part1(input), "5: {input}");

        let input = "?###???????? 3,2,1";
        assert_eq!(10, Day::part1(input), "6: {input}");

        Day::test_part1(21)
    }

    #[test]
    fn test_part1_brute_force() {
        let input = "???.### 1,1,3";
        assert_eq!(1, part1_brute_force(input), "1: {input}");

        let input = ".??..??...?##. 1,1,3";
        assert_eq!(4, part1_brute_force(input), "2: {input}");

        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(1, part1_brute_force(input), "3: {input}");

        let input = "????.#...#... 4,1,1";
        assert_eq!(1, part1_brute_force(input), "4: {input}");

        let input = "????.######..#####. 1,6,5";
        assert_eq!(4, part1_brute_force(input), "5: {input}");

        let input = "?###???????? 3,2,1";
        assert_eq!(10, part1_brute_force(input), "6: {input}");

        let input = Day::SAMPLE_PART1;
        assert_eq!(21, part1_brute_force(input));
    }

    #[test]
    fn test_part2() {
        let input = "???.### 1,1,3";
        assert_eq!(1, Day::part2(input), "1: {input}");

        let input = ".??..??...?##. 1,1,3";
        assert_eq!(16384, Day::part2(input), "2: {input}");

        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(1, Day::part2(input), "3: {input}");

        let input = "????.#...#... 4,1,1";
        assert_eq!(16, Day::part2(input), "4: {input}");

        let input = "????.######..#####. 1,6,5";
        assert_eq!(2500, Day::part2(input), "5: {input}");

        let input = "?###???????? 3,2,1";
        assert_eq!(506250, Day::part2(input), "6: {input}");

        Day::test_part2(525152)
    }
}

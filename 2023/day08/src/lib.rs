use rayon::prelude::*;
use std::collections::HashMap;

use aoc::Aoc;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u64;
    const DAY_NUMBER: u8 = 8;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let (_, (directions, nodes)) =
            parsers::part1(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));

        let mut directions = directions.iter().cycle();
        let mut current_id = "AAA";
        let mut steps = 0;
        while let (Some(direction), Some(choice)) = (directions.next(), nodes.get(current_id)) {
            steps += 1;
            current_id = match direction {
                Direction::Left => choice[0].as_str(),
                Direction::Right => choice[1].as_str(),
            };
            if current_id == "ZZZ" {
                break;
            }
        }

        steps
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let (_, (directions, nodes)) =
            parsers::part1(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));

        let start_ids = nodes
            .keys()
            .filter_map(|id| id.ends_with('A').then_some(id.as_str()))
            .collect::<Vec<_>>();
        start_ids
            .into_par_iter()
            .map(|id| loop_length(id, &directions, &nodes))
            .reduce_with(lcm)
            .expect("There should be at least one start_id -> loop length")
    }
}

/// For Bench comparison : nodes stored in Hashmap of &str (single threaded)
pub fn part2_hash_str_singlethread(input: &str) -> u64 {
    let (_, (directions, nodes)) =
        parsers::part1_str(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));

    nodes
        .keys()
        .filter(|id| id.ends_with('A'))
        .map(|start_id| loop_length_str(start_id, &directions, &nodes))
        .reduce(lcm)
        .expect("There should be at least one start_id -> loop length")
}

/// For Bench comparison : nodes stored in Hashmap of &str (single threaded)
pub fn part2_hash_string_singlethread(input: &str) -> u64 {
    let (_, (directions, nodes)) =
        parsers::part1(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));

    nodes
        .keys()
        .filter(|id| id.ends_with('A'))
        .map(|start_id| loop_length(start_id, &directions, &nodes))
        .reduce(lcm)
        .expect("There should be at least one start_id -> loop length")
}

/// For Bench comparison : nodes stored in Hashmap of &str (multithreaded with rayon)
pub fn part2_hash_str(input: &str) -> u64 {
    let (_, (directions, nodes)) =
        parsers::part1_str(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));

    let start_ids = nodes
        .keys()
        .filter(|id| id.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    start_ids
        .into_par_iter()
        .map(|id| loop_length_str(id, &directions, &nodes))
        .reduce_with(lcm)
        .expect("There should be at least one start_id -> loop length")
}

/// Please don't try me...
#[allow(dead_code)]
pub fn part2_brute_force(input: &str) -> u64 {
    let (_, (directions, nodes)) =
        parsers::part1_str(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));

    let mut ids = nodes
        .keys()
        .filter_map(|id| id.ends_with('A').then_some(*id))
        .collect::<Vec<_>>();
    println!("Starting ids: {}\n{ids:?}", ids.len());

    for (step, direction) in directions.iter().cycle().enumerate() {
        for id in &mut ids {
            let choice = nodes.get(*id).unwrap();
            *id = match direction {
                Direction::Left => choice[0],
                Direction::Right => choice[1],
            };
        }
        if ids.iter().all(|id| id.ends_with('Z')) {
            return step as u64 + 1;
        }
        if step % 1_000_000 == 0 {
            println!("Step {} Million, ids: {ids:?}", step / 1_000_000);
        }
    }
    unreachable!("Loop should return directly");
}

fn loop_length(
    start_id: &str,
    directions: &[Direction],
    nodes: &HashMap<String, [String; 2]>,
) -> u64 {
    let mut id = start_id;
    let mut directions = directions.iter().cycle().enumerate();
    while let (Some((step, direction)), Some(choice)) = (directions.next(), nodes.get(id)) {
        id = match direction {
            Direction::Left => choice[0].as_str(),
            Direction::Right => choice[1].as_str(),
        };
        if id.ends_with('Z') {
            return step as u64 + 1;
        }
    }
    unreachable!("Loop should return directly");
}

fn loop_length_str(
    start_id: &str,
    directions: &[Direction],
    nodes: &HashMap<&str, [&str; 2]>,
) -> u64 {
    let mut id = start_id;
    let mut directions = directions.iter().cycle().enumerate();
    while let (Some((step, direction)), Some(choice)) = (directions.next(), nodes.get(id)) {
        id = match direction {
            Direction::Left => choice[0],
            Direction::Right => choice[1],
        };
        if id.ends_with('Z') {
            return step as u64 + 1;
        }
    }
    unreachable!("Loop should return directly");
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

fn gcd(a: u64, b: u64) -> u64 {
    match a % b {
        0 => b,
        c => gcd(b, c),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}
impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

mod parsers {
    use std::collections::HashMap;

    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, anychar, line_ending},
        multi::many1,
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use super::*;

    fn direction(input: &str) -> IResult<&str, Direction> {
        anychar.map_res(Direction::try_from).parse(input)
    }

    fn node(input: &str) -> IResult<&str, (String, [String; 2])> {
        let (input, id) = alpha1(input)?;
        let (input, _) = tag(" = (")(input)?;
        let (input, left) = alpha1(input)?;
        let (input, _) = tag(", ")(input)?;
        let (input, right) = alpha1(input)?;
        let (input, _) = tag(")")(input)?;

        Ok((input, (id.into(), [left.into(), right.into()])))
    }
    fn node_str(input: &str) -> IResult<&str, (&str, [&str; 2])> {
        let (input, id) = alpha1(input)?;
        let (input, _) = tag(" = (")(input)?;
        let (input, left) = alpha1(input)?;
        let (input, _) = tag(", ")(input)?;
        let (input, right) = alpha1(input)?;
        let (input, _) = tag(")")(input)?;

        Ok((input, (id, [left, right])))
    }

    type HashMapString = HashMap<String, [String; 2]>;
    type HashMapStr<'a> = HashMap<&'a str, [&'a str; 2]>;

    pub fn part1(input: &str) -> IResult<&str, (Vec<Direction>, HashMapString)> {
        let (input, directions) = many1(direction)(input)?;
        let (input, _) = line_ending(input)?;
        let (input, _) = line_ending(input)?;

        let mut nodes = HashMap::new();
        let mut input = input;
        while !input.is_empty() {
            let (remain, (id, directions)) = node(input)?;
            nodes.insert(id, directions);
            let (remain, _) = line_ending(remain)?;
            input = remain;
        }

        Ok((input, (directions, nodes)))
    }

    pub fn part1_str(input: &str) -> IResult<&str, (Vec<Direction>, HashMapStr)> {
        let (input, directions) = many1(direction)(input)?;
        let (input, _) = line_ending(input)?;
        let (input, _) = line_ending(input)?;

        let mut nodes = HashMap::new();
        let mut input = input;
        while !input.is_empty() {
            let (remain, (id, directions)) = node_str(input)?;
            nodes.insert(id, directions);
            let (remain, _) = line_ending(remain)?;
            input = remain;
        }

        Ok((input, (directions, nodes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Day::SAMPLE_PART1;
        assert_eq!(2, Day::part1(input));

        let input = include_str!("../inputs/sample1.1.txt");
        assert_eq!(6, Day::part1(input));
        Day::test_part1(2)
    }

    #[test]
    fn test_part2_brute() {
        let input = Day::SAMPLE_PART2;
        assert_eq!(6, part2_brute_force(input));
    }

    #[test]
    fn test_part2() {
        Day::test_part2(6)
    }

    #[test]
    fn test_part2_hash_str() {
        let input = Day::INPUT;
        assert_eq!(15746133679061, part2_hash_str(input))
    }

    #[test]
    fn test_part2_single() {
        let input = Day::INPUT;
        assert_eq!(15746133679061, part2_hash_str_singlethread(input))
    }
}

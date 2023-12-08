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
            .into_iter()
            .map(|id| loop_length(id, &directions, &nodes))
            .reduce(lcm)
            .expect("There should be at least one start_id -> loop length")
    }
}

/// Try me for fun...
#[allow(dead_code)]
fn brute_force(directions: &[Direction], nodes: &HashMap<String, [String; 2]>) -> u64 {
    let mut directions = directions.iter().cycle();
    let mut current_id = nodes
        .keys()
        .filter_map(|id| id.ends_with('A').then_some(id.as_str()))
        .collect::<Vec<_>>();
    println!("Starting ids: {}\n{current_id:?}", current_id.len());
    let mut steps = 0;
    while let Some(direction) = directions.next() {
        steps += 1;

        for i in 0..current_id.len() {
            let id = &mut current_id[i];
            let choice = nodes.get(*id).unwrap();
            *id = match direction {
                Direction::Left => choice[0].as_str(),
                Direction::Right => choice[1].as_str(),
            };
        }
        if current_id.iter().all(|id| id.ends_with('Z')) {
            break;
        }
        if steps % 1_000_000 == 0 {
            println!("Step {} Million, ids: {current_id:?}", steps / 1_000_000);
        }
    }

    steps
}

fn loop_length(
    start_id: &str,
    directions: &[Direction],
    nodes: &HashMap<String, [String; 2]>,
) -> u64 {
    let mut directions = directions.iter().cycle();
    let mut current_id = start_id;
    let mut steps = 0;
    while let (Some(direction), Some(choice)) = (directions.next(), nodes.get(current_id)) {
        steps += 1;
        current_id = match direction {
            Direction::Left => choice[0].as_str(),
            Direction::Right => choice[1].as_str(),
        };
        if current_id.ends_with('Z') {
            break;
        }
    }

    steps
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
enum Direction {
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

    pub fn part1(input: &str) -> IResult<&str, (Vec<Direction>, HashMap<String, [String; 2]>)> {
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
    fn test_part2() {
        Day::test_part2(6)
    }
}

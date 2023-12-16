use std::collections::VecDeque;

use aoc::Aoc;
use bitflags::bitflags;
use itertools::Itertools;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = usize;
    const DAY_NUMBER: u8 = 16;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let (_, mirrors) =
            parsers::mirror_map(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));
        let start = (Direction::East, (0, 0));
        compute_path(start, &mirrors)
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let (_, mirrors) =
            parsers::mirror_map(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));
        let height = mirrors.len();
        let width = mirrors[0].len();
        starting_positions(height, width)
            .par_bridge()
            .map(|start| compute_path(start, &mirrors))
            .max()
            .unwrap()
    }
}

fn compute_path(start: (Direction, (usize, usize)), mirrors: &Vec<Vec<Option<Mirror>>>) -> usize {
    let height = mirrors.len();
    let width = mirrors[0].len();
    let mut energized = (0..height)
        .map(|_| (0..width).map(|_| Direction::empty()).collect_vec())
        .collect_vec();
    let mut beams = VecDeque::from([start]);
    while let Some((direction, (x, y))) = beams.pop_front() {
        let tile = &mut energized[y][x];
        if tile.contains(direction) {
            continue;
        };
        tile.insert(direction);
        let mirror = mirrors[y][x];
        let next = match mirror {
            Some(mirror) => direction.bounce(mirror),
            None => vec![direction],
        };
        for direction in next {
            let Some((x, y)) = direction
                .next_coords((x, y))
                .filter(|(x, y)| *x < width && *y < height)
            else {
                continue;
            };
            beams.push_back((direction, (x, y)))
        }
    }
    energized
        .iter()
        .flat_map(|line| line.iter().filter(|tile| !tile.is_empty()))
        .count()
}

#[allow(dead_code)]
fn debug_energized(grid: &Vec<Vec<Vec<Direction>>>) {
    let data = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|tile| if tile.is_empty() { '.' } else { '#' })
                .collect::<String>()
        })
        .join("\n");
    println!("{data}")
}

fn starting_positions(
    height: usize,
    width: usize,
) -> impl Iterator<Item = (Direction, (usize, usize))> {
    let top = (0..width).map(move |x| (Direction::South, (x, 0)));
    let bottom = (0..width).map(move |x| (Direction::North, (x, height - 1)));
    let left = (0..height).map(move |y| (Direction::East, (0, y)));
    let right = (0..height).map(move |y| (Direction::West, (width - 1, y)));
    top.chain(bottom).chain(left).chain(right)
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Copy,Clone)]
    struct Direction: u8 {
        const North = 0b0001;
        const South = 0b0010;
        const East =  0b0100;
        const West =  0b1000;
    }
}

impl Direction {
    fn next_coords(self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::North if y != 0 => Some((x, y - 1)),
            Direction::South => Some((x, y + 1)),
            Direction::East => Some((x + 1, y)),
            Direction::West if x != 0 => Some((x - 1, y)),
            _ => None,
        }
    }
    fn bounce(self, mirror: Mirror) -> Vec<Direction> {
        match (self, mirror) {
            (Direction::South, Mirror::SplitVertical)
            | (Direction::West, Mirror::SplitHorizontal)
            | (Direction::East, Mirror::SplitHorizontal)
            | (Direction::North, Mirror::SplitVertical) => vec![self],
            (Direction::South, Mirror::SplitHorizontal)
            | (Direction::North, Mirror::SplitHorizontal) => vec![Direction::West, Direction::East],
            (Direction::South, Mirror::SouthEast) | (Direction::North, Mirror::NorthEast) => {
                vec![Direction::East]
            }
            (Direction::South, Mirror::NorthEast) | (Direction::North, Mirror::SouthEast) => {
                vec![Direction::West]
            }
            (Direction::West, Mirror::SplitVertical) | (Direction::East, Mirror::SplitVertical) => {
                vec![Direction::North, Direction::South]
            }
            (Direction::West, Mirror::SouthEast) | (Direction::East, Mirror::NorthEast) => {
                vec![Direction::North]
            }
            (Direction::West, Mirror::NorthEast) | (Direction::East, Mirror::SouthEast) => {
                vec![Direction::South]
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mirror {
    SplitVertical,
    SplitHorizontal,
    NorthEast,
    SouthEast,
}

mod parsers {
    use nom::{
        character::complete::{anychar, line_ending},
        multi::{many1, separated_list1},
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use super::*;

    pub fn mirror_map(input: &str) -> IResult<&str, Vec<Vec<Option<Mirror>>>> {
        separated_list1(line_ending, many1(mirror))(input)
    }

    fn mirror(input: &str) -> IResult<&str, Option<Mirror>> {
        anychar
            .map_res(|c: char| match c {
                '.' => Ok(None),
                '/' => Ok(Some(Mirror::NorthEast)),
                '\\' => Ok(Some(Mirror::SouthEast)),
                '|' => Ok(Some(Mirror::SplitVertical)),
                '-' => Ok(Some(Mirror::SplitHorizontal)),
                _ => Err(()),
            })
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(46)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(51)
    }
}

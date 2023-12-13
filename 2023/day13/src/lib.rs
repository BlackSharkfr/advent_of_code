use aoc::Aoc;
use itertools::Itertools;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 13;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        input
            .split("\r\n\r\n")
            .par_bridge()
            .map(|str| {
                let (_, grid) = parsers::part1(str)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on input {str}"));
                Symmetry::find_symmetry_part1(&grid).to_number()
            })
            .sum()
    }

    fn part2(input: &str) -> Self::OUTPUT {
        input
            .split("\r\n\r\n")
            .par_bridge()
            .map(|str| {
                let (_, grid) = parsers::part1(str)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on input {str}"));
                Symmetry::find_symmetry_part2(&grid).to_number()
            })
            .sum()
    }
}

/// For Benching purposes
pub mod single_thread {
    use super::*;
    pub fn part1(input: &str) -> u32 {
        input
            .split("\r\n\r\n")
            .map(|str| {
                let (_, grid) = parsers::part1(str)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on input {str}"));
                Symmetry::find_symmetry_part1(&grid).to_number()
            })
            .sum()
    }

    pub fn part2(input: &str) -> u32 {
        input
            .split("\r\n\r\n")
            .map(|str| {
                let (_, grid) = parsers::part1(str)
                    .unwrap_or_else(|e| panic!("Parser failed {e:?} on input {str}"));
                Symmetry::find_symmetry_part2(&grid).to_number()
            })
            .sum()
    }
}

/// Caution with the puzzle text :
/// - A Horizontal line of reflection == Vertical Symmetry  
/// - A Vertical line of reflection   ==  Horizontal Symmetry
#[derive(Debug)]
enum Symmetry {
    Vertical(usize),
    Horizontal(usize),
}
impl Symmetry {
    fn to_number(self) -> u32 {
        match self {
            Self::Horizontal(n) => n as u32,
            Self::Vertical(n) => 100 * n as u32,
        }
    }

    fn find_symmetry_part1(input: &Vec<Vec<Tile>>) -> Self {
        let height = input.len();
        let width = input[0].len();

        // Vertical symmetry
        if let Some(y) = (0..height - 1).position(|y| {
            (0..width).all(|x| {
                let above_mirror = (0..=y).rev().map(|i| &input[i][x]);
                let below_mirror = (y + 1..height).map(|i| &input[i][x]);
                above_mirror.zip(below_mirror).all(|(a, b)| a == b)
            })
        }) {
            return Self::Vertical(y + 1);
        }

        // Horizontal symmetry
        if let Some(x) = (0..width - 1).position(|x| {
            (0..height).all(|y| {
                let left_of_mirror = (0..=x).rev().map(|i| &input[y][i]);
                let right_of_mirror = (x + 1..width).map(|i| &input[y][i]);
                left_of_mirror.zip(right_of_mirror).all(|(l, r)| l == r)
            })
        }) {
            return Self::Horizontal(x + 1);
        }

        panic!("Symmetry not found : \n{}", vec2d_to_string(input),)
    }

    fn find_symmetry_part2(input: &Vec<Vec<Tile>>) -> Self {
        let height = input.len();
        let width = input[0].len();

        // Vertical symmetry
        if let Some(y) = (0..height - 1).position(|y| {
            (0..width)
                .map(|x| {
                    let above_mirror = (0..=y).rev().map(|i| &input[i][x]);
                    let below_mirror = (y + 1..height).map(|i| &input[i][x]);
                    above_mirror
                        .zip(below_mirror)
                        .filter(|(a, b)| a != b)
                        .count()
                })
                .sum::<usize>()
                == 1
        }) {
            return Self::Vertical(y + 1);
        }

        // Horizontal symmetry
        if let Some(x) = (0..width - 1).position(|x| {
            (0..height)
                .map(|y| {
                    let left_of_mirror = (0..=x).rev().map(|i| &input[y][i]);
                    let right_of_mirror = (x + 1..width).map(|i| &input[y][i]);
                    left_of_mirror
                        .zip(right_of_mirror)
                        .filter(|(l, r)| l != r)
                        .count()
                })
                .sum::<usize>()
                == 1
        }) {
            return Self::Horizontal(x + 1);
        }

        panic!("Symmetry not found : \n{}", vec2d_to_string(input),)
    }
}

/// Prints the grid for debugging
fn vec2d_to_string(input: &Vec<Vec<Tile>>) -> String {
    input.iter().map(|line| format!("{line:?}")).join("\n")
}

#[derive(Debug, PartialEq)]
enum Tile {
    Ground,
    Mirror,
}
impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Tile::Mirror),
            '.' => Ok(Tile::Ground),
            _ => Err(()),
        }
    }
}

mod parsers {
    use nom::{
        character::complete::{anychar, line_ending},
        multi::{many1, separated_list1},
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use super::*;

    pub fn part1(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
        separated_list1(line_ending, line)(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<Tile>> {
        many1(tile)(input)
    }

    fn tile(input: &str) -> IResult<&str, Tile> {
        anychar.map_res(Tile::try_from).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../inputs/sample1.1.txt");
        assert_eq!(5, Day::part1(input));

        let input = include_str!("../inputs/sample1.2.txt");
        assert_eq!(400, Day::part1(input));

        Day::test_part1(405);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../inputs/sample1.1.txt");
        assert_eq!(300, Day::part2(input));

        let input = include_str!("../inputs/sample1.2.txt");
        assert_eq!(100, Day::part2(input));

        let input = "...##.#
.##.###
.##.###
...##.#
#...###
.#..##.
##.##.#
...####
#.#.###
#....#.
..#.##.
..#.##.
#....#.
#.#.#.#
...####";
        assert_eq!(1100, Day::part2(input));

        Day::test_part2(400)
    }
}

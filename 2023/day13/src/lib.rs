use aoc::Aoc;
use itertools::Itertools;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 13;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let Ok((_, grids)) = parsers::part1(input) else {
            panic!("Parser failed")
        };
        grids
            .iter()
            .map(Symmetry::find_symmetry_part1)
            .map(Symmetry::to_number)
            .sum()
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let Ok((_, grids)) = parsers::part1(input) else {
            panic!("Parser failed")
        };
        grids
            .iter()
            .map(Symmetry::find_symmetry_part2)
            .map(Symmetry::to_number)
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
        // println!("Symmetry found : {self:?}");
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
                (0..=y)
                    .filter(|i| i + y + 1 < height)
                    .all(|i| input[y - i][x] == input[y + 1 + i][x])
            })
        }) {
            return Self::Vertical(y + 1);
        }

        // Horizontal
        if let Some(x) = (0..width - 1).position(|x| {
            input.iter().all(|line| {
                (0..=x)
                    .filter(|i| i + x + 1 < width)
                    .all(|i| line[x - i] == line[x + 1 + i])
            })
        }) {
            return Self::Horizontal(x + 1);
        }

        // Error
        panic!(
            "Symmetry not found : \n{}",
            input.iter().map(|line| format!("{line:?}")).join("\n")
        )
    }

    fn find_symmetry_part2(input: &Vec<Vec<Tile>>) -> Self {
        let height = input.len();
        let width = input[0].len();

        // Vertical symmetry
        if let Some(y) = (0..height - 1).position(|y| {
            (0..width)
                .map(|x| {
                    (0..=y)
                        .filter(|i| i + y + 1 < height)
                        .filter(|i| input[y - i][x] != input[y + 1 + i][x])
                        .count()
                })
                .sum::<usize>()
                == 1
        }) {
            return Self::Vertical(y + 1);
        }

        // Horizontal
        if let Some(x) = (0..width - 1).position(|x| {
            input
                .iter()
                .map(|line| {
                    (0..=x)
                        .filter(|i| i + x + 1 < width)
                        .filter(|i| line[x - i] != line[x + 1 + i])
                        .count()
                })
                .sum::<usize>()
                == 1
        }) {
            return Self::Horizontal(x + 1);
        }

        // Error
        panic!(
            "Symmetry not found : \n{}",
            input.iter().map(|line| format!("{line:?}")).join("\n")
        )
    }
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

    pub fn part1(input: &str) -> IResult<&str, Vec<Vec<Vec<Tile>>>> {
        separated_list1(line_ending, grid)(input)
    }

    fn grid(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
        many1(line)(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<Tile>> {
        many1(tile).terminated(line_ending).parse(input)
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

use std::str::FromStr;

use aoc::Aoc;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 21;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let (grid, start) = parsers::part1(input);
        let height = grid.len();
        let width = grid[0].len();
        let mut previous = Vec::new();
        let mut current = vec![start];

        let range = if cfg!(test) { 6 } else { 64 };
        for step in 1..=range {
            previous = std::mem::take(&mut current);
            for (x, y) in &previous {
                let directions = Directions::All
                    .into_iter()
                    .filter_map(|direction| direction.movement((*x, *y), (width, height)));
                for (x, y) in directions {
                    if grid[y][x] != Tile::Rock {
                        current.push((x, y));
                    }
                }
            }
            current.sort();
            current.dedup();
        }
        current.len() as u32
    }

    fn part2(_input: &str) -> Self::OUTPUT {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Rock,
    Garden,
    Start,
}
impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Rock),
            "." => Ok(Self::Garden),
            "S" => Ok(Self::Start),
            _ => Err(()),
        }
    }
}
impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Self::Rock),
            '.' => Ok(Self::Garden),
            'S' => Ok(Self::Start),
            _ => Err(()),
        }
    }
}

enum Directions {
    North,
    South,
    East,
    West,
}
impl Directions {
    const All: [Self; 4] = [
        Directions::North,
        Directions::South,
        Directions::East,
        Directions::West,
    ];

    fn movement(
        self,
        (x, y): (usize, usize),
        (width, height): (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Directions::North if y != 0 => Some((x, y - 1)),
            Directions::South if y + 1 != height => Some((x, y + 1)),
            Directions::East if x + 1 != width => Some((x + 1, y)),
            Directions::West if x != 0 => Some((x - 1, y)),
            _ => None,
        }
    }
}

mod parsers {
    use super::*;

    pub fn part1(input: &str) -> (Vec<Vec<Tile>>, (usize, usize)) {
        let mut start = None;
        let grid = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.char_indices()
                    .map(|(x, c)| {
                        let t = Tile::try_from(c).unwrap();
                        if t == Tile::Start {
                            start = Some((x, y));
                        }
                        t
                    })
                    .collect()
            })
            .collect();
        (grid, start.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(16)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(0)
    }
}

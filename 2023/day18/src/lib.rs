use aoc::Aoc;
use itertools::Itertools;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u64;
    const DAY_NUMBER: u8 = 18;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let edges = parsers::part1(input);

        let (mut x, mut y) = (0, 0);
        let (mut xmin, mut xmax, mut ymin, mut ymax) = (0, 0, 0, 0);
        for edge in &edges {
            (x, y) = edge.direction.next_coordinates((x, y), edge.amount as i64);
            xmin = xmin.min(x);
            xmax = xmax.max(x);
            ymin = ymin.min(y);
            ymax = ymax.max(y);
        }

        let width = xmax - xmin;
        let height = ymax - ymin;
        let mut grid = (0..=height)
            .map(|_| (0..=width).map(|_| Option::<Dig>::None).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (mut x, mut y) = (-xmin, -ymin);
        for (edge, next_edge) in edges.iter().circular_tuple_windows() {
            for _ in 1..edge.amount {
                (x, y) = edge.direction.next_coordinates((x, y), 1);
                grid[y as usize][x as usize] = Some(Dig::from(edge.direction))
            }
            (x, y) = edge.direction.next_coordinates((x, y), 1);
            grid[y as usize][x as usize] = Some(Dig::from((edge.direction, next_edge.direction)))
        }

        let mut count = 0;
        for line in &grid {
            let mut inside = false;
            for square in line {
                match square {
                    None => {
                        if inside {
                            count += 1;
                        }
                    }
                    Some(dig) => {
                        count += 1;
                        match dig {
                            Dig::Vertical | Dig::CornerNW | Dig::CornerNE => inside = !inside,
                            Dig::Horizontal | Dig::CornerSW | Dig::CornerSE => (),
                        }
                    }
                }
            }
        }

        count
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let edges = parsers::part2(input);

        let (mut x, mut y) = (0, 0);
        let coords = std::iter::once((0, 0))
            .chain(edges.iter().map(|edge| {
                (x, y) = edge.direction.next_coordinates((x, y), edge.amount as i64);
                (x, y)
            }))
            .collect::<Vec<_>>();

        let area = coords
            .iter()
            .circular_tuple_windows()
            .map(|((x1, y1), (x2, y2))| (y1 + y2) * (x1 - x2))
            .sum::<i64>()
            / 2;
        let perimeter = edges.iter().map(|edge| edge.amount).sum::<u32>();
        area.unsigned_abs() + (perimeter as u64 / 2) + 1
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}
impl Direction {
    fn next_coordinates(self, (x, y): (i64, i64), amount: i64) -> (i64, i64) {
        match self {
            Direction::Up => (x, y - amount),
            Direction::Down => (x, y + amount),
            Direction::Left => (x - amount, y),
            Direction::Right => (x + amount, y),
        }
    }

    fn from_digit_part2(value: &str) -> Result<Self, ()> {
        match value {
            "0" => Ok(Direction::Right),
            "1" => Ok(Direction::Down),
            "2" => Ok(Direction::Left),
            "3" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

struct Edge {
    direction: Direction,
    amount: u32,
}

#[derive(Debug, Clone)]
enum Dig {
    Vertical,
    Horizontal,
    CornerNE,
    CornerNW,
    CornerSE,
    CornerSW,
}
impl From<Direction> for Dig {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up | Direction::Down => Dig::Vertical,
            Direction::Left | Direction::Right => Dig::Horizontal,
        }
    }
}
impl From<(Direction, Direction)> for Dig {
    fn from((previous, next): (Direction, Direction)) -> Self {
        match (previous, next) {
            (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => Dig::CornerSW,
            (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => Dig::CornerSE,
            (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => Dig::CornerNW,
            (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => Dig::CornerNE,
            (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => Dig::Vertical,
            (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => {
                Dig::Horizontal
            }
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => {
                panic!("invalid sequence of directions {:?} {:?}", previous, next)
            }
        }
    }
}

mod parsers {
    use nom::{
        bytes::complete::{tag, take, take_until, take_until1},
        character::complete::{anychar, line_ending, space1, u32},
        multi::separated_list1,
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use super::*;

    fn direction(input: &str) -> IResult<&str, Direction> {
        anychar.map_res(Direction::try_from).parse(input)
    }

    fn edge_part2(input: &str) -> IResult<&str, Edge> {
        let (input, _) = take_until1("#")(input)?;
        let (input, _) = tag("#")(input)?;
        let (input, amount) = take(5_usize)
            .map_res(|s| u32::from_str_radix(s, 16))
            .parse(input)?;
        let (input, direction) = take(1_usize)
            .map_res(|s| Direction::from_digit_part2(s))
            .parse(input)?;
        let (input, _) = tag(")")(input)?;

        let edge = Edge { direction, amount };
        Ok((input, edge))
    }

    pub fn edge_part1(input: &str) -> IResult<&str, Edge> {
        let (input, direction) = direction(input)?;
        let (input, _) = space1(input)?;
        let (input, amount) = u32(input)?;
        let (input, _) = take_until("\n")(input)?;

        let edge = Edge { direction, amount };
        Ok((input, edge))
    }

    pub fn part1(input: &str) -> Vec<Edge> {
        separated_list1(line_ending, edge_part1)(input).unwrap().1
    }

    pub fn part2(input: &str) -> Vec<Edge> {
        separated_list1(line_ending, edge_part2)(input).unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(62)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(952408144115)
    }
}

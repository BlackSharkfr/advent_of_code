use aoc::Aoc;
use itertools::Itertools;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 18;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let edges = parsers::part1(input);

        let (mut x, mut y) = (0, 0);
        let (mut xmin, mut xmax, mut ymin, mut ymax) = (0, 0, 0, 0);
        for edge in &edges {
            (x, y) = edge.direction.next_coordinates((x, y), edge.amount as i32);
            xmin = xmin.min(x);
            xmax = xmax.max(x);
            ymin = ymin.min(y);
            ymax = ymax.max(y);
        }
        println!(
            "Final coordinates : ({},{}), x range : [{},{}], y range : [{},{}]",
            x, y, xmin, xmax, ymin, ymax
        );

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

    fn part2(_input: &str) -> Self::OUTPUT {
        todo!()
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
    fn next_coordinates(self, (x, y): (i32, i32), amount: i32) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - amount as i32),
            Direction::Down => (x, y + amount as i32),
            Direction::Left => (x - amount as i32, y),
            Direction::Right => (x + amount as i32, y),
        }
    }
}

struct Color(String);

struct Edge {
    direction: Direction,
    amount: u8,
    color: Color,
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
        bytes::complete::tag,
        character::complete::{alphanumeric1, anychar, line_ending, space1, u8},
        multi::separated_list1,
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use super::*;

    fn direction(input: &str) -> IResult<&str, Direction> {
        anychar.map_res(Direction::try_from).parse(input)
    }

    fn color(input: &str) -> IResult<&str, Color> {
        let (input, _) = tag("(#")(input)?;
        let (input, color) = alphanumeric1(input)?;
        let (input, _) = tag(")")(input)?;

        Ok((input, Color(color.to_string())))
    }

    pub fn edge(input: &str) -> IResult<&str, Edge> {
        let (input, direction) = direction(input)?;
        let (input, _) = space1(input)?;
        let (input, amount) = u8(input)?;
        let (input, _) = space1(input)?;
        let (input, color) = color(input)?;

        let edge = Edge {
            direction,
            amount,
            color,
        };
        Ok((input, edge))
    }

    pub fn part1(input: &str) -> Vec<Edge> {
        separated_list1(line_ending, edge)(input).unwrap().1
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
        Day::test_part2(0)
    }
}

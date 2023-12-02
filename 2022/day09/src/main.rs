use std::{
    collections::HashSet,
    ops::{AddAssign, Deref, DerefMut, Sub, SubAssign},
    str::FromStr,
};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 : {}", part1(input));
    println!("Part 2 : {}", part2(input));
}

fn part1(input: &str) -> usize {
    let moves = parse_moves(input);

    let mut rope = Rope(vec![Position2D::default(); 2]);
    let mut positions = HashSet::<Position2D>::from([Position2D::default()]);

    for direction in moves.into_iter() {
        rope.pull(direction);
        positions.insert(rope.last().unwrap().clone());
    }

    positions.len()
}

fn part2(input: &str) -> usize {
    let moves = parse_moves(input);

    let mut rope = Rope(vec![Position2D::default(); 10]);
    let mut positions = HashSet::<Position2D>::from([Position2D::default()]);

    for direction in moves.into_iter() {
        rope.pull(direction);
        positions.insert(rope.last().unwrap().clone());
    }

    positions.len()
}

fn parse_moves(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .flat_map(|movement| (0..movement.amount).map(move |_| movement.direction))
        .collect()
}

struct Move {
    amount: u8,
    direction: Direction,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let direction = words
            .next()
            .ok_or("First word missing")?
            .parse::<Direction>()?;
        let amount = words
            .next()
            .ok_or("Second word missing")?
            .parse::<u8>()
            .map_err(|e| format!("Amount incorrect. Expected a u8 '{e}'"))?;
        Ok(Move { amount, direction })
    }
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(format!(
                "Direction incorrect. Expected U | D | R | L. Found '{s}'"
            )),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position2D {
    x: isize,
    y: isize,
}
impl Default for Position2D {
    fn default() -> Self {
        Position2D { x: 0, y: 0 }
    }
}
impl AddAssign for Position2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct Rope(Vec<Position2D>);
impl Deref for Rope {
    type Target = Vec<Position2D>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Rope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Rope {
    fn pull(&mut self, direction: Direction) {
        self[0] += direction;
        for i in 0..self.len() - 1 {
            let dp = self[i].clone() - self[i + 1].clone();
            match (dp.x.abs(), dp.y.abs()) {
                (0, 0) | (0, 1) | (1, 1) | (1, 0) => (),
                (2, 0) | (0, 2) | (2, 2) => {
                    self[i + 1] += Position2D {
                        x: dp.x / 2,
                        y: dp.y / 2,
                    };
                }
                (2, 1) => {
                    self[i + 1] += Position2D {
                        x: dp.x / 2,
                        y: dp.y,
                    }
                }
                (1, 2) => {
                    self[i + 1] += Position2D {
                        x: dp.x,
                        y: dp.y / 2,
                    }
                }
                _ => panic!("Invalid movement ({}, {})", dp.x, dp.y),
            }
        }
    }
}

impl AddAssign<Direction> for Position2D {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}
impl SubAssign<Direction> for Position2D {
    fn sub_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x += 1,
            Direction::Right => self.x -= 1,
        }
    }
}
impl Sub for Position2D {
    type Output = Position2D;
    fn sub(self, rhs: Self) -> Self::Output {
        Position2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = include_str!("sample.txt");
    static SAMPLE2: &str = include_str!("sample2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE2), 36)
    }
}

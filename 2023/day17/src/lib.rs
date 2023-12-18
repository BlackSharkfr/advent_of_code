use std::collections::{BinaryHeap, HashSet};

use aoc::Aoc;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 17;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let heatmap = parsers::heat_map(input);
        dijkstra(&heatmap, 1, 3)
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let heatmap = parsers::heat_map(input);
        dijkstra(&heatmap, 4, 10)
    }
}

fn dijkstra(heatmap: &Vec<Vec<u8>>, min_moves: u8, max_moves: u8) -> u32 {
    let height = heatmap.len();
    let width = heatmap[0].len();
    let start_right = Crucible {
        direction: Direction::East,
        ..Default::default()
    };
    let start_down = Crucible {
        direction: Direction::South,
        ..Default::default()
    };
    let mut queue = BinaryHeap::from([(start_right), (start_down)]);
    let mut visited = HashSet::new();
    loop {
        let crucible = queue.pop().expect("Queue is empty");
        if crucible.x == width - 1 && crucible.y == height - 1 && crucible.moves >= min_moves {
            return crucible.total_heat_loss;
        }
        if !visited.insert(crucible.state()) {
            continue;
        }

        let next_moves = crucible
            .next_directions(min_moves, max_moves)
            .map(|c| c.move_forwards(width, height))
            .flatten();
        for mut crucible in next_moves {
            crucible.total_heat_loss += heatmap[crucible.y][crucible.x] as u32;
            queue.push(crucible);
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Crucible {
    x: usize,
    y: usize,
    total_heat_loss: u32,
    direction: Direction,
    moves: u8,
}
impl Crucible {
    pub fn move_forwards(mut self, width: usize, height: usize) -> Option<Self> {
        match self.direction {
            Direction::North => self.y = self.y.checked_sub(1)?,
            Direction::East if self.x != width - 1 => self.x += 1,
            Direction::South if self.y != height - 1 => self.y += 1,
            Direction::West => self.x = self.x.checked_sub(1)?,
            _ => return None,
        };
        self.moves += 1;
        Some(self)
    }
    pub fn next_directions(self, min_moves: u8, max_moves: u8) -> impl Iterator<Item = Self> {
        let left = (min_moves <= self.moves).then_some(Self {
            direction: self.direction.turn_left(),
            moves: 0,
            ..self
        });

        let right = (min_moves <= self.moves).then_some(Self {
            direction: self.direction.turn_right(),
            moves: 0,
            ..self
        });

        let straight = (self.moves < max_moves).then_some(self);

        [straight, left, right].into_iter().flatten()
    }
    fn state(&self) -> (usize, usize, Direction, u8) {
        (self.x, self.y, self.direction, self.moves)
    }
}
impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.total_heat_loss.cmp(&self.total_heat_loss)
    }
}
impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    #[default]
    East,
    South,
    West,
}
impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

pub mod parsers {
    pub fn heat_map(input: &str) -> Vec<Vec<u8>> {
        let mut heatmap = Vec::new();
        let mut line = Vec::new();
        for c in input.chars() {
            match c.to_digit(10) {
                Some(n) => line.push(n as u8),
                None => {
                    if !line.is_empty() {
                        heatmap.push(line);
                        line = Vec::with_capacity(heatmap[0].len())
                    }
                }
            }
        }
        if !line.is_empty() {
            heatmap.push(line);
        }
        heatmap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(102)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(94)
    }
}

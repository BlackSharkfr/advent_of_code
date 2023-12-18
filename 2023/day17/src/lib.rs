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

fn dijkstra(heatmap: &Vec<Vec<u32>>, min_moves: u8, max_moves: u8) -> u32 {
    let height = heatmap.len() as u8;
    let width = heatmap[0].len() as u8;
    let end = (width - 1, height - 1);

    let mut queue = BinaryHeap::from([HeapState(Crucible::default(), 0)]);
    let mut visited = HashSet::new();
    loop {
        let HeapState(crucible, heat) = queue.pop().expect("Queue has run out of nodes");
        for crucible in crucible.next_moves(min_moves, max_moves, width, height) {
            if !visited.insert(crucible.visited_state()) {
                continue;
            }
            let heat = heat + (heatmap[crucible.y as usize][crucible.x as usize]);
            if (crucible.x, crucible.y) == end && crucible.moves >= min_moves {
                return heat;
            }
            queue.push(HeapState(crucible, heat));
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HeapState(Crucible, u32);
impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HeapState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.1.cmp(&other.1)).reverse()
    }
}

/// For benchmark purposes, comparison with the [`pathfinding`] crate
pub mod using_pathfinding {

    use crate::Crucible;

    pub fn dijkstra(heatmap: &Vec<Vec<u32>>, min_moves: u8, max_moves: u8) -> u32 {
        let start = Crucible::default();
        let height = heatmap.len();
        let width = heatmap[0].len();
        let successors = |c: &Crucible| {
            c.clone()
                .next_moves(min_moves, max_moves, width as u8, height as u8)
                .map(|c| {
                    let x = c.x as usize;
                    let y = c.y as usize;
                    (c, heatmap[y][x])
                })
        };
        let success = |c: &Crucible| {
            c.x == width as u8 - 1 && c.y == height as u8 - 1 && c.moves >= min_moves
        };
        let res = pathfinding::directed::dijkstra::dijkstra(&start, successors, success);
        res.unwrap().1
    }

    pub fn astar(heatmap: &Vec<Vec<u32>>, min_moves: u8, max_moves: u8) -> u32 {
        let start = Crucible::default();
        let height = heatmap.len();
        let width = heatmap[0].len();
        let successors = |c: &Crucible| {
            c.clone()
                .next_moves(min_moves, max_moves, width as u8, height as u8)
                .map(|c| {
                    let x = c.x as usize;
                    let y = c.y as usize;
                    (c, heatmap[y][x])
                })
        };
        let success = |c: &Crucible| {
            c.x == width as u8 - 1 && c.y == height as u8 - 1 && c.moves >= min_moves
        };
        let heuristic_map = {
            let mut map = heatmap.clone();
            for y in (0..height).rev() {
                for x in (0..width).rev() {
                    if x == width - 1 && y == height - 1 {
                        continue;
                    }
                    let below = if y != height - 1 {
                        heatmap[y + 1][x]
                    } else {
                        u32::MAX
                    };
                    let right = if x != width - 1 {
                        heatmap[y][x + 1]
                    } else {
                        u32::MAX
                    };
                    map[y][x] += below.min(right);
                }
            }
            map
        };
        let heuristic = |c: &Crucible| {
            let x = c.x as usize;
            let y = c.y as usize;
            heuristic_map[y][x]
        };
        let res = pathfinding::directed::astar::astar(&start, successors, heuristic, success);
        res.unwrap().1
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Crucible {
    x: u8,
    y: u8,
    direction: Direction,
    moves: u8,
}

impl Crucible {
    fn visited_state(&self) -> u32 {
        (self.x as u32)
            | (self.y as u32) << 8
            | (self.moves as u32) << 16
            | (self.direction as u32) << 24
    }
    fn move_forwards(mut self, width: u8, height: u8) -> Option<Self> {
        match self.direction {
            Direction::North if self.y != 0 => self.y -= 1,
            Direction::East if self.x != width - 1 => self.x += 1,
            Direction::South if self.y != height - 1 => self.y += 1,
            Direction::West if self.x != 0 => self.x -= 1,
            _ => return None,
        };
        self.moves += 1;
        Some(self)
    }
    fn next_directions(self, min_moves: u8, max_moves: u8) -> impl Iterator<Item = Self> {
        if self.moves == 0 {
            let east = Self {
                direction: Direction::East,
                ..self
            };
            let south = Self {
                direction: Direction::South,
                ..self
            };
            return [Some(east), Some(south), None].into_iter().flatten();
        }
        let left = (min_moves <= self.moves).then_some(self.turn_left());
        let right = (min_moves <= self.moves).then_some(self.turn_right());
        let straight = (self.moves < max_moves).then_some(self);

        [straight, left, right].into_iter().flatten()
    }
    fn next_moves(
        self,
        min_moves: u8,
        max_moves: u8,
        width: u8,
        height: u8,
    ) -> impl Iterator<Item = Self> {
        self.next_directions(min_moves, max_moves)
            .filter_map(move |c| c.move_forwards(width, height))
    }
    fn turn_left(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            direction: self.direction.turn_left(),
            moves: 0,
        }
    }
    fn turn_right(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            direction: self.direction.turn_right(),
            moves: 0,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    North = 0,
    #[default]
    East = 1,
    South = 2,
    West = 3,
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
    pub fn heat_map(input: &str) -> Vec<Vec<u32>> {
        let mut heatmap = Vec::new();
        let mut line = Vec::new();
        for c in input.chars() {
            match c.to_digit(10) {
                Some(n) => line.push(n),
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
        let input = Day::SAMPLE_PART1;
        let heatmap = parsers::heat_map(input);
        assert_eq!(102, dijkstra(&heatmap, 1, 3));

        let input = Day::INPUT;
        let heatmap = parsers::heat_map(input);
        assert_eq!(861, dijkstra(&heatmap, 1, 3));
        // Day::test_part1(102)
    }

    #[test]
    fn pathfinding_dijkstra() {
        let input = Day::SAMPLE_PART1;
        let heatmap = parsers::heat_map(input);
        assert_eq!(102, using_pathfinding::dijkstra(&heatmap, 1, 3));
        assert_eq!(94, using_pathfinding::dijkstra(&heatmap, 4, 10));

        let input = Day::INPUT;
        let heatmap = parsers::heat_map(input);
        assert_eq!(861, using_pathfinding::dijkstra(&heatmap, 1, 3));
        assert_eq!(1037, using_pathfinding::dijkstra(&heatmap, 4, 10));
    }

    #[test]
    fn pathfinding_astar() {
        let input = Day::SAMPLE_PART1;
        let heatmap = parsers::heat_map(input);
        assert_eq!(102, using_pathfinding::astar(&heatmap, 1, 3));
        assert_eq!(94, using_pathfinding::astar(&heatmap, 4, 10));

        let input = Day::INPUT;
        let heatmap = parsers::heat_map(input);
        assert_eq!(861, using_pathfinding::astar(&heatmap, 1, 3));
        assert_eq!(1037, using_pathfinding::astar(&heatmap, 4, 10));
    }

    #[test]
    fn test_part2() {
        let input = Day::SAMPLE_PART2;
        let heatmap = parsers::heat_map(input);
        assert_eq!(94, dijkstra(&heatmap, 4, 10));

        let input = Day::INPUT;
        let heatmap = parsers::heat_map(input);
        assert_eq!(1037, dijkstra(&heatmap, 4, 10));
        // Day::test_part2(94)
    }
}

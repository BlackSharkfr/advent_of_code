use aoc::Aoc;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 10;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let map = parsers::part1_hashmap(input);
        let start = find_start(&map).expect("Starting location not found");
        let pipes = find_pipe_loop(start, &map);
        pipes.len() as u32 / 2
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let map = parsers::part1_hashmap(input);

        let start = find_start(&map).expect("Starting location not found");
        let pipes = find_pipe_loop(start, &map);
        let (min_x, max_x, min_y, max_y) = min_max_coords(&pipes);

        // Genericise the solution for any starting location along the pipe
        let replacement = start_replacement(start, pipes[1], pipes[pipes.len() - 1]);

        let mut pipes_map = HashMap::with_capacity(pipes.len());
        pipes_map.insert(start, replacement);
        for coords in &pipes[1..pipes.len()] {
            if let Some(pipe) = map.get(coords).cloned() {
                pipes_map.insert(*coords, pipe);
            }
        }

        (min_y + 1..max_y)
            .map(|y| {
                let mut inside = false;
                let mut count = 0;
                for x in min_x..max_x {
                    match pipes_map.get(&(x, y)) {
                        // Both solutions work thanks to the Genericisation above
                        // Some(Pipe::BottomLeft) | Some(Pipe::BottomRight) | Some(Pipe::Vertical) => {
                        Some(Pipe::TopLeft) | Some(Pipe::TopRight) | Some(Pipe::Vertical) => {
                            inside = !inside
                        }
                        None if inside => count += 1,
                        _ => (),
                    }
                }
                count
            })
            .sum()
    }
}

fn find_start(map: &HashMap<Coords, Pipe>) -> Option<Coords> {
    map.iter()
        .find_map(|(coords, pipe)| (*pipe == Pipe::Start).then_some(*coords))
}

fn find_pipe_loop(start: Coords, map: &HashMap<Coords, Pipe>) -> Vec<Coords> {
    for mut direction in Direction::ALL {
        let mut coords = start;
        let mut pipes = vec![start];
        loop {
            coords = direction.next_coords(coords);
            let Some(pipe) = map.get(&coords) else {
                break;
            };
            if *pipe == Pipe::Start {
                return pipes;
            }
            pipes.push(coords);
            let Some(next) = direction.next_direction(pipe) else {
                break;
            };
            direction = next;
        }
    }
    panic!("Searched all directions but did not find the loop")
}

fn start_replacement(start: Coords, mut a: Coords, mut b: Coords) -> Pipe {
    a.0 -= start.0;
    a.1 -= start.1;
    b.0 -= start.0;
    b.1 -= start.1;
    match (a, b) {
        ((1, 0), (-1, 0)) | ((-1, 0), (1, 0)) => Pipe::Horizontal,
        ((0, 1), (0, -1)) | ((0, -1), (0, 1)) => Pipe::Vertical,
        ((-1, 0), (0, -1)) | ((0, -1), (-1, 0)) => Pipe::BottomRight,
        ((0, -1), (1, 0)) | ((1, 0), (0, -1)) => Pipe::BottomLeft,
        ((0, 1), (1, 0)) | ((1, 0), (0, 1)) => Pipe::TopLeft,
        ((0, 1), (-1, 0)) | ((-1, 0), (0, 1)) => Pipe::TopRight,
        _ => panic!("Invalid input coordinates"),
    }
}

/// Returns (min X, max X, min Y, max Y)
fn min_max_coords(coords: &[Coords]) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for (x, y) in coords {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    (min_x, max_x, min_y, max_y)
}

type Coords = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    /// Move one step forwards
    fn next_coords(&self, (x, y): Coords) -> Coords {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    /// Change direction based on pipe shape
    ///
    /// returns `None` if the pipe is entered from a wrong direction (genericise solution)
    fn next_direction(self, pipe: &Pipe) -> Option<Direction> {
        match (self, pipe) {
            (_, Pipe::Vertical) => Some(self),
            (_, Pipe::Horizontal) => Some(self),
            (Direction::Up, Pipe::TopLeft) => Some(Direction::Right),
            (Direction::Left, Pipe::TopLeft) => Some(Direction::Down),
            (Direction::Up, Pipe::TopRight) => Some(Direction::Left),
            (Direction::Right, Pipe::TopRight) => Some(Direction::Down),
            (Direction::Down, Pipe::BottomLeft) => Some(Direction::Right),
            (Direction::Left, Pipe::BottomLeft) => Some(Direction::Up),
            (Direction::Down, Pipe::BottomRight) => Some(Direction::Left),
            (Direction::Right, Pipe::BottomRight) => Some(Direction::Up),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Start,
}
impl TryFrom<char> for Pipe {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'F' => Ok(Self::TopLeft),
            'J' => Ok(Self::BottomRight),
            'L' => Ok(Self::BottomLeft),
            '7' => Ok(Self::TopRight),
            'S' => Ok(Self::Start),
            _ => Err(()),
        }
    }
}
impl Pipe {
    /// For debugging
    fn to_char(self) -> char {
        match self {
            Pipe::Vertical => '│',
            Pipe::Horizontal => '─',
            Pipe::TopLeft => '┌',
            Pipe::TopRight => '┐',
            Pipe::BottomLeft => '└',
            Pipe::BottomRight => '┘',
            Pipe::Start => 'S',
        }
    }
}
impl std::fmt::Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

mod parsers {

    use std::collections::HashMap;

    use super::*;

    pub fn part1_hashmap(input: &str) -> HashMap<(i32, i32), Pipe> {
        input
            .lines()
            .enumerate()
            .par_bridge()
            .flat_map_iter(|(y, line)| {
                line.char_indices().flat_map(move |(x, c)| {
                    Pipe::try_from(c)
                        .ok()
                        .map(|pipe| ((x as i32, y as i32), pipe))
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Day::SAMPLE_PART1;
        assert_eq!(4, Day::part1(input));

        let input = include_str!("../inputs/sample1.1.txt");
        assert_eq!(8, Day::part1(input));
    }

    #[test]
    fn test_part2() {
        println!("Sample0");
        let input = Day::SAMPLE_PART2;
        assert_eq!(4, Day::part2(input));

        println!("Sample1");
        let input = include_str!("../inputs/sample2.1.txt");
        assert_eq!(8, Day::part2(input));

        println!("Sample2");
        let input = include_str!("../inputs/sample2.2.txt");
        assert_eq!(10, Day::part2(input));
    }
}

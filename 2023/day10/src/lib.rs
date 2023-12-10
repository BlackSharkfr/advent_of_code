use std::collections::HashMap;

use aoc::Aoc;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 10;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let map = parsers::part1_hashmap(input);

        let start = find_start(&map).expect("Map should contain a start");
        for mut direction in Direction::ALL {
            let mut coords = start;
            let mut count = 0;
            loop {
                coords = direction.next_coords(coords);
                let Some(pipe) = map.get(&coords) else {
                    break;
                };
                count += 1;
                if *pipe == Pipe::Start {
                    return count / 2;
                }
                let Some(next) = direction.next(pipe) else {
                    break;
                };
                direction = next;
            }
        }
        panic!("Searched all 4 directions without looping to the start")
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let map = parsers::part1_hashmap(input);

        let polygon = find_polygon(&map);

        let (min_x, max_x, min_y, max_y) = min_max_coords(&polygon);
        (min_y + 1..max_y)
            .flat_map(|y| (min_x + 1..max_x).map(move |x| (x, y)))
            .par_bridge()
            .filter(|(x, y)| {
                !is_on_edge((*x, *y), &polygon) && winding_number((*x, *y), &polygon) != 0
            })
            .count() as u32
    }
}

fn find_start(map: &HashMap<Coords, Pipe>) -> Option<Coords> {
    map.par_iter()
        .find_map_any(|(coords, pipe)| (*pipe == Pipe::Start).then_some(*coords))
}

/// implementation of Dan Sunday's Point in Polygon using Winding number algorithm
fn winding_number((x, y): Coords, polygon: &[Coords]) -> i32 {
    let mut wn = 0;
    for edge in polygon.windows(2) {
        let (x1, y1) = (edge[0].0, edge[0].1);
        let (x2, y2) = (edge[1].0, edge[1].1);
        if y1 <= y {
            // start y <= point.y
            if y2 > y {
                // upwards crossing
                let offset = left_of_edge((x, y), (x1, y1), (x2, y2));
                if offset > 0 {
                    // Point left of edge
                    wn += 1;
                }
            }
        } else {
            // start y > point.y
            if y2 <= y {
                // downwards crossing
                let offset = left_of_edge((x, y), (x1, y1), (x2, y2));
                if offset < 0 {
                    // Point right of edge
                    wn -= 1
                }
            }
        }
    }

    wn
}

fn left_of_edge((x, y): Coords, (x1, y1): Coords, (x2, y2): Coords) -> i32 {
    (x2 - x1) * (y - y1) - (x - x1) * (y2 - y1)
}

fn is_on_edge((x, y): Coords, polygon: &[Coords]) -> bool {
    for edge in polygon.windows(2) {
        let (x1, y1) = (edge[0].0, edge[0].1);
        let (x2, y2) = (edge[1].0, edge[1].1);
        if x == x1 && x == x2 {
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);
            if ymin <= y && y <= ymax {
                return true;
            }
            continue;
        }
        if y == y1 && y == y2 {
            let xmin = x1.min(x2);
            let xmax = x1.max(x2);
            if xmin <= x && x <= xmax {
                return true;
            }
        }
    }
    false
}

/// Find the Pipes and descibe it as a closed polygon
fn find_polygon(map: &HashMap<(i32, i32), Pipe>) -> Vec<(i32, i32)> {
    let start = find_start(map).expect("Map should contain a start");

    for mut direction in Direction::ALL {
        let mut polygon = vec![start];
        let mut coords = start;
        loop {
            coords = direction.next_coords(coords);
            let Some(pipe) = map.get(&coords) else {
                break;
            };
            match pipe {
                Pipe::BottomLeft | Pipe::BottomRight | Pipe::TopLeft | Pipe::TopRight => {
                    polygon.push(coords)
                }
                Pipe::Start => {
                    polygon.push(coords);
                    return polygon;
                }
                _ => (),
            }

            let Some(next) = direction.next(pipe) else {
                break;
            };
            direction = next;
        }
    }
    panic!("Searched all 4 directions without looping to the start")
}

/// Returns (min X, max X, min Y, max Y)
fn min_max_coords(path: &[Coords]) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for (x, y) in path {
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

    fn next_coords(&self, (x, y): Coords) -> Coords {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    fn next(self, pipe: &Pipe) -> Option<Direction> {
        match (pipe, self) {
            (Pipe::Vertical, _) => Some(self),
            (Pipe::Horizontal, _) => Some(self),
            (Pipe::TopLeft, Direction::Up) => Some(Direction::Right),
            (Pipe::TopLeft, Direction::Left) => Some(Direction::Down),
            (Pipe::TopRight, Direction::Up) => Some(Direction::Left),
            (Pipe::TopRight, Direction::Right) => Some(Direction::Down),
            (Pipe::BottomLeft, Direction::Down) => Some(Direction::Right),
            (Pipe::BottomLeft, Direction::Left) => Some(Direction::Up),
            (Pipe::BottomRight, Direction::Down) => Some(Direction::Left),
            (Pipe::BottomRight, Direction::Right) => Some(Direction::Up),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Pipe {
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Start,
}
impl Pipe {
    fn to_char(&self) -> char {
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
            .flat_map(|(y, line)| {
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

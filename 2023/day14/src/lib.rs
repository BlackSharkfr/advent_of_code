use aoc::Aoc;
use itertools::Itertools;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = usize;
    const DAY_NUMBER: u8 = 14;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let (_, mut grid) = parsers::grid(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));
        // println!("Initial grid : \n{}", debug_grid(&grid));
        move_north(&mut grid);
        // println!("After Movement : \n{}", debug_grid(&grid));
        count_load(&grid)
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let (_, mut grid) = parsers::grid(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));

        let mut cycle_history = Vec::new();
        let Some(loop_start) = (0..1_000_000_000).find_map(|_| {
            cycle_history.push(grid.clone());
            cycle(&mut grid);
            cycle_history.iter().position(|prev| *prev == grid)
        }) else {
            // Did you really run a billion cycles without running into a loop ?
            return count_load(&grid);
        };

        let loop_length = cycle_history.len() - loop_start;
        // println!("Loop found starts at {loop_start} cycles, loop length : {loop_length}, history[{loop_start}] == history[{}]", cycle_history.len(),);
        let solution_index = loop_start + ((1_000_000_000 - loop_start) % loop_length);
        count_load(&cycle_history[solution_index])
    }
}
type Grid = Vec<Vec<Option<Rock>>>;
#[allow(dead_code)]
fn debug_grid(grid: &Grid) -> String {
    grid.iter()
        .map(|line| line.iter().map(Rock::opt_to_char).collect::<String>())
        .join("\n")
}

fn move_north(grid: &mut Grid) {
    let height = grid.len();
    let width = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == Some(Rock::Round) {
                grid[y][x] = None;
                let insert = (0..y)
                    .rev()
                    .find(|y| (grid[*y][x] != None))
                    .map(|y| y + 1)
                    .unwrap_or(0);
                grid[insert][x] = Some(Rock::Round);
            }
        }
    }
}

fn move_south(grid: &mut Grid) {
    let height = grid.len();
    let width = grid[0].len();
    for y in (0..height).rev() {
        for x in 0..width {
            if grid[y][x] == Some(Rock::Round) {
                grid[y][x] = None;
                let insert = (y..height)
                    .find(|y| (grid[*y][x] != None))
                    .map(|y| y - 1)
                    .unwrap_or(height - 1);
                grid[insert][x] = Some(Rock::Round);
            }
        }
    }
}

fn move_west(grid: &mut Grid) {
    let height = grid.len();
    let width = grid[0].len();
    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == Some(Rock::Round) {
                grid[y][x] = None;
                let insert = (0..x)
                    .rev()
                    .find(|x| (grid[y][*x] != None))
                    .map(|x| x + 1)
                    .unwrap_or(0);
                grid[y][insert] = Some(Rock::Round);
            }
        }
    }
}

fn move_east(grid: &mut Grid) {
    let height = grid.len();
    let width = grid[0].len();
    for x in (0..width).rev() {
        for y in 0..height {
            if grid[y][x] == Some(Rock::Round) {
                grid[y][x] = None;
                let insert = (x..width)
                    .find(|x| (grid[y][*x] != None))
                    .map(|x| x - 1)
                    .unwrap_or(width - 1);
                grid[y][insert] = Some(Rock::Round);
            }
        }
    }
}

fn cycle(grid: &mut Grid) {
    move_north(grid);
    move_west(grid);
    move_south(grid);
    move_east(grid);
}

fn count_load(grid: &Grid) -> usize {
    (1..=grid.len())
        .rev()
        .zip(grid.iter())
        .map(|(moment, line)| {
            moment
                * line
                    .iter()
                    .filter(|rock| **rock == Some(Rock::Round))
                    .count()
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Rock {
    Round,
    Square,
}

impl Rock {
    fn as_char(&self) -> char {
        match self {
            Rock::Round => 'O',
            Rock::Square => '#',
        }
    }
    fn opt_to_char(rock: &Option<Self>) -> char {
        match rock {
            None => '.',
            Some(rock) => rock.as_char(),
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

    pub fn grid(input: &str) -> IResult<&str, Grid> {
        separated_list1(line_ending, many1(rock))(input)
    }

    fn rock(input: &str) -> IResult<&str, Option<Rock>> {
        anychar
            .map_res(|c| match c {
                'O' => Ok(Some(Rock::Round)),
                '#' => Ok(Some(Rock::Square)),
                '.' => Ok(None),
                _ => Err(()),
            })
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(136)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(64)
    }
}

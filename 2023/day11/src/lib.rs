use aoc::Aoc;
use itertools::Itertools;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = usize;
    const DAY_NUMBER: u8 = 11;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let mut coords = parsers::part1(input);
        expand_universe(&mut coords, 1);
        sum_distances(&coords)
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let mut coords = parsers::part1(input);
        expand_universe(&mut coords, 1_000_000 - 1);
        sum_distances(&coords)
    }
}

fn sum_distances(coords: &[Coords]) -> usize {
    coords
        .iter()
        .tuple_combinations()
        .map(|(a, b)| manhattan_distance(*a, *b))
        .sum()
}

fn expand_universe(coords: &mut [Coords], expanse_ratio: usize) {
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    for (x, y) in coords.iter() {
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }
    for i in (0..=max_x).rev() {
        if coords.iter().all(|(x, _)| *x != i) {
            for (x, _) in coords.iter_mut().filter(|(x, _)| *x > i) {
                *x += expanse_ratio
            }
        }
    }
    for i in (0..=max_y).rev() {
        if coords.iter().all(|(_, y)| *y != i) {
            for (_, y) in coords.iter_mut().filter(|(_, y)| *y > i) {
                *y += expanse_ratio
            }
        }
    }
}

fn manhattan_distance(a: Coords, b: Coords) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

type Coords = (usize, usize);

mod parsers {
    use super::*;

    /// Returns Vec<Coords of galaxy> and Vec<Empty line numbers>
    pub fn part1(input: &str) -> Vec<Coords> {
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(374)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(0)
    }
}

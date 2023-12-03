use aoc::Aoc;
use itertools::Itertools;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 3;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> u32 {
        part1_attempt3(&input)
    }

    fn part2(input: &str) -> u32 {
        part2_attempt1(input)
    }
}

/// First attempt
///
/// Searches for digits, then builds the number and scans around for symbols at the same time
/// Not optimal :
/// - Checks the same coordinates multiple times
/// - Big ugly match statement
pub fn part1_attempt1(input: &str) -> u32 {
    let grid = Vec2d::from(input);
    let mut total = 0;
    let mut current_number = Option::<Number>::None;
    for (y, line) in grid.data.iter().enumerate() {
        if let Some(Number::Adjacent(n)) = current_number.take() {
            total += n
        }
        for (x, c) in line.iter().cloned().enumerate() {
            match (
                c.to_digit(10),
                grid.check_symbol_around(x, y),
                &mut current_number,
            ) {
                (None, _, None) => (),
                (None, _, Some(Number::Adjacent(n))) => {
                    total += *n;
                    current_number = None
                }
                (None, _, Some(Number::Isolated(_))) => current_number = None,

                (Some(n), false, None) => current_number = Some(Number::Isolated(n)),
                (Some(n), false, Some(Number::Isolated(current))) => *current = (*current * 10) + n,
                (Some(n), false, Some(Number::Adjacent(current))) => *current = (*current * 10) + n,

                (Some(n), true, None) => current_number = Some(Number::Adjacent(n)),
                (Some(n), true, Some(Number::Isolated(current))) => {
                    current_number = Some(Number::Adjacent((*current * 10) + n))
                }
                (Some(n), true, Some(Number::Adjacent(current))) => *current = (*current * 10) + n,
            }
        }
    }
    if let Some(Number::Adjacent(n)) = current_number {
        total += n
    }
    total
}

/// First altenative
///
/// Search for numbers, builds the numbers, then look for symbols around the numbers
/// Argh !!! it's slower !
/// - Builds the number multiple times before dedup
/// - Scans where no symbols are present take time
pub fn part1_attempt2(input: &str) -> u32 {
    let grid = Vec2d::from(input);
    grid.iter_coordinates()
        .filter_map(|(x, y)| grid.get_number_with_coordinates(x, y))
        .dedup()
        .filter_map(|(number, xmin, xmax, y)| {
            let xmin = xmin.saturating_sub(1);
            let xmax = xmax + 1;
            // Above
            if y != 0 {
                for x in xmin..=xmax {
                    if grid.get(x, y - 1).map(is_symbol).unwrap_or(false) {
                        return Some(number);
                    }
                }
            }
            // Below
            if y != grid.height() - 1 {
                for x in xmin..=xmax {
                    if grid.get(x, y + 1).map(is_symbol).unwrap_or(false) {
                        return Some(number);
                    }
                }
            }
            // Left and right
            if grid.get(xmin, y).map(is_symbol).unwrap_or(false)
                || grid.get(xmax, y).map(is_symbol).unwrap_or(false)
            {
                return Some(number);
            }
            None
        })
        .sum()
}

/// Second alternative
///
/// Search for symbols then look for numbers around the symbols
/// Yes ! it's faster !
pub fn part1_attempt3(input: &str) -> u32 {
    let grid = Vec2d::from(input);
    let grid = &grid;
    grid.iter_with_coordinates()
        .filter(|(_, _, c)| is_symbol(c))
        .flat_map(|(x, y, _)| {
            Direction::ALL.into_iter().flat_map({
                move |direction| {
                    direction
                        .offset(x, y)
                        .and_then(|(x, y)| grid.get_number_with_coordinates(x, y))
                }
            })
        })
        .dedup()
        .map(|v| v.0)
        .sum()
}

/// First attempt
///
/// It's suprizingly quite fast ! but it's ugly
/// - custom search around gears to avoid scanning and building numbers multiple times
pub fn part2_attempt1(input: &str) -> u32 {
    let grid = Vec2d::from(input);
    let mut total = 0;
    let gears = grid.iter_with_coordinates().filter(|(_, _, c)| is_gear(c));
    for (x, y, _) in gears {
        let get_number = |dir: Direction| dir.offset(x, y).and_then(|(x, y)| grid.get_number(x, y));
        let top = get_number(Direction::Top);
        let top_left = top
            .is_none()
            .then(|| get_number(Direction::TopLeft))
            .unwrap_or(None);
        let top_right = top
            .is_none()
            .then(|| get_number(Direction::TopRight))
            .unwrap_or(None);
        let left = get_number(Direction::Left);
        let right = get_number(Direction::Right);
        let bottom = get_number(Direction::Bottom);
        let bottom_left = bottom
            .is_none()
            .then(|| get_number(Direction::BottomLeft))
            .unwrap_or(None);
        let bottom_right = bottom
            .is_none()
            .then(|| get_number(Direction::BottomRight))
            .unwrap_or(None);

        let numbers = [
            top_left,
            top,
            top_right,
            left,
            right,
            bottom_left,
            bottom,
            bottom_right,
        ];
        // println!("Gear ({x},{y}): {numbers:?}");

        if numbers.iter().flatten().count() == 2 {
            total += numbers.into_iter().flatten().fold(1, |acc, n| acc * n)
        }
    }
    total
}

pub fn part2_attempt2(input: &str) -> u32 {
    let grid = Vec2d::from(input);
    grid.iter_with_coordinates()
        .filter(|(_, _, c)| is_gear(c))
        .filter_map(|(x, y, _)| {
            let numbers = Direction::ALL
                .iter()
                .filter_map(|direction| direction.offset(x, y))
                .filter_map(|(x, y)| grid.get_number_with_coordinates(x, y))
                .map(|items| items.0)
                .dedup();

            (numbers.clone().count() == 2).then_some(numbers.into_iter().fold(1, |acc, n| acc * n))
        })
        .sum()
}

pub fn part2_attempt3(input: &str) -> u32 {
    let grid = Vec2d::from((input, |c: char| match c.to_digit(10) {
        Some(n) => Some(Item::Number(n as u8)),
        None if c == '.' => None,
        _ => Some(Item::Symbol(c as u8)),
    }));
    grid.iter_with_coordinates()
        .filter(|(_, _, value)| value.as_ref().is_some_and(Item::is_gear))
        .filter_map(|(x, y, _)| {
            let numbers = Direction::ALL
                .iter()
                .filter_map(|direction| direction.offset(x, y))
                .filter_map(|(x, y)| get_number(&grid, x, y))
                .dedup();
            (numbers.clone().count() == 2).then_some(
                numbers
                    .into_iter()
                    // .map(|(n, ..)| n)
                    .fold(1, |acc, n| acc * n),
            )
        })
        .sum()
}

fn get_number(grid: &Vec2d<Option<Item>>, x: usize, y: usize) -> Option<u32> {
    let Some(left_index) = (0..=x)
        .rev()
        .take_while(|x| {
            grid.get(*x, y)
                .map(|value| value.as_ref().is_some_and(Item::is_number))
                .unwrap_or(false)
        })
        .last()
    else {
        return None;
    };
    (left_index..)
        .map_while(|x| {
            grid.get(x, y)
                .and_then(|value| value.as_ref().and_then(Item::to_u32))
        })
        .fold(0, |acc, n| (acc * 10) + n)
        .into()
}

enum Item {
    Number(u8),
    Symbol(u8),
}
impl Item {
    fn is_gear(&self) -> bool {
        matches!(self, Self::Symbol(b'*'))
    }
    fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }
    fn to_u32(&self) -> Option<u32> {
        match self {
            Item::Number(n) => Some(*n as u32),
            _ => None,
        }
    }
}

/// 2D Vector, reusable for future days
///
/// Input is line by line :  
/// x is the horizontal character position within each line
/// y is the vertical line index
///
/// # Example usage
///
/// Create a character grid from input string
/// ```
/// # use day03::Vec2d;
/// let input: &str = "inputstring";
/// let grid: Vec2d<char> = Vec2d::from(input);
/// assert_eq!(grid.get(3,0), Some(&'u'));
/// assert_eq!(grid.get(6,3), None);
/// ```
///
/// Create a custom grid with a per-character processing function
/// ```
/// # use day03::Vec2d;
/// let input: &str = "123...654";
/// let to_digit = |c:char| c.to_digit(10);
/// let grid: Vec2d<Option<u32>> = Vec2d::from((input, to_digit));
/// assert_eq!(grid.get(2,0), Some(&Some(3)));
/// assert_eq!(grid.get(4,0), Some(&None));
/// assert_eq!(grid.get(6,3), None);
/// ```
pub struct Vec2d<T: Sized> {
    pub data: Vec<Vec<T>>,
}
/// Reusable impls for future days
impl<T> Vec2d<T> {
    /// Get the data using (x,y) coordinates
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|line| line.get(x))
    }

    /// Iterate over the whole grid line by line
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flat_map(|line| line.iter())
    }

    /// Iterate over the whole grid line by line, with coordinates
    ///
    /// Returned Item: (x, y, T)
    pub fn iter_with_coordinates(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, c)| (x, y, c)))
    }

    /// Iterate over the grid coordinates, line by line
    pub fn iter_coordinates(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(y, line)| (0..line.len()).map(move |x| (x, y)))
    }

    /// Returns the width of the grid.  
    ///
    /// Naive impementation only checks the first row.  
    /// To verify all rows have the same width, use [`width_full`](Grid::width_full_check)
    pub fn width_naive(&self) -> usize {
        self.data.get(0).map(|line| line.len()).unwrap_or_default()
    }

    /// Returns the width of the grid.  
    ///
    /// Full check impementation checks every row has the same length.  
    /// Returns `None` if the rows have a different sizes
    pub fn width_full_check(&self) -> Option<usize> {
        let width = self.width_naive();
        for line in &self.data {
            if line.len() != width {
                return None;
            }
        }
        Some(width)
    }

    /// Height of the grid
    ///
    /// Always reliable
    pub fn height(&self) -> usize {
        self.data.len()
    }
}

impl From<&str> for Vec2d<char> {
    fn from(input: &str) -> Self {
        let data = input.lines().map(|line| line.chars().collect()).collect();
        Vec2d { data }
    }
}
impl<F, T> From<(&str, F)> for Vec2d<T>
where
    F: Fn(char) -> T,
{
    fn from((input, f): (&str, F)) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().map(|c| f(c)).collect())
            .collect();
        Vec2d { data }
    }
}

/// Specific to Day03
impl Vec2d<char> {
    /// Returns true is there is a symbol in any of the 8 surrounding coordinates
    fn check_symbol_around(&self, x: usize, y: usize) -> bool {
        Direction::ALL.into_iter().any(|direction| {
            let Some((x, y)) = direction.offset(x, y) else {
                return false;
            };
            let Some(c) = self.get(x, y) else {
                return false;
            };
            !c.is_ascii_digit() && *c != '.'
        })
    }

    fn get_number(&self, x: usize, y: usize) -> Option<u32> {
        let Some(left_index) = (0..=x)
            .rev()
            .take_while(|x| self.get(*x, y).map(|c| c.is_ascii_digit()).unwrap_or(false))
            .last()
        else {
            return None;
        };
        (left_index..)
            .map_while(|x| self.get(x, y).and_then(|c| c.to_digit(10)))
            .fold(0, |acc, n| (acc * 10) + n)
            .into()
    }

    fn get_number_with_coordinates(
        &self,
        x: usize,
        y: usize,
    ) -> Option<(u32, usize, usize, usize)> {
        if self.get(x, y).map(|c| !c.is_ascii_digit()).unwrap_or(true) {
            return None;
        }
        let Some(left_index) = (0..=x)
            .rev()
            .take_while(|x| self.get(*x, y).map(|c| c.is_ascii_digit()).unwrap_or(false))
            .last()
        else {
            return None;
        };
        let right_index = (left_index..)
            .position(|x| self.get(x, y).map(|c| !c.is_ascii_digit()).unwrap_or(true))
            .unwrap_or(left_index + 1)
            + left_index;
        let number = (left_index..right_index)
            .filter_map(|x| self.get(x, y).and_then(|c| c.to_digit(10)))
            .fold(0, |acc, n| (acc * 10) + n);
        Some((number, left_index, right_index - 1, y))
    }
}

fn is_symbol(c: &char) -> bool {
    *c != '.' && !c.is_ascii_digit()
}

fn is_gear(c: &char) -> bool {
    *c == '*'
}

#[derive(PartialEq)]
enum Number {
    Isolated(u32),
    Adjacent(u32),
}

enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}
impl Direction {
    const ALL: [Self; 8] = [
        Self::TopLeft,
        Self::Top,
        Self::TopRight,
        Self::Left,
        Self::Right,
        Self::BottomLeft,
        Self::Bottom,
        Self::BottomRight,
    ];
    fn offset(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::TopLeft if x != 0 && y != 0 => Some((x - 1, y - 1)),
            Direction::Top if y != 0 => Some((x, y - 1)),
            Direction::TopRight if y != 0 => Some((x + 1, y - 1)),
            Direction::Left if x != 0 => Some((x - 1, y)),
            Direction::Right => Some((x + 1, y)),
            Direction::BottomLeft if x != 0 => Some((x - 1, y + 1)),
            Direction::Bottom => Some((x, y + 1)),
            Direction::BottomRight => Some((x + 1, y + 1)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = Day::SAMPLE_PART1;
        assert_eq!(4361, part1_attempt1(input));

        let input = Day::INPUT;
        assert_eq!(559667, part1_attempt1(input));
    }

    #[test]
    fn test_part1_2() {
        let input = Day::SAMPLE_PART1;
        assert_eq!(4361, part1_attempt2(input));

        let input = Day::INPUT;
        assert_eq!(559667, part1_attempt2(input));
    }

    #[test]
    fn test_part1_3() {
        let input = Day::SAMPLE_PART1;
        assert_eq!(4361, part1_attempt3(input));

        let input = Day::INPUT;
        assert_eq!(559667, part1_attempt3(input));
    }

    #[test]
    fn test_part2_1() {
        let input = Day::SAMPLE_PART2;
        assert_eq!(467835, part2_attempt1(input));

        let input = Day::INPUT;
        assert_eq!(86841457, part2_attempt1(input));
    }

    #[test]
    fn test_part2_2() {
        let input = Day::SAMPLE_PART2;
        assert_eq!(467835, part2_attempt3(input));

        let input = Day::INPUT;
        assert_eq!(86841457, part2_attempt3(input));
    }

    #[test]
    fn test_part2_3() {
        let input = Day::SAMPLE_PART2;
        assert_eq!(467835, part2_attempt3(input));

        let input = Day::INPUT;
        assert_eq!(86841457, part2_attempt3(input));
    }
}

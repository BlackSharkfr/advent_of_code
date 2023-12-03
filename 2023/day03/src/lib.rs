use aoc::Aoc;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u32;
    const DAY_NUMBER: u8 = 3;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> u32 {
        let grid = Grid::from(input);
        grid.compute_part1()
    }

    fn part2(input: &str) -> u32 {
        let grid = Grid::from(input);
        grid.compute_part2()
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
/// # use day03::Grid;
/// let input: &str = "inputstring";
/// let grid: Grid<char> = Grid::from(input);
/// assert_eq!(grid.get(3,0), Some(&'u'));
/// assert_eq!(grid.get(6,3), None);
/// ```
///
/// Create a custom grid with a per-character processing function
/// ```
/// # use day03::Grid;
/// let input: &str = "123...654";
/// let to_digit = |c:char| c.to_digit(10);
/// let grid: Grid<Option<u32>> = Grid::from((input, to_digit));
/// assert_eq!(grid.get(2,0), Some(&Some(3)));
/// assert_eq!(grid.get(4,0), Some(&None));
/// assert_eq!(grid.get(6,3), None);
/// ```
pub struct Grid<T: Sized> {
    pub data: Vec<Vec<T>>,
}
/// Reusable impls for future days
impl<T> Grid<T> {
    /// Get the data using (x,y) coordinates
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|line| line.get(x))
    }

    /// Iterate over the whole grid line by line
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flat_map(|line| line.iter())
    }

    /// Iterate over the whole grid line by line, with coordinates.
    ///
    /// Returned Item: (x, y, T)
    pub fn iter_with_coordinates(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, c)| (x, y, c)))
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
}
impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> std::ops::DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl<T> AsRef<Vec<Vec<T>>> for Grid<T> {
    fn as_ref(&self) -> &Vec<Vec<T>> {
        &self.data
    }
}
impl From<&str> for Grid<char> {
    fn from(input: &str) -> Self {
        let data = input.lines().map(|line| line.chars().collect()).collect();
        Grid { data }
    }
}
impl<F, T> From<(&str, F)> for Grid<T>
where
    F: Fn(char) -> T,
{
    fn from((input, f): (&str, F)) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().map(|c| f(c)).collect())
            .collect();
        Grid { data }
    }
}

/// Specific to Day03
impl Grid<char> {
    fn compute_part1(&self) -> u32 {
        let mut total = 0;
        let mut current_number = Option::<Number>::None;
        for (y, line) in self.data.iter().enumerate() {
            if let Some(Number::Adjacent(n)) = current_number.take() {
                total += n
            }
            for (x, c) in line.iter().cloned().enumerate() {
                match (
                    c.to_digit(10),
                    self.check_symbol_around(x, y),
                    &mut current_number,
                ) {
                    (None, _, None) => (),
                    (None, _, Some(Number::Adjacent(n))) => {
                        total += *n;
                        current_number = None
                    }
                    (None, _, Some(Number::Isolated(_))) => current_number = None,

                    (Some(n), false, None) => current_number = Some(Number::Isolated(n)),
                    (Some(n), false, Some(Number::Isolated(current))) => {
                        *current = (*current * 10) + n
                    }
                    (Some(n), false, Some(Number::Adjacent(current))) => {
                        *current = (*current * 10) + n
                    }

                    (Some(n), true, None) => current_number = Some(Number::Adjacent(n)),
                    (Some(n), true, Some(Number::Isolated(current))) => {
                        current_number = Some(Number::Adjacent((*current * 10) + n))
                    }
                    (Some(n), true, Some(Number::Adjacent(current))) => {
                        *current = (*current * 10) + n
                    }
                }
            }
        }
        if let Some(Number::Adjacent(n)) = current_number {
            total += n
        }
        total
    }

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

    fn compute_part2(&self) -> u32 {
        let mut total = 0;
        let gears = self.iter_with_coordinates().filter(|(_, _, c)| is_gear(c));
        for (x, y, _) in gears {
            let get_number =
                move |dir: Direction| dir.offset(x, y).and_then(|(x, y)| self.get_number(x, y));
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
            if numbers.iter().flatten().count() == 2 {
                total += numbers
                    .into_iter()
                    .flatten()
                    .reduce(|acc, n| acc * n)
                    .unwrap_or_default()
            }
        }
        total
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
    fn offset(self, x: usize, y: usize) -> Option<(usize, usize)> {
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
    fn test_part1() {
        Day::test_part1(4361)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(467835)
    }
}

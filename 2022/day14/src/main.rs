use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    str::FromStr,
};

static INPUT: &str = include_str!("input.txt");
fn main() {
    let time_start = std::time::Instant::now();
    println!("Part 1 : {}", part1(INPUT));
    let end_part1 = std::time::Instant::now();
    println!("Part 2 : {}", part2(INPUT));
    let end_part2 = std::time::Instant::now();
    println!(
        "Time part 1 : {} µs\nTime part 2 : {} µs",
        end_part1.duration_since(time_start).as_micros(),
        end_part2.duration_since(end_part1).as_micros(),
    )
}

fn part1(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();
    grid.pour_sand()
    // println!("Grid : \n{grid:?}");
}

fn part2(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();
    grid.resize_for_part2();
    grid.pour_sand()
    // println!("Grid : \n{grid:?}");
}

#[derive(Debug, Clone)]
struct Point2D {
    x: usize,
    y: usize,
}
impl FromStr for Point2D {
    type Err = ();
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut s = str.split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        Ok(Point2D { x, y })
    }
}
impl Point2D {
    fn interpolate(point_a: &Point2D, point_b: &Point2D) -> Vec<Point2D> {
        let is_horizontal = point_a.y == point_b.y;
        if is_horizontal {
            let y = point_a.y;
            let (x1, x2) = if point_a.x <= point_b.x {
                (point_a.x, point_b.x)
            } else {
                (point_b.x, point_a.x)
            };
            (x1..=x2).map(move |x| Point2D { x, y }).collect()
        } else {
            let x = point_a.x;
            let (y1, y2) = if point_a.y <= point_b.y {
                (point_a.y, point_b.y)
            } else {
                (point_b.y, point_a.y)
            };
            (y1..=y2).map(move |y| Point2D { x, y }).collect()
        }
    }
}
struct Grid {
    grid: Vec<Vec<GridPoint>>,
    sand_point: Point2D,
}
impl Deref for Grid {
    type Target = Vec<Vec<GridPoint>>;
    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}
impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}
impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rock_lines = input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|point| point.parse::<Point2D>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let points = rock_lines.iter().flat_map(|line| line.iter());
        let min_x = points.clone().map(|point| point.x).min().unwrap_or(500);
        let max_x = points.clone().map(|point| point.x).max().unwrap_or(500);
        let max_y = points.clone().map(|point| point.y).max().unwrap_or(0);

        let sand_point = Point2D {
            x: 500 - min_x,
            y: 0,
        };
        let mut grid = Grid::init(max_x - min_x + 1, max_y + 1, sand_point);
        for points in rock_lines.iter().flat_map(|line| line.windows(2)) {
            for point in Point2D::interpolate(&points[0], &points[1]) {
                grid[point.x - min_x][point.y] = GridPoint::Rock;
            }
        }
        Ok(grid)
    }
}
impl Grid {
    fn init(width: usize, height: usize, sand_point: Point2D) -> Grid {
        Grid {
            grid: vec![vec![GridPoint::Air; height]; width],
            sand_point,
        }
    }
    fn add_one_grain(&mut self, cache: &mut Vec<Point2D>) -> Result<(), ()> {
        // let mut position = self.sand_point.clone();
        let mut position = cache.pop().ok_or(())?.clone();
        if self[position.x][position.y] != GridPoint::Air {
            return Err(());
        }
        loop {
            // Down
            if position.y + 1 == self[0].len() {
                self[position.x][position.y] = GridPoint::Sand;
                return Err(());
            }
            if self[position.x][position.y + 1] == GridPoint::Air {
                cache.push(position.clone());
                position.y += 1;
                continue;
            }
            // Left
            if position.x == 0 {
                self[position.x][position.y] = GridPoint::Sand;
                return Err(());
            }
            if self[position.x - 1][position.y + 1] == GridPoint::Air {
                cache.push(position.clone());
                position.x -= 1;
                position.y += 1;
                continue;
            }
            // Right
            if position.x + 1 == self.len() {
                self[position.x][position.y] = GridPoint::Sand;
                return Err(());
            }
            if self[position.x + 1][position.y + 1] == GridPoint::Air {
                cache.push(position.clone());
                position.x += 1;
                position.y += 1;
                continue;
            }
            // Lock
            self[position.x][position.y] = GridPoint::Sand;

            return Ok(());
        }
    }

    fn pour_sand(&mut self) -> usize {
        let mut count = 0;
        let mut fountain = Vec::new();
        fountain.push(self.sand_point.clone());
        while self.add_one_grain(&mut fountain).is_ok() {
            count += 1;
        }
        count
    }

    fn resize_for_part2(&mut self) {
        let height = self[0].len();
        let new_height = height + 2;
        self.sand_point.x += height + 1;
        for col in self.iter_mut() {
            col.extend_from_slice(&[GridPoint::Air, GridPoint::Rock]);
        }
        for _ in 0..height {
            self.insert(0, vec![GridPoint::Air; new_height]);
            self.push(vec![GridPoint::Air; new_height])
        }
        let y_rock_floor = self[0].len() - 1;
        for x in 0..self.len() {
            self[x][y_rock_floor] = GridPoint::Rock;
        }
    }
}
impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.len();
        let height = self[0].len();
        for y in 0..height {
            for x in 0..width {
                write!(f, "{:?}", self[x][y])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum GridPoint {
    Air,
    Rock,
    Sand,
}
impl Debug for GridPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "O"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 24)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 93)
    }
}

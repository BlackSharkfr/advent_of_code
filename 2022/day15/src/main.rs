use rayon::prelude::*;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
    time::Instant,
};

static INPUT: &str = include_str!("input.txt");
fn main() {
    let time_start = Instant::now();
    let pt1 = part1(INPUT, 2000000);
    let time_part1 = Instant::now();
    let pt2 = part2(INPUT, 4000000);
    let time_part2 = Instant::now();
    println!(
        "Part 1 : {} | time {} ms",
        pt1,
        time_part1.duration_since(time_start).as_millis()
    );
    println!(
        "Part 2 : {} | time {} ms",
        pt2,
        time_part2.duration_since(time_part1).as_millis()
    );
}
fn parse_input(str: &str) -> Result<Vec<Sensor>, String> {
    str.lines()
        .enumerate()
        .map(|(line_number, line)| {
            line.parse()
                .map_err(|e| format!("Line {line_number} - {line} - {e}"))
        })
        .collect()
}

fn part1(input: &str, y: isize) -> usize {
    let sensors = parse_input(input).unwrap();

    // let grid = Grid::from(sensors);
    // grid.get_line(10)
    //     .unwrap()
    //     .iter()
    //     .filter(|celltype| **celltype == CellType::Surveyed)
    //     .count()

    // let min_x = sensors
    //     .iter()
    //     .map(|s| s.sensor.0 - manhattan_distance(s.sensor, s.closestbeacon))
    //     .min()
    //     .unwrap();
    // let max_x = sensors
    //     .iter()
    //     .map(|s| s.sensor.0 + manhattan_distance(s.sensor, s.closestbeacon))
    //     .max()
    //     .unwrap();
    // // println!("x_range : ({min_x}..={max_x})");
    // // println!("y: {y}");
    // (min_x..=max_x)
    //     .par_bridge()
    //     .filter(|&x| {
    //         if sensors
    //             .iter()
    //             .any(|sensor| sensor.sensor == (x, y) || sensor.closestbeacon == (x, y))
    //         {
    //             // println!("Sensor found : ({x}, {y})");
    //             return false;
    //         }
    //         sensors
    //             .iter()
    //             .any(|sensor| manhattan_distance((x, y), sensor.sensor) <= sensor.m_dist)
    //     })
    //     .count()

    let mut ranges = sensors
        .iter()
        .filter_map(|sensor| sensor.line_range(y))
        .collect::<Vec<_>>();
    ranges.sort_by_key(|(a, _b)| *a);

    let mut combined_ranges = vec![ranges.first().unwrap().clone()];
    for (a, b) in ranges.into_iter().skip(1) {
        match combined_ranges
            .iter_mut()
            .find(|(range_a, range_b)| range_a <= &a && range_b >= &a)
        {
            Some(range) if range.1 >= b => (),
            Some(range) => range.1 = b,
            None => combined_ranges.push((a, b)),
        }
    }

    // let min = ranges.iter().map(|(a, b)| *a).min().unwrap();
    // let max = ranges.iter().map(|(a, b)| *b).max().unwrap();
    // println!("Minmax : {min} {max}");
    // dbg!(&ranges);

    combined_ranges
        .into_iter()
        .map(|(a, b)| b - a)
        .sum::<isize>() as usize
}

fn part2(input: &str, max_size: isize) -> usize {
    let sensors = parse_input(input).unwrap();

    let (x, y) = (0..=max_size)
        .par_bridge()
        .find_map_any(|y| {
            let mut ranges = sensors
                .iter()
                .filter_map(|sensor| sensor.line_range(y as isize))
                .filter(|(a, b)| *a <= max_size as isize && *b >= 0)
                .map(|(a, b)| (a.clamp(0, max_size as isize), b.clamp(0, max_size as isize)))
                .collect::<Vec<_>>();
            ranges.sort_by_key(|(a, _b)| *a);

            let mut range_scanner = 0_isize;
            for range in ranges {
                // println!("Range : {range:?}");
                if range.0 > range_scanner {
                    break;
                }
                if range.1 > max_size {
                    range_scanner = max_size + 1;
                    break;
                }
                if range.1 > range_scanner {
                    range_scanner = range.1 + 1
                }
            }

            if range_scanner >= max_size {
                return None;
            }
            Some((range_scanner as usize, y as usize))
        })
        .unwrap();

    println!(" Found ({x},{y})");
    (x * 4000000) + y
}

struct Grid {
    grid: Vec<Vec<CellType>>,
    offset: (isize, isize),
}
impl Grid {
    fn get(&self, mut x: isize, mut y: isize) -> Option<CellType> {
        x -= self.offset.0;
        y -= self.offset.1;
        self.grid
            .get(y as usize)
            .and_then(|line| line.get(x as usize))
            .cloned()
    }
    fn get_mut(&mut self, mut x: isize, mut y: isize) -> Option<&mut CellType> {
        x -= self.offset.0;
        y -= self.offset.1;
        self.grid
            .get_mut(y as usize)
            .and_then(|line| line.get_mut(x as usize))
    }
    fn get_line(&self, mut y: isize) -> Option<&Vec<CellType>> {
        y -= self.offset.1;
        self.grid.get(y as usize)
    }
    fn mark_sensor(&mut self, sensor: &Sensor) {
        let m_dist = manhattan_distance(sensor.sensor, sensor.closestbeacon);
        let x_range = sensor.sensor.0 - m_dist..=sensor.sensor.0 + m_dist;
        let y_range = sensor.sensor.1 - m_dist..=sensor.sensor.1 + m_dist;
        for x in x_range {
            for y in y_range.clone() {
                if manhattan_distance((x, y), sensor.sensor) <= m_dist {
                    if self.get(x as isize, y as isize) == Some(CellType::NoSurvey) {
                        *self.get_mut(x as isize, y as isize).unwrap() = CellType::Surveyed;
                    }
                }
            }
        }

        *self
            .get_mut(sensor.sensor.0 as isize, sensor.sensor.1 as isize)
            .unwrap() = CellType::Sensor;
        *self
            .get_mut(
                sensor.closestbeacon.0 as isize,
                sensor.closestbeacon.1 as isize,
            )
            .unwrap() = CellType::Beacon;
    }
}
impl From<Vec<Sensor>> for Grid {
    fn from(sensors: Vec<Sensor>) -> Self {
        let min_x = sensors
            .iter()
            .map(|s| s.sensor.0 - manhattan_distance(s.sensor, s.closestbeacon))
            .min()
            .unwrap();
        let max_x = sensors
            .iter()
            .map(|s| s.sensor.0 + manhattan_distance(s.sensor, s.closestbeacon))
            .max()
            .unwrap();
        let min_y = sensors
            .iter()
            .map(|s| s.sensor.1 - manhattan_distance(s.sensor, s.closestbeacon))
            .min()
            .unwrap();
        let max_y = sensors
            .iter()
            .map(|s| s.sensor.1 + manhattan_distance(s.sensor, s.closestbeacon))
            .max()
            .unwrap();
        let offset = (min_x, min_y);
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let grid = vec![vec![CellType::NoSurvey; width as usize]; height as usize];
        let mut grid = Grid { grid, offset };
        for sensor in sensors {
            println!("Marking sensor : {sensor:?}");
            grid.mark_sensor(&sensor);
        }
        // dbg!(&grid);
        grid
    }
}
impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Grid properties : Offsets : {:?} - Grid size : ({},{})",
            self.offset,
            self.grid[0].len(),
            self.grid.len()
        )?;
        for line in &self.grid {
            for cell in line {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellType {
    NoSurvey,
    Sensor,
    Beacon,
    Surveyed,
}
impl Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            CellType::NoSurvey => '.',
            CellType::Sensor => 'S',
            CellType::Beacon => 'B',
            CellType::Surveyed => '#',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug)]
struct Sensor {
    sensor: (isize, isize),
    closestbeacon: (isize, isize),
    m_dist: isize,
}
impl FromStr for Sensor {
    type Err = String;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let str = str.split(|c: char| "=,:".contains(c)).collect::<Vec<_>>();
        // 0 : Sensor at x
        // 1 : value of x
        // 2 : , y
        // 3 : value of y
        // 4 : closest beacon is at x
        // 5 : value of beacon relative x
        // 6 : , y
        // 7 : value of beacon relative y

        let sensor_x = str
            .get(1)
            .and_then(|x| x.parse::<isize>().ok())
            .ok_or("Failed to parse sensor x")?;
        let sensor_y = str
            .get(3)
            .and_then(|y| y.parse::<isize>().ok())
            .ok_or("Failed to parse sensor y")?;
        let rel_beacon_x = str
            .get(5)
            .and_then(|x| x.parse::<isize>().ok())
            .ok_or("Failed to parse relative beacon x")?;
        let rel_beacon_y = str
            .get(7)
            .and_then(|y| y.parse::<isize>().ok())
            .ok_or("Failed to parse relative beacon y")?;
        let sensor = Sensor {
            sensor: (sensor_x, sensor_y),
            closestbeacon: (rel_beacon_x, rel_beacon_y),
            m_dist: manhattan_distance((sensor_x, sensor_y), (rel_beacon_x, rel_beacon_y)),
        };
        Ok(sensor)
    }
}

impl Sensor {
    fn line_range(&self, y: isize) -> Option<(isize, isize)> {
        use std::cmp::max;
        use std::cmp::min;
        let miny = min(self.sensor.1, y);
        let maxy = max(self.sensor.1, y);
        let dist = maxy - miny;
        let remain = self.m_dist - dist;
        if remain < 0 {
            // println!("Sensor : {self:?}\nDist : {dist} / {remain}. None");
            None
        } else {
            // println!(
            //     "Sensor : {self:?}\nDist : {dist} / {remain}. Some({},{})",
            //     self.sensor.0 - dist,
            //     self.sensor.0 + dist
            // );
            Some((self.sensor.0 - remain, self.sensor.0 + remain))
        }
    }
}

fn manhattan_distance(point_a: (isize, isize), point_b: (isize, isize)) -> isize {
    use std::cmp::max;
    use std::cmp::min;
    let x = max(point_a.0, point_b.0) - min(point_a.0, point_b.0);
    let y = max(point_a.1, point_b.1) - min(point_a.1, point_b.1);
    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE, 10), 26)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE, 20), 56000011)
    }
}

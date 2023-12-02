use std::{collections::VecDeque, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let time_start = std::time::Instant::now();
    let heightmap: HeightMap = input.parse().unwrap();
    let time_parse = std::time::Instant::now();
    let part1 = part1(&heightmap);
    let time_part1 = std::time::Instant::now();
    let part2 = part2(&heightmap);
    let time_part2 = std::time::Instant::now();

    println!(
        "Part 1 : {}\nPart 2 : {}\n\nTime parse : {} µs\nTime part 1 : {} µs\nTime part 2 : {} µs\nTime full process : {} µs",
        part1,
        part2,
        time_parse.duration_since(time_start).as_micros(),
        time_part1.duration_since(time_parse).as_micros(),
        time_part2.duration_since(time_part1).as_micros(),
        time_part2.duration_since(time_start).as_micros(),
    )
}

fn part1(heightmap: &HeightMap) -> usize {
    let mut distancemap = vec![vec![None; heightmap.width()]; heightmap.height()];
    distancemap[heightmap.start.1][heightmap.start.0] = Some(0);
    compute_distance_part1(&mut distancemap, heightmap, heightmap.start);
    distancemap[heightmap.finish.1][heightmap.finish.0].unwrap() as usize
}

fn part2(heightmap: &HeightMap) -> usize {
    let mut distancemap = vec![vec![None; heightmap.width()]; heightmap.height()];
    distancemap[heightmap.finish.1][heightmap.finish.0] = Some(0);
    compute_distance_part2(&mut distancemap, heightmap, heightmap.finish);

    heightmap
        .iter_positions_and_height()
        .filter(|(_, height)| *height == 0)
        .filter_map(|((x, y), _)| distancemap[y][x])
        .min()
        .unwrap() as usize
}

fn compute_distance_part1(
    distancemap: &mut [Vec<Option<u16>>],
    heightmap: &HeightMap,
    start_position: (usize, usize),
) {
    let mut position_queue = VecDeque::new();
    position_queue.push_back(start_position);

    while let Some(position) = position_queue.pop_front() {
        let (x, y) = position;
        let distance = distancemap[y][x].unwrap();
        for (next_x, next_y) in heightmap.connected_nodes_part1(position) {
            if distancemap[next_y][next_x].is_some() {
                continue;
            }
            distancemap[next_y][next_x] = Some(distance + 1);
            position_queue.push_back((next_x, next_y));
        }
    }
}

fn compute_distance_part2(
    distancemap: &mut [Vec<Option<u16>>],
    heightmap: &HeightMap,
    start_position: (usize, usize),
) {
    let mut position_queue = VecDeque::new();
    position_queue.push_back(start_position);

    while let Some(position) = position_queue.pop_front() {
        let (x, y) = position;
        let route_length = distancemap[y][x].unwrap();
        if heightmap.get(position).unwrap() == 0 {
            continue;
        }
        for (next_x, next_y) in heightmap.connected_nodes_part2(position) {
            if distancemap[next_y][next_x].is_some() {
                continue;
            }
            distancemap[next_y][next_x] = Some(route_length + 1);
            position_queue.push_back((next_x, next_y));
        }
    }
}

#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    finish: (usize, usize),
}

impl HeightMap {
    fn get(&self, position: (usize, usize)) -> Option<u8> {
        self.map
            .get(position.1)
            .and_then(|line| line.get(position.0).cloned())
    }
    fn width(&self) -> usize {
        self.map[0].len()
    }
    fn height(&self) -> usize {
        self.map.len()
    }
    fn neighbour_positions(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = position;
        let mut positions = Vec::new();
        //up
        if y != 0 {
            positions.push((x, y - 1))
        }
        //down
        if y + 1 != self.height() {
            positions.push((x, y + 1))
        }
        //left
        if x != 0 {
            positions.push((x - 1, y))
        }
        //right
        if x + 1 != self.width() {
            positions.push((x + 1, y))
        }
        positions
    }

    fn connected_nodes_part1(
        &self,
        position: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let height = self.get(position).unwrap();
        self.neighbour_positions(position)
            .into_iter()
            .filter_map(move |next_pos| {
                let next_height = self.get(next_pos).unwrap();
                (next_height <= height + 1).then_some(next_pos)
            })
    }

    fn connected_nodes_part2(
        &self,
        position: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let height = self.get(position).unwrap();
        self.neighbour_positions(position)
            .into_iter()
            .filter_map(move |next_pos| {
                let next_height = self.get(next_pos).unwrap();
                (next_height >= height - 1).then_some(next_pos)
            })
    }

    fn iter_positions_and_height(&self) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
        self.map.iter().enumerate().flat_map(move |(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, &height)| ((x, y), height))
        })
    }
}

impl FromStr for HeightMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut finish = None;

        let map = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.char_indices()
                    .map(|(x, c)| match c {
                        'a'..='z' => c as u8 - b'a',
                        'S' => {
                            start = Some((x, y));
                            0
                        }
                        'E' => {
                            finish = Some((x, y));
                            b'z' - b'a'
                        }
                        _ => panic!("Unacceptable character '{c}'"),
                    })
                    .collect()
            })
            .collect();

        Ok(HeightMap {
            map,
            start: start.unwrap(),
            finish: finish.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn test_part1() {
        let heightmap = SAMPLE.parse().unwrap();
        assert_eq!(part1(&heightmap), 31);
    }

    #[test]
    fn test_part2() {
        let heightmap = SAMPLE.parse().unwrap();
        assert_eq!(part2(&heightmap), 29);
    }
}

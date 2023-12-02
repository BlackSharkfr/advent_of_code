use std::ops::{Add, Deref, DerefMut};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 : {}", part1(input));
    println!("Part 2 : {}", part2(input));
}

type HeightMap = Vec<Vec<i16>>;

#[derive(Debug)]
struct VisibilityMap(Vec<Vec<bool>>);
impl Deref for VisibilityMap {
    type Target = Vec<Vec<bool>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for VisibilityMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn parse_heightmap(input: &str) -> HeightMap {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .unwrap_or_else(|| panic!("Expected '{c}' to be a digit"))
                        as i16
                })
                .collect()
        })
        .collect()
}

fn visibility(heightmap: &HeightMap, from_direction: Direction) -> VisibilityMap {
    let height = heightmap.len();
    let width = match heightmap.get(0) {
        None => return VisibilityMap(Vec::new()),
        Some(row) => row.len(),
    };
    let mut visibility = VisibilityMap(
        (0..height)
            .map(|_| (0..width).map(|_| false).collect())
            .collect(),
    );
    match from_direction {
        Direction::North => {
            for col_index in 0..width {
                let mut tallest = -1;
                for row_index in 0..height {
                    let tree = get_tree_height(heightmap, row_index, col_index);
                    if tree > tallest {
                        tallest = tree;
                        visibility[row_index][col_index] = true;
                    }
                }
            }
        }
        Direction::South => {
            for col_index in 0..width {
                let mut tallest = -1;
                for row_index in (0..height).rev() {
                    let tree = get_tree_height(heightmap, row_index, col_index);
                    if tree > tallest {
                        tallest = tree;
                        visibility[row_index][col_index] = true;
                    }
                }
            }
        }
        Direction::East => {
            for row_index in 0..height {
                let mut tallest = -1;
                for col_index in (0..width).rev() {
                    let tree = get_tree_height(heightmap, row_index, col_index);
                    if tree > tallest {
                        tallest = tree;
                        visibility[row_index][col_index] = true;
                    }
                }
            }
        }
        Direction::West => {
            for row_index in 0..height {
                let mut tallest = -1;
                for col_index in 0..width {
                    let tree = get_tree_height(heightmap, row_index, col_index);
                    if tree > tallest {
                        tallest = tree;
                        visibility[row_index][col_index] = true;
                    }
                }
            }
        }
    }
    visibility
}

fn get_tree_height(heightmap: &HeightMap, row_index: usize, col_index: usize) -> i16 {
    *heightmap
        .get(row_index)
        .and_then(|row| row.get(col_index))
        .unwrap_or_else(|| {
            panic!(
                "Index out of bounds. Cannot read Height map (row {row_index}, column {col_index})"
            )
        })
}

fn count_part1(map: &VisibilityMap) -> usize {
    map.iter()
        .map(|line| line.iter().filter(|item| **item).count())
        .sum()
}

impl Add for VisibilityMap {
    type Output = VisibilityMap;
    fn add(self, rhs: Self) -> Self::Output {
        VisibilityMap(self.iter().enumerate().map(|(row_index, row)| {
            let rhs_row = rhs.get(row_index).unwrap_or_else(|| panic!("Index out of bounds accessing RHS row index {row_index} out of {}", rhs.len() - 1));
            row.iter().enumerate().map(|(col_index, &lhs_value)| {
                let &rhs_value = rhs_row.get(col_index).unwrap_or_else(|| panic!("Index out of bounds accessing RHS col index {col_index} out of {}, within row {row_index}", rhs_row.len()));
                lhs_value | rhs_value
            }).collect()
        }).collect())
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn part1(input: &str) -> usize {
    let heightmap = parse_heightmap(input);
    let visibility = visibility(&heightmap, Direction::West)
        + visibility(&heightmap, Direction::East)
        + visibility(&heightmap, Direction::North)
        + visibility(&heightmap, Direction::South);
    count_part1(&visibility)
}

fn part2(input: &str) -> usize {
    let heightmap = parse_heightmap(input);
    let height = heightmap.len();
    let width = match heightmap.get(0) {
        None => return 0,
        Some(row) => row.len(),
    };
    heightmap
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(col_index, &tree)| {
                    let score_north = {
                        let mut score = 0;
                        for r in (0..row_index).rev() {
                            let look_at_tree = get_tree_height(&heightmap, r, col_index);
                            score += 1;
                            if look_at_tree >= tree {
                                break;
                            }
                        }
                        score
                    };
                    let score_south = {
                        let mut score = 0;
                        for r in (row_index + 1)..height {
                            let look_at_tree = get_tree_height(&heightmap, r, col_index);
                            score += 1;
                            if look_at_tree >= tree {
                                break;
                            }
                        }
                        score
                    };
                    let score_east = {
                        let mut score = 0;
                        for c in (col_index + 1)..width {
                            let look_at_tree = get_tree_height(&heightmap, row_index, c);
                            score += 1;
                            if look_at_tree >= tree {
                                break;
                            }
                        }
                        score
                    };
                    let score_west = {
                        let mut score = 0;
                        for c in (0..col_index).rev() {
                            let look_at_tree = get_tree_height(&heightmap, row_index, c);
                            score += 1;
                            if look_at_tree >= tree {
                                break;
                            }
                        }
                        score
                    };
                    let score = score_east * score_south * score_west * score_north;
                    if score > 100000 {
                        println!("Tree ({row_index} {col_index}) : Score {score}.\t North: {score_north}, South: {score_south}, East: {score_east}, West: {score_west}");
                    }
                    score
                })
                .max()
                .unwrap_or_default()
        })
        .max()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 8);
    }
}

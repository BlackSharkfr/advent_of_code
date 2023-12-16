use aoc::Aoc;
use bitflags::bitflags;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = usize;
    const DAY_NUMBER: u8 = 16;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let (_, tiles) = parsers::tile_map(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));
        let start = (Tile::RayEast, (0, 0));
        brute_force::compute_path(start, tiles)
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let (_, mirrors) =
            parsers::tile_map(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));
        brute_force::compute_part2(mirrors)
    }
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Copy,Clone)]
    pub struct Direction: u8 {
        const North = 0b0001;
        const South = 0b0010;
        const East =  0b0100;
        const West =  0b1000;
    }

    #[derive(Debug, PartialEq, Eq, Copy,Clone)]
    pub struct Tile: u8 {
        const RayNorth = 1 << 0;
        const RaySouth = 1 << 1;
        const RayEast =  1 << 2;
        const RayWest =  1 << 3;
        const AnyRay = Self::RayNorth.bits() | Self::RaySouth.bits() | Self::RayEast.bits() | Self::RayWest.bits();
        const MirrorNS = 1 << 4;
        const MirrorEW = 1 << 5;
        const MirrorNE = 1 << 6;
        const MirrorSE = 1 << 7;
        const AnyMirror = Self::MirrorNS.bits() | Self::MirrorEW.bits() | Self::MirrorNE.bits() | Self::MirrorSE.bits() ;
    }
}

impl Tile {
    fn raywalk(&mut self, movement: Self) -> Self {
        if self.contains(movement) {
            return Self::empty();
        }
        *self |= movement;

        // Go straight
        if !self.intersects(Self::AnyMirror) {
            return movement;
        }
        if self.contains(Self::MirrorNE) {
            return match movement {
                Self::RayEast => Self::RayNorth,
                Self::RaySouth => Self::RayWest,
                Self::RayWest => Self::RaySouth,
                Self::RayNorth => Self::RayEast,
                _ => unreachable!(),
            };
        }
        if self.contains(Self::MirrorSE) {
            return match movement {
                Self::RayEast => Self::RaySouth,
                Self::RaySouth => Self::RayEast,
                Self::RayWest => Self::RayNorth,
                Self::RayNorth => Self::RayWest,
                _ => unreachable!(),
            };
        }
        if self.contains(Self::MirrorEW) {
            return match movement {
                Self::RayEast | Self::RayWest => movement,
                Self::RaySouth | Self::RayNorth => Self::RayWest | Self::RayEast,
                _ => unreachable!(),
            };
        }
        match movement {
            Self::RayEast | Self::RayWest => Self::RayNorth | Self::RaySouth,
            Self::RaySouth | Self::RayNorth => movement,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mirror {
    SplitVertical,
    SplitHorizontal,
    NorthEast,
    SouthEast,
}

pub mod brute_force {
    use super::*;
    pub fn part1(input: &str) -> usize {
        let (_, tiles) = parsers::tile_map(input).unwrap();
        let start = (Tile::RayEast, (0, 0));
        compute_path(start, tiles)
    }

    pub fn compute_path(start: (Tile, (usize, usize)), mut tiles: Vec<Vec<Tile>>) -> usize {
        let height = tiles.len();
        let width = tiles[0].len();
        let mut beams = Vec::from([start]);
        while let Some((beam, (x, y))) = beams.pop() {
            let tile = &mut tiles[y][x];
            for next in tile.raywalk(beam).iter() {
                let beam = match next {
                    Tile::RayEast if x != width - 1 => (next, (x + 1, y)),
                    Tile::RayWest if x != 0 => (next, (x - 1, y)),
                    Tile::RayNorth if y != 0 => (next, (x, y - 1)),
                    Tile::RaySouth if y != height - 1 => (next, (x, y + 1)),
                    _ => continue,
                };
                beams.push(beam);
            }
        }
        tiles
            .iter()
            .flat_map(|line| line.iter())
            .filter(|tile| tile.intersects(Tile::AnyRay))
            .count()
    }

    pub fn compute_part2(tiles: Vec<Vec<Tile>>) -> usize {
        let height = tiles.len();
        let width = tiles[0].len();
        starting_positions(height, width)
            .par_bridge()
            .map(|start| compute_path(start, tiles.clone()))
            .max()
            .unwrap()
    }

    fn starting_positions(
        height: usize,
        width: usize,
    ) -> impl Iterator<Item = (Tile, (usize, usize))> {
        let top = (0..width).map(move |x| (Tile::RaySouth, (x, 0)));
        let bottom = (0..width).map(move |x| (Tile::RayNorth, (x, height - 1)));
        let left = (0..height).map(move |y| (Tile::RayEast, (0, y)));
        let right = (0..height).map(move |y| (Tile::RayWest, (width - 1, y)));
        top.chain(bottom).chain(left).chain(right)
    }
}

pub mod parsers {
    use nom::{
        character::complete::{anychar, line_ending},
        multi::{many1, separated_list1},
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use super::*;

    pub fn mirror_map(input: &str) -> IResult<&str, Vec<Vec<Option<Mirror>>>> {
        separated_list1(line_ending, many1(mirror))(input)
    }

    fn mirror(input: &str) -> IResult<&str, Option<Mirror>> {
        anychar
            .map_res(|c: char| match c {
                '.' => Ok(None),
                '/' => Ok(Some(Mirror::NorthEast)),
                '\\' => Ok(Some(Mirror::SouthEast)),
                '|' => Ok(Some(Mirror::SplitVertical)),
                '-' => Ok(Some(Mirror::SplitHorizontal)),
                _ => Err(()),
            })
            .parse(input)
    }

    pub fn tile_map(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
        separated_list1(line_ending, many1(tile))(input)
    }

    fn tile(input: &str) -> IResult<&str, Tile> {
        anychar
            .map_res(|c| match c {
                '.' => Ok(Tile::empty()),
                '/' => Ok(Tile::MirrorNE),
                '\\' => Ok(Tile::MirrorSE),
                '|' => Ok(Tile::MirrorNS),
                '-' => Ok(Tile::MirrorEW),
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
        Day::test_part1(46)
    }

    #[test]
    fn test_part1_tiles() {
        let input = Day::SAMPLE_PART1;
        assert_eq!(46, brute_force::part1(input));

        let input = Day::INPUT;
        assert_eq!(8112, brute_force::part1(input));
    }

    #[test]
    fn test_part2() {
        Day::test_part2(51)
    }

    #[test]
    fn test_part2_tiles() {
        let input = Day::SAMPLE_PART1;
        let tiles = parsers::tile_map(input).unwrap().1;
        assert_eq!(51, brute_force::compute_part2(tiles));

        let input = Day::INPUT;
        let tiles = parsers::tile_map(input).unwrap().1;
        assert_eq!(8314, brute_force::compute_part2(tiles));
    }
}

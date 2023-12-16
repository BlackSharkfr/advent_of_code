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
        brute_force::compute_path(Tile::RayEast, (0, 0), tiles)
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let (_, mirrors) =
            parsers::tile_map(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));
        brute_force::compute_part2(mirrors)
    }
}

pub type TileMap = Vec<Vec<Tile>>;

bitflags! {
    /// Single byte combining data for both the mirrors and the ray directions
    #[derive(Debug, PartialEq, Eq, Copy,Clone)]
    pub struct Tile: u8 {
        const RayNorth = 1 << 0;
        const RaySouth = 1 << 1;
        const RayEast =  1 << 2;
        const RayWest =  1 << 3;
        const AllRays = Self::RayNorth.bits() | Self::RaySouth.bits() | Self::RayEast.bits() | Self::RayWest.bits();
        const SplitterNS = 1 << 4;
        const SplitterEW = 1 << 5;
        const MirrorNE = 1 << 6;
        const MirrorSE = 1 << 7;
        const AllMirrors = Self::SplitterNS.bits() | Self::SplitterEW.bits() | Self::MirrorNE.bits() | Self::MirrorSE.bits() ;
    }
}

impl Tile {
    /// Write the light ray in `self` and outputs the next ray
    /// Input rays must be "pure" rays (single direction, no mirror)
    /// Output rays may contain 0, 1 or 2 rays combined
    fn raywalk(&mut self, input_ray: Self) -> Self {
        // Tile already visited with the same ray direction.
        if self.contains(input_ray) {
            return Self::empty();
        }

        *self |= input_ray;

        // Go straight
        if !self.intersects(Self::AllMirrors) {
            return input_ray;
        }

        // Bouncing mirrors
        if self.intersects(Self::MirrorNE | Self::MirrorSE) {
            return match (self.contains(Self::MirrorNE), input_ray) {
                (true, Self::RayEast) => Self::RayNorth,
                (true, Self::RaySouth) => Self::RayWest,
                (true, Self::RayWest) => Self::RaySouth,
                (true, Self::RayNorth) => Self::RayEast,
                (false, Self::RayEast) => Self::RaySouth,
                (false, Self::RaySouth) => Self::RayEast,
                (false, Self::RayWest) => Self::RayNorth,
                (false, Self::RayNorth) => Self::RayWest,
                _ => unreachable!(),
            };
        }

        // Splitting Mirrors
        match (self.contains(Self::SplitterEW), input_ray) {
            (true, Self::RayEast) | (true, Self::RayWest) => input_ray,
            (true, Self::RaySouth) | (true, Self::RayNorth) => Self::RayEast | Self::RayWest,
            (false, Self::RaySouth) | (false, Self::RayNorth) => input_ray,
            (false, Self::RayEast) | (false, Self::RayWest) => Self::RaySouth | Self::RayNorth,
            _ => unreachable!(),
        }
    }
}

pub mod brute_force {
    use super::*;
    pub fn part1(input: &str) -> usize {
        let (_, tiles) = parsers::tile_map(input).unwrap();
        compute_path(Tile::RayEast, (0, 0), tiles)
    }

    pub fn compute_path(
        start_ray: Tile,
        start_coords: (usize, usize),
        mut tiles: TileMap,
    ) -> usize {
        let height = tiles.len();
        let width = tiles[0].len();

        let mut beams = Vec::from([(start_ray, start_coords)]);
        while let Some((ray, (x, y))) = beams.pop() {
            let tile = &mut tiles[y][x];
            for next_ray in tile.raywalk(ray).iter() {
                let next_coords = match next_ray {
                    Tile::RayEast if x != width - 1 => (x + 1, y),
                    Tile::RayWest if x != 0 => (x - 1, y),
                    Tile::RayNorth if y != 0 => (x, y - 1),
                    Tile::RaySouth if y != height - 1 => (x, y + 1),
                    _ => continue,
                };
                beams.push((next_ray, next_coords));
            }
        }

        tiles
            .iter()
            .flat_map(|line| line.iter())
            .filter(|tile| tile.intersects(Tile::AllRays))
            .count()
    }

    pub fn compute_part2(tiles: TileMap) -> usize {
        starting_positions(&tiles)
            .par_bridge()
            .map(|(ray, coords)| compute_path(ray, coords, tiles.clone()))
            .max()
            .unwrap()
    }

    fn starting_positions(tiles: &TileMap) -> impl Iterator<Item = (Tile, (usize, usize))> {
        let height = tiles.len();
        let width = tiles[0].len();
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

    pub fn tile_map(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
        separated_list1(line_ending, many1(tile))(input)
    }

    fn tile(input: &str) -> IResult<&str, Tile> {
        anychar
            .map_res(|c| match c {
                '.' => Ok(Tile::empty()),
                '/' => Ok(Tile::MirrorNE),
                '\\' => Ok(Tile::MirrorSE),
                '|' => Ok(Tile::SplitterNS),
                '-' => Ok(Tile::SplitterEW),
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

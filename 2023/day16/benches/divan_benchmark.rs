use aoc::Aoc;
use day16::{parsers, Day, Tile};
use divan::{Bencher, Divan};
use std::time::Duration;
fn main() {
    Divan::default()
        .min_time(Duration::from_millis(500))
        .max_time(Duration::from_millis(500))
        .main()
}

#[divan::bench]
fn nom_parser(bencher: Bencher) {
    bencher
        .with_inputs(|| Day::INPUT)
        .bench_values(|input| parsers::tile_map(input))
}

mod part1 {
    use super::*;

    #[divan::bench]
    fn brute_force(bencher: Bencher) {
        bencher
            .with_inputs(|| parsers::tile_map(Day::INPUT).unwrap())
            .bench_values(|(_, mirrors)| {
                day16::brute_force::compute_path(Tile::RayEast, (0, 0), mirrors)
            })
    }
}

mod part2 {
    use super::*;

    #[divan::bench]
    fn brute_force(bencher: Bencher) {
        bencher
            .with_inputs(|| parsers::tile_map(Day::INPUT).unwrap().1)
            .bench_values(|tiles| day16::brute_force::compute_part2(tiles))
    }
}

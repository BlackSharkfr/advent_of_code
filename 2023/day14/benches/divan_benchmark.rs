use aoc::Aoc;
use day14::Day;
use divan::{Bencher, Divan};
use std::time::Duration;
fn main() {
    Divan::default()
        .min_time(Duration::from_millis(500))
        .max_time(Duration::from_millis(500))
        .main()
}

mod part1 {
    use day14::parsers;

    use super::*;
    #[divan::bench]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part1(input))
    }

    #[divan::bench]
    fn nom_parser(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| parsers::grid(input))
    }

    #[divan::bench]
    fn move_north(bencher: Bencher) {
        let input = Day::INPUT;
        let (_, grid) = parsers::grid(input).unwrap();
        bencher
            .with_inputs(|| grid.clone())
            .bench_values(|mut grid| day14::move_north(&mut grid))
    }

    #[divan::bench]
    fn move_south(bencher: Bencher) {
        let input = Day::INPUT;
        let (_, grid) = parsers::grid(input).unwrap();
        bencher
            .with_inputs(|| grid.clone())
            .bench_values(|mut grid| day14::move_south(&mut grid))
    }

    #[divan::bench]
    fn move_west(bencher: Bencher) {
        let input = Day::INPUT;
        let (_, grid) = parsers::grid(input).unwrap();
        bencher
            .with_inputs(|| grid.clone())
            .bench_values(|mut grid| day14::move_west(&mut grid))
    }

    #[divan::bench]
    fn move_east(bencher: Bencher) {
        let input = Day::INPUT;
        let (_, grid) = parsers::grid(input).unwrap();
        bencher
            .with_inputs(|| grid.clone())
            .bench_values(|mut grid| day14::move_east(&mut grid))
    }
}

mod part2 {
    use day14::dual_thread;

    use super::*;
    #[divan::bench]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part2(input))
    }

    #[divan::bench]
    fn dual_thread(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| dual_thread::part2(input))
    }
}

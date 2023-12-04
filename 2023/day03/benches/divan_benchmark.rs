use aoc::Aoc;
use day03::Day;
use divan::Bencher;
use std::time::Duration;
fn main() {
    divan::main()
}

mod part1 {
    use day03::{part1_attempt1, part1_attempt2, part1_attempt3, part1_pretty};

    use super::*;
    #[divan::bench(min_time = Duration::from_millis(500))]
    fn attempt1(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part1_attempt1(input))
    }

    #[divan::bench(min_time = Duration::from_millis(500))]
    fn attempt2(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part1_attempt2(input))
    }
    #[divan::bench(min_time = Duration::from_millis(500))]
    fn attempt3(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part1_attempt3(input))
    }
    #[divan::bench(min_time = Duration::from_millis(500))]
    fn pretty(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part1_pretty(input))
    }
}

mod part2 {
    use day03::{
        part2_attempt1, part2_attempt2, part2_attempt3, part2_parser, part2_pretty, part2_search,
    };

    use super::*;
    #[divan::bench(min_time = Duration::from_millis(500))]
    fn attempt1(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_attempt1(input))
    }

    #[divan::bench(min_time = Duration::from_millis(500))]
    fn attempt2(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_attempt2(input))
    }

    #[divan::bench(min_time = Duration::from_millis(500))]
    fn attemtp3(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_attempt3(input))
    }

    #[divan::bench(min_time = Duration::from_millis(500))]
    fn pretty(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_pretty(input))
    }

    #[divan::bench(min_time = Duration::from_millis(500))]
    fn pretty_parser(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_parser(input))
    }

    #[divan::bench(min_time = Duration::from_millis(500))]
    fn pretty_search(bencher: Bencher) {
        bencher
            .with_inputs(|| part2_parser(Day::INPUT))
            .bench_values(|(numbers, number_coords, gears)| {
                part2_search(&numbers, &number_coords, &gears)
            })
    }
}

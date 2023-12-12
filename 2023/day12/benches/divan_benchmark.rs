use aoc::Aoc;
use day12::Day;
use divan::{Bencher, Divan};
use std::time::Duration;
fn main() {
    Divan::default()
        .min_time(Duration::from_millis(500))
        .max_time(Duration::from_millis(500))
        .main()
}

mod part1 {
    use day12::{part1_brute_force, part1_mutual_cache};

    use super::*;

    #[divan::bench]
    fn cached(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part1(input))
    }

    #[divan::bench]
    fn mutual_cached(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part1_mutual_cache(input))
    }

    #[divan::bench]
    fn brute_force(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part1_brute_force(input))
    }
}

mod part2 {
    use day12::part2_mutual_cache;

    use super::*;
    #[divan::bench]
    fn cached(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part2(input))
    }

    #[divan::bench]
    fn mutual_cached(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_mutual_cache(input))
    }
}

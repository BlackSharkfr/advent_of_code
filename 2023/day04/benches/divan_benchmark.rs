use aoc::Aoc;
use day04::Day;
use divan::{Bencher, Divan};
use std::time::Duration;
fn main() {
    Divan::default()
        .min_time(Duration::from_millis(500))
        .max_time(Duration::from_millis(500))
        .main()
}

mod part1 {
    use day04::part1_rayon;

    use super::*;
    #[divan::bench]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part1(input))
    }

    #[divan::bench]
    fn rayon(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part1_rayon(input))
    }
}

mod part2 {
    use day04::part2_rayon;

    use super::*;
    #[divan::bench]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part2(input))
    }

    #[divan::bench]
    fn rayon(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_rayon(input))
    }
}

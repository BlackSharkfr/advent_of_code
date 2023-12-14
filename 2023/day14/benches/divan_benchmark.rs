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
    use super::*;
    #[divan::bench]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part1(input))
    }
}

mod part2 {
    use super::*;
    #[divan::bench]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part2(input))
    }
}

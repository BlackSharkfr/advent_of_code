use aoc::Aoc;
use day09::Day;
use divan::{Bencher, Divan};
use std::time::Duration;
fn main() {
    Divan::default()
        .min_time(Duration::from_millis(500))
        .max_time(Duration::from_millis(500))
        .main()
}

mod part1_singlethread {
    use day09::{part1_inplace, part1_recursive_inplace};

    use super::*;
    #[divan::bench]
    fn recursive(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part1(input))
    }

    #[divan::bench]
    fn in_place(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part1_inplace(input))
    }
    #[divan::bench]
    fn in_place_recursive(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part1_recursive_inplace(input))
    }
}

mod part2_rayon {
    use day09::{part2_inplace, part2_inplace_recursive};

    use super::*;
    #[divan::bench]
    fn recursive(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part2(input))
    }
    #[divan::bench]
    fn in_place(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_inplace(input))
    }
    #[divan::bench]
    fn in_place_recursive(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_inplace_recursive(input))
    }
}

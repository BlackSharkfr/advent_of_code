use aoc::Aoc;
use day08::Day;
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
    use day08::{part2_hash_str, part2_hash_str_singlethread, part2_hash_string_singlethread};

    use super::*;
    #[divan::bench]
    fn hashmap_string(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part2(input))
    }

    #[divan::bench]
    fn hashmap_str(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_hash_str(input))
    }

    #[divan::bench]
    fn hashmap_str_singlethread(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_hash_str_singlethread(input))
    }
    #[divan::bench]
    fn hashmap_string_singlethread(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_hash_string_singlethread(input))
    }
}

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
    use day08::{
        part2_btree_str, part2_btree_str_singlethread, part2_encoded, part2_encoded_singlethreaded,
        part2_hash_str, part2_hash_str_singlethread, part2_hash_string_singlethread,
    };

    use super::*;
    #[divan::bench]
    fn rayon_hashmap_string(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part2(input))
    }

    #[divan::bench]
    fn rayon_hashmap_str(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_hash_str(input))
    }

    #[divan::bench]
    fn hashmap_str(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_hash_str_singlethread(input))
    }
    #[divan::bench]
    fn hashmap_string(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_hash_string_singlethread(input))
    }

    #[divan::bench]
    fn rayon_btreemap_str(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_btree_str(input))
    }
    #[divan::bench]
    fn btreemap_str(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_btree_str_singlethread(input))
    }

    #[divan::bench]
    fn array_encoded(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_encoded_singlethreaded(input))
    }
    #[divan::bench]
    fn rayon_array_encoded(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| part2_encoded(input))
    }
}

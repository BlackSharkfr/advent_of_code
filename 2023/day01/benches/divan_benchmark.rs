use aoc::Aoc;
use day01::Day;
use divan::Bencher;
use std::time::Duration;
fn main() {
    divan::main()
}

mod part1 {
    use super::*;
    #[divan::bench(min_time = Duration::from_millis(500))]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part1(input))
    }
}

mod part2 {

    use super::*;

    #[divan::bench(min_time = Duration::from_millis(500))]
    fn std_iter(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part2(input))
    }

    // #[divan::bench(min_time = Duration::from_millis(500))]
    // fn same_iter(bencher: Bencher) {
    //     bencher
    //         .with_inputs(|| Day01::INPUT)
    //         .bench_values(|input| day01::part2_same_iter(input))
    // }

    #[divan::bench(min_time = Duration::from_millis(500))]
    fn rayon(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| day01::part2_rayon(input))
    }
}

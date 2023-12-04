use aoc::Aoc;
use day04::Day;
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
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part2(input))
    }
}

use aoc::Aoc;
use day17::Day;
use divan::{Bencher, Divan};
use std::time::Duration;
fn main() {
    Divan::default()
        .sample_count(1)
        .min_time(Duration::from_millis(500))
        .max_time(Duration::from_millis(500))
        .main()
}

mod part1 {
    use super::*;
    #[divan::bench]
    fn my_own_dijkstra(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part1(input))
    }

    #[divan::bench]
    fn pathfinding_dijkstra(bencher: Bencher) {
        bencher.with_inputs(|| Day::INPUT).bench_values(|input| {
            let heatmap = day17::parsers::heat_map(input);
            day17::using_pathfinding::dijkstra(&heatmap, 1, 3)
        })
    }
    #[divan::bench]
    fn pathfinding_astar(bencher: Bencher) {
        bencher.with_inputs(|| Day::INPUT).bench_values(|input| {
            let heatmap = day17::parsers::heat_map(input);
            day17::using_pathfinding::astar(&heatmap, 1, 3)
        })
    }
}

mod part2 {
    use super::*;
    #[divan::bench]
    fn my_own_dijkstra(bencher: Bencher) {
        bencher
            .with_inputs(|| Day::INPUT)
            .bench_values(|input| Day::part2(input))
    }

    #[divan::bench]
    fn pathfinding_dijkstra(bencher: Bencher) {
        bencher.with_inputs(|| Day::INPUT).bench_values(|input| {
            let heatmap = day17::parsers::heat_map(input);
            day17::using_pathfinding::dijkstra(&heatmap, 4, 10)
        })
    }

    #[divan::bench]
    fn pathfinding_astar(bencher: Bencher) {
        bencher.with_inputs(|| Day::INPUT).bench_values(|input| {
            let heatmap = day17::parsers::heat_map(input);
            day17::using_pathfinding::astar(&heatmap, 4, 10)
        })
    }
}

pub trait Aoc {
    const DAY_NUMBER: u8;
    const INPUT: &'static str;
    const SAMPLE_PART1: &'static str;
    const SAMPLE_PART2: &'static str;
    type OUTPUT: PartialEq + std::fmt::Display + std::fmt::Debug;

    fn main() {
        let input = Self::INPUT;
        println!("Day {:02} :", Self::DAY_NUMBER);
        println!("Part 1 : {}", Self::part1(input));
        println!("Part 2 : {}", Self::part2(input));
    }
    fn part1(input: &str) -> Self::OUTPUT;
    fn part2(input: &str) -> Self::OUTPUT;

    fn test_part1(expected: Self::OUTPUT) {
        let input = Self::SAMPLE_PART1;
        assert_eq!(expected, Self::part1(input));
    }
    fn test_part2(expected: Self::OUTPUT) {
        let input = Self::SAMPLE_PART2;
        assert_eq!(expected, Self::part2(input));
    }
}

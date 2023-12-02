use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 : {}", part1(input));
    println!("Part 2 : {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut chars = VecDeque::<char>::with_capacity(5);
    for (index, c) in input.char_indices() {
        chars.push_back(c);
        if chars.len() == 5 {
            chars.pop_front();
        }
        let charset = chars.iter().cloned().collect::<HashSet<char>>();
        if charset.len() == 4 {
            return index + 1;
        }
    }
    panic!("Did not find marker")
}

fn part2(input: &str) -> usize {
    let mut chars = VecDeque::<char>::with_capacity(15);
    for (index, c) in input.char_indices() {
        chars.push_back(c);
        if chars.len() == 15 {
            chars.pop_front();
        }
        let charset = chars.iter().cloned().collect::<HashSet<char>>();
        if charset.len() == 14 {
            return index + 1;
        }
    }
    panic!("Did not find marker");
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = include_str!("sample.txt");
    static SAMPLE2: &str = include_str!("sample2.txt");

    #[test]
    fn test_part1() {
        let samples = SAMPLE.lines().collect::<Vec<_>>();
        assert_eq!(part1(samples[0]), 7);
        assert_eq!(part1(samples[1]), 5);
        assert_eq!(part1(samples[2]), 6);
        assert_eq!(part1(samples[3]), 10);
        assert_eq!(part1(samples[4]), 11);
    }

    #[test]
    fn test_part2() {
        let samples = SAMPLE2.lines().collect::<Vec<_>>();
        assert_eq!(part2(samples[0]), 19);
        assert_eq!(part2(samples[1]), 23);
        assert_eq!(part2(samples[2]), 23);
        assert_eq!(part2(samples[3]), 29);
        assert_eq!(part2(samples[4]), 26);
    }
}

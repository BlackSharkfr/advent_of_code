fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 : {}", part1(input));
    println!("Part 2 : {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let (elf1, elf2) = parse_elves_sections(line);
            (elf1.contains(elf2.start()) && elf1.contains(elf2.end()))
                || (elf2.contains(elf1.start()) && elf2.contains(elf1.end()))
        })
        .count() as u32
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let (elf1, elf2) = parse_elves_sections(line);
            elf1.contains(elf2.start()) || elf2.contains(elf1.start())
        })
        .count() as u32
}

fn parse_elves_sections(
    line: &&str,
) -> (std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>) {
    let mut elves = line.split(',').map(|data| {
        let mut data = data
            .split('-')
            .map(|s| s.parse::<u32>().expect("Ranges should be a number"));
        let start = data.next().expect("Start of range should be defined");
        let end = data.next().expect("End of renge should be defined");
        start..=end
    });
    let elf1 = elves.next().expect("1st elf should have a range");
    let elf2 = elves.next().expect("2nd elf should have a renge");
    (elf1, elf2)
}

#[cfg(test)]
mod tests {

    use super::*;

    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 4);
    }
}

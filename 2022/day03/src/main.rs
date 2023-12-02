fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 : {}", part1(input));
    println!("Part 2 : {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            if chars.len() % 2 != 0 {
                panic!("Line '{line}' has an odd number of characters")
            }
            let mid = chars.len() / 2;
            let pocket1 = &chars[0..mid];
            let pocket2 = &chars[mid..];
            let c = pocket1
                .iter()
                .find(|c| pocket2.contains(*c))
                .unwrap_or_else(|| panic!("No common character found on line '{line}'"));
            priority(*c)
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    if lines.len() % 3 != 0 {
        panic!("Elves are not grouped by 3")
    };
    lines
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .skip(1)
                .fold(group[0].to_string(), |common, elf| {
                    common
                        .chars()
                        .filter(|&c| elf.contains(c))
                        .collect::<String>()
                })
                .chars()
                .next()
                .map(priority)
                .expect("There was no common character")
        })
        .sum::<u32>()
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("Wrong character {c}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 157)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 70)
    }
}

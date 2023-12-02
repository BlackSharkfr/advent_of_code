fn main() {
    let input = include_str!("input.txt");
    let result1 = compute_part1(input);
    let result2 = compute_part2(input);
    println!("Part 1 : {result1}");
    println!("Part 2 : {result2}");
}

fn compute_part1(input: &str) -> u32 {
    let elves_food = parse_elves_food(input);
    let elves_calories = elves_to_calories(elves_food);

    elves_calories.iter().max().cloned().unwrap_or_default()
}

fn compute_part2(input: &str) -> u32 {
    let elves_food = parse_elves_food(input);
    let mut elves_calories = elves_to_calories(elves_food);
    elves_calories.sort();

    let elves_sorted = elves_calories;
    let elves_top3 = elves_sorted
        .iter()
        .rev()
        .take(3)
        .cloned()
        .collect::<Vec<_>>();
    elves_top3.iter().sum::<u32>()
}

fn parse_elves_food(input: &str) -> Vec<Vec<u32>> {
    let lines = input.lines().collect::<Vec<_>>();
    let elves_lines = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
    let elves_food = elves_lines
        .iter()
        .map(|lines| {
            lines
                .iter()
                .filter_map(|line| line.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    elves_food
}

fn elves_to_calories(elves_food: Vec<Vec<u32>>) -> Vec<u32> {
    let elves_calories = elves_food
        .iter()
        .map(|food| food.iter().sum::<u32>())
        .collect::<Vec<_>>();
    elves_calories
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn part1() {
        assert_eq!(compute_part1(SAMPLE), 24000);
    }

    #[test]
    fn part2() {
        assert_eq!(compute_part2(SAMPLE), 45000);
    }
}

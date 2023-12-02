use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1 : {}", part1(INPUT));
    println!("Part 2 : {}", part2(INPUT));
}

fn part1(data: &str) -> usize {
    let mut monkeys = parse_monkeys(data);
    let mut business = vec![0_usize; monkeys.len()];
    for _ in 0..20 {
        for monkey_id in 0..monkeys.len() {
            let monkey = monkeys[monkey_id].clone();
            for item in &monkey.items {
                let mut item = monkey.operation.apply(*item);
                item = item / 3;
                if item % monkey.test_divisible == 0 {
                    monkeys[monkey.on_true].items.push(item)
                } else {
                    monkeys[monkey.on_false].items.push(item)
                }
            }
            business[monkey_id] += monkey.items.len();
            monkeys[monkey_id].items.clear();
        }
    }
    business.sort();
    business[monkeys.len() - 1] * business[monkeys.len() - 2]
}

fn part2(data: &str) -> usize {
    let mut monkeys = parse_monkeys(data);
    let mut business = vec![0_usize; monkeys.len()];
    let common_multiple = monkeys
        .iter()
        .map(|monkey| monkey.test_divisible)
        .product::<usize>();
    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            let monkey = &mut monkeys[index];
            let mut send_to_monkey_true = Vec::new();
            let mut send_to_monkey_false = Vec::new();
            for item in &monkey.items {
                let mut item = monkey.operation.apply(*item);
                item = item % common_multiple;
                if item % monkey.test_divisible == 0 {
                    send_to_monkey_true.push(item)
                } else {
                    send_to_monkey_false.push(item)
                }
            }
            business[index] += monkey.items.len();
            monkey.items.clear();
            let index_true = monkey.on_true;
            let index_false = monkey.on_false;
            monkeys[index_true]
                .items
                .extend_from_slice(&send_to_monkey_true);
            monkeys[index_false]
                .items
                .extend_from_slice(&send_to_monkey_false);
        }
    }

    business.sort();
    business[monkeys.len() - 1] * business[monkeys.len() - 2]
}

fn parse_monkeys(str: &str) -> Vec<Monkey> {
    str.lines()
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|lines| {
            let items = lines[1]
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect();

            let operation = lines[2].parse().unwrap();
            let test_divisible = lines[3]
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();
            let on_true = lines[4]
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            let on_false = lines[5]
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            Monkey {
                items,
                operation,
                test_divisible,
                on_true,
                on_false,
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_divisible: usize,
    on_true: usize,
    on_false: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        match words[0] {
            "+" => {
                let value = words[1].parse().unwrap();
                Ok(Operation::Add(value))
            }
            "*" => {
                if words[1] == "old" {
                    return Ok(Operation::Square);
                }
                let value = words[1].parse().unwrap();
                Ok(Operation::Multiply(value))
            }
            _ => panic!(),
        }
    }
}
impl Operation {
    fn apply(&self, value: usize) -> usize {
        match self {
            Operation::Add(add) => value + add,
            Operation::Multiply(multiply) => value * multiply,
            Operation::Square => value * value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2713310158);
    }
}

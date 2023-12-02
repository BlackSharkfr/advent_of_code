fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 : {}", part1(input));
    println!("Part 2 : {}", part2(input));
}

fn part1(input: &str) -> usize {
    let lists = input.lines().collect::<Vec<_>>();
    let lists = lists
        .split(|line| line.is_empty())
        .map(|lines| {
            lines
                .iter()
                .map(|line| parse_list(line).0)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    lists
        .iter()
        .enumerate()
        .filter_map(|(index, pair)| {
            let a = &pair[0];
            let b = &pair[1];
            match a.cmp(&b) {
                std::cmp::Ordering::Greater => None,
                _ => Some(index + 1),
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut lists = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_list(line).0)
        .collect::<Vec<_>>();

    let a = ListOrNumber::List(vec![ListOrNumber::List(vec![ListOrNumber::Number(2)])]);
    let b = ListOrNumber::List(vec![ListOrNumber::List(vec![ListOrNumber::Number(6)])]);
    lists.push(a.clone());
    lists.push(b.clone());

    // println!("Sort !");
    // lists.sort(|a, b| a.is_order_correct(b));
    lists.sort();

    let index_a = lists.iter().position(|value| *value == a).unwrap() + 1;
    let index_b = lists.iter().position(|value| *value == b).unwrap() + 1;

    index_a * index_b
}

fn parse_list(str: &str) -> (ListOrNumber, &str) {
    let mut str = str.strip_prefix('[').unwrap();
    let mut list = Vec::new();
    while !str.is_empty() {
        if str.starts_with('[') {
            let (inner, remain_str) = parse_list(str);
            list.push(inner);
            str = remain_str;
            continue;
        }
        if str.starts_with(']') {
            return (ListOrNumber::List(list), &str[1..]);
        }
        if str.starts_with(',') {
            str = &str[1..];
            continue;
        }
        let mid = str.find(|c| ",]".contains(c)).unwrap();
        let (num, remain) = str.split_at(mid);
        str = remain;
        if num.is_empty() {
            continue;
        }
        let number = num
            .parse()
            .map_err(|e| format!("Failed to parse {num}. {e}"))
            .unwrap();
        list.push(ListOrNumber::Number(number));
    }
    panic!("List was not closed by char ']'");
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ListOrNumber {
    Number(usize),
    List(Vec<ListOrNumber>),
}
impl Ord for ListOrNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            (Self::List(list_a), Self::Number(_)) if list_a.is_empty() => std::cmp::Ordering::Less,
            (Self::List(_), Self::Number(_)) => self.cmp(&Self::List(vec![other.clone()])),
            (Self::Number(_), Self::List(list_b)) if list_b.is_empty() => {
                std::cmp::Ordering::Greater
            }
            (Self::Number(_), Self::List(_)) => Self::List(vec![self.clone()]).cmp(other),
            (Self::List(list_a), Self::List(list_b)) => {
                let len = list_a.len().max(list_b.len());
                for i in 0..len {
                    let Some(a) = list_a.get(i) else { return std::cmp::Ordering::Less };
                    let Some(b) = list_b.get(i) else { return std::cmp::Ordering::Greater };
                    match a.cmp(b) {
                        std::cmp::Ordering::Equal => (),
                        cmp => return cmp,
                    }
                }
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl PartialOrd for ListOrNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE: &str = include_str!("sample.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 140)
    }
}

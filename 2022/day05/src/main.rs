use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}
impl Move {
    fn move_boxes_part1(self, stacks: &mut [Stack]) {
        for _ in 0..self.amount {
            let c = stacks[self.from]
                .pop()
                .expect("There should have been a char in the stack");
            stacks[self.to].push(c);
        }
    }

    fn move_boxes_part2(self, stacks: &mut [Stack]) {
        let temp_stack = (0..self.amount)
            .map(|_| {
                stacks[self.from]
                    .pop()
                    .expect("There should have been a char in the stack")
            })
            .collect::<Vec<char>>();
        for c in temp_stack.into_iter().rev() {
            stacks[self.to].push(c)
        }
    }
}

impl FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        words.next().filter(|word| *word == "move").ok_or(())?;
        let amount = words.next().ok_or(())?.parse::<usize>().map_err(|_| ())?;

        words.next().filter(|word| *word == "from").ok_or(())?;
        let from = words.next().ok_or(())?.parse::<usize>().map_err(|_| ())? - 1;

        words.next().filter(|word| *word == "to").ok_or(())?;
        let to = words.next().ok_or(())?.parse::<usize>().map_err(|_| ())? - 1;

        Ok(Move { amount, from, to })
    }
}

#[derive(Debug)]
struct LineOfBoxes(Vec<Option<char>>);
impl FromStr for LineOfBoxes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut boxes = Vec::new();
        let chars = s.chars().collect::<Vec<_>>();
        for chunk in chars.chunks(4) {
            match chunk[0..=2] {
                [' ', ' ', ' '] => boxes.push(None),
                ['[', c, ']'] => boxes.push(Some(c)),
                _ => return Err(()),
            }
        }
        if boxes.is_empty() {
            return Err(());
        }
        Ok(LineOfBoxes(boxes))
    }
}

type Stack = Vec<char>;

fn main() {
    let (stacks, moves) = parse_input(INPUT);
    println!("Part1 : {}", part1(moves.clone(), stacks.clone()));
    println!("Part2 : {}", part2(moves, stacks));
}

fn parse_input(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let mut boxes = Vec::new();
    let mut moves = Vec::new();

    for line in input.lines() {
        if let Ok(m) = line.parse::<Move>() {
            moves.push(m);
            continue;
        }
        if let Ok(b) = line.parse::<LineOfBoxes>() {
            boxes.push(b)
        }
    }

    boxes.reverse();
    let height = boxes.len();
    let width = boxes[0].0.len();
    let stacks = (0..width)
        .map(|stack_id| {
            (0..height)
                .filter_map(|height_id| boxes[height_id].0[stack_id])
                .collect()
        })
        .collect();

    (stacks, moves)
}

fn part1(moves: Vec<Move>, mut stacks: Vec<Vec<char>>) -> String {
    for m in moves {
        m.move_boxes_part1(&mut stacks)
    }
    top_of_stacks(stacks)
}

fn top_of_stacks(stacks: Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|stack| {
            stack
                .last()
                .expect("There should be at least one box in the stack")
        })
        .collect()
}
fn part2(moves: Vec<Move>, mut stacks: Vec<Vec<char>>) -> String {
    for m in moves {
        m.move_boxes_part2(&mut stacks)
    }
    top_of_stacks(stacks)
}

#[cfg(test)]
mod tests {

    use super::*;

    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn test_part1() {
        let (stacks, moves) = parse_input(SAMPLE);
        assert_eq!(part1(moves, stacks), "CMZ");
    }

    #[test]
    fn test_part2() {
        let (stacks, moves) = parse_input(SAMPLE);
        assert_eq!(part2(moves, stacks), "MCD");
    }
}

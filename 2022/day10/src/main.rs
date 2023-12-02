use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 : {}", part1(input));
    println!("Part 2 : \n{}", part2(input));
}

enum Instruction {
    Noop,
    Addx(i32),
}
impl Instruction {
    fn expand_cycles(self) -> Vec<Self> {
        match &self {
            Instruction::Noop => vec![self],
            Instruction::Addx(_) => vec![Instruction::Noop, self],
        }
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        match words.next() {
            Some("noop") => Ok(Self::Noop),
            Some("addx") => {
                let value = words.next().ok_or(
                    "Failed to parse Operation::Addx. Expected 2nd word to be a number, found None",
                )?;
                let value = value
                    .parse()
                    .map_err(|e| format!("Failed to parse Operation::Addx. Expected 2nd word to be an i32 number, found '{value}' : {e}"))?;
                Ok(Self::Addx(value))
            }
            None => Err("Failed to parse Operation. Provided string is empty")?,
            _ => Err(format!("Failed to parse Operation. Expected 1st word to be an Operation variant, found '{s}'")),
        }
    }
}

fn part1(input: &str) -> i32 {
    let ops = input
        .lines()
        .flat_map(|line| line.parse::<Instruction>().unwrap().expand_cycles());
    let mut cycle: u32 = 1;
    let mut x = 1;
    let mut sum_signal = 0;
    for op in ops {
        let sum_signal: &mut i32 = &mut sum_signal;
        if cycle % 40 == 20 {
            let signal_strength = cycle as i32 * x;
            *sum_signal += signal_strength;
        }
        if let Instruction::Addx(value) = op {
            x += value;
        }
        cycle += 1;
    }
    sum_signal
}

fn part2(input: &str) -> String {
    let mut pixels = String::with_capacity(41 * 6);
    let mut sprite = 1;
    for (cycle, instruction) in input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .flat_map(Instruction::expand_cycles)
        .enumerate()
    {
        let pixel = cycle % 40;
        let c = if (sprite - 1..=sprite + 1).contains(&(pixel as i32)) {
            '#'
        } else {
            ' '
        };
        pixels.push(c);
        if pixel == 39 {
            pixels.push('\n')
        }
        if let Instruction::Addx(value) = instruction {
            sprite += value
        }
    }
    pixels.pop();
    pixels
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 13140);
    }

    #[test]
    fn test_part2() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            .to_string()
            .replace('.', " ");
        assert_eq!(part2(SAMPLE), expected);
    }
}

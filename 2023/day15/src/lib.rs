use std::{
    array,
    hash::{Hash, Hasher},
};

use aoc::Aoc;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u64;
    const DAY_NUMBER: u8 = 15;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let mut hasher = HolidayHasher::new();
        let mut total = 0;
        for byte in input.as_bytes() {
            if *byte == b',' {
                total += hasher.finish();
                hasher = HolidayHasher::new();
                continue;
            }
            (*byte).hash(&mut hasher);
        }
        total += hasher.finish();
        total
    }

    fn part2(mut input: &str) -> Self::OUTPUT {
        let mut lensboxes: [Vec<(&str, u32)>; 256] = array::from_fn(|_| Vec::new());
        while let Ok((remain, instruction)) = parsers::instruction(input) {
            input = remain;

            let box_index = HolidayHasher::hash_str(instruction.label) as usize;
            let lensbox = &mut lensboxes[box_index];

            match instruction.action {
                Action::AddLens(power) => {
                    match lensbox.iter_mut().find(|(l, _)| *l == instruction.label) {
                        Some((_, p)) => *p = power,
                        None => lensbox.push((instruction.label, power)),
                    }
                }
                Action::RemoveLens => {
                    if let Some(remove_index) =
                        lensbox.iter().position(|(l, _)| *l == instruction.label)
                    {
                        lensbox.remove(remove_index);
                    }
                }
            }
        }
        lensboxes
            .into_iter()
            .zip(1..)
            .map(|(lensbox, box_number)| {
                lensbox
                    .into_iter()
                    .zip(1..)
                    .map(|((_, power), slot_number)| box_number * slot_number * power as u64)
                    .sum::<u64>()
            })
            .sum::<u64>()
    }
}

struct HolidayHasher(u8);
impl Hasher for HolidayHasher {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = self.0.wrapping_add(*byte);
            self.0 = self.0.wrapping_mul(17);
        }
    }
}
impl HolidayHasher {
    fn new() -> Self {
        Self(0)
    }
    fn hash_str(str: &str) -> u64 {
        let mut hasher: HolidayHasher = HolidayHasher::new();
        for b in str.as_bytes() {
            (*b).hash(&mut hasher)
        }
        hasher.finish()
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    pub label: &'a str,
    pub action: Action,
}
#[derive(Debug)]
enum Action {
    AddLens(u32),
    RemoveLens,
}

mod parsers {
    use nom::{
        character::complete::{alpha1, anychar, u32},
        combinator::fail,
        IResult,
    };

    use super::*;
    pub fn instruction(input: &str) -> IResult<&str, Instruction> {
        let (input, label) = alpha1(input)?;
        let (input, action) = action(input)?;
        // remove optionnal ','
        let input = input.get(1..).unwrap_or("");

        let instruction = Instruction { label, action };
        Ok((input, instruction))
    }

    fn action(input: &str) -> IResult<&str, Action> {
        let (remain, c) = anychar(input)?;
        match c {
            '=' => {
                let (input, power) = u32(remain)?;
                Ok((input, Action::AddLens(power)))
            }
            '-' => Ok((remain, Action::RemoveLens)),
            _ => fail(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(30, HolidayHasher::hash_str("rn=1"));
        assert_eq!(253, HolidayHasher::hash_str("cm-"));
        assert_eq!(97, HolidayHasher::hash_str("qp=3"));
        assert_eq!(47, HolidayHasher::hash_str("cm=2"));
        assert_eq!(14, HolidayHasher::hash_str("qp-"));
        assert_eq!(180, HolidayHasher::hash_str("pc=4"));
        assert_eq!(9, HolidayHasher::hash_str("ot=9"));
        assert_eq!(197, HolidayHasher::hash_str("ab=5"));
        assert_eq!(48, HolidayHasher::hash_str("pc-"));
        assert_eq!(214, HolidayHasher::hash_str("pc=6"));
        assert_eq!(231, HolidayHasher::hash_str("ot=7"));

        Day::test_part1(1320)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(145)
    }
}

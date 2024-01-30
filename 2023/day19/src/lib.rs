use std::{ops::Range, str::FromStr};

use aoc::Aoc;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u64;
    const DAY_NUMBER: u8 = 19;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let (workflows, pieces) = parsers::part1(input).unwrap().1;

        let mut total = 0;
        for piece in &pieces {
            let mut current = "in";
            loop {
                let workflow = workflows.get(current).unwrap();
                let next = workflow.process_piece(piece);
                match next {
                    "R" => break,
                    "A" => {
                        total += piece.value();
                        break;
                    }
                    _ => current = next,
                }
            }
        }

        total
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let (workflows, _) = parsers::part1(input).unwrap().1;
        let mut ranges = vec![("in".to_string(), PieceRange::default())];
        let mut count = 0;
        while let Some((name, mut range)) = ranges.pop() {
            match name.as_str() {
                "R" => continue,
                "A" => {
                    count += range.count();
                    continue;
                }
                _ => (),
            }
            let workflow = workflows.get(&name).unwrap();
            for (condition, next) in &workflow.conditions {
                if let Some(new_range) = range.split_condition(condition) {
                    ranges.push((next.clone(), new_range));
                }
            }
            ranges.push((workflow.default.clone(), range));
        }

        count as u64
    }
}

struct Piece {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

#[derive(Debug, Clone)]
struct PieceRange {
    x: Range<u16>,
    m: Range<u16>,
    a: Range<u16>,
    s: Range<u16>,
}
impl Default for PieceRange {
    fn default() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }
}
impl PieceRange {
    fn split_condition(&mut self, condition: &Condition) -> Option<Self> {
        let mut new = self.clone();
        let (range, new_range) = match condition.xmas {
            Xmas::X => (&mut self.x, &mut new.x),
            Xmas::M => (&mut self.m, &mut new.m),
            Xmas::A => (&mut self.a, &mut new.a),
            Xmas::S => (&mut self.s, &mut new.s),
        };
        match condition.ord {
            GreaterOrLesser::Greater => {
                if range.end <= condition.value {
                    return None;
                }
                if range.start <= condition.value {
                    range.end = condition.value + 1;
                    new_range.start = condition.value + 1;
                } else {
                    range.end = range.start;
                }
                Some(new)
            }
            GreaterOrLesser::Lesser => {
                if range.start >= condition.value {
                    return None;
                }
                if range.end >= condition.value {
                    range.start = condition.value;
                    new_range.end = condition.value;
                } else {
                    range.end = range.start;
                }
                Some(new)
            }
        }
    }

    fn count(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

struct WorkFlow {
    conditions: Vec<(Condition, String)>,
    default: String,
}

#[derive(Debug)]
struct Condition {
    xmas: Xmas,
    ord: GreaterOrLesser,
    value: u16,
}

#[derive(Debug)]
enum Xmas {
    X,
    M,
    A,
    S,
}
impl FromStr for Xmas {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Xmas::X),
            "m" => Ok(Xmas::M),
            "a" => Ok(Xmas::A),
            "s" => Ok(Xmas::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum GreaterOrLesser {
    Greater,
    Lesser,
}
impl FromStr for GreaterOrLesser {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(GreaterOrLesser::Lesser),
            ">" => Ok(GreaterOrLesser::Greater),
            _ => Err(()),
        }
    }
}

impl WorkFlow {
    fn process_piece<'a>(&'a self, piece: &Piece) -> &'a str {
        for (condition, destination) in &self.conditions {
            if piece.check_condition(condition) {
                return destination.as_str();
            }
        }
        &self.default
    }
}

impl Piece {
    fn check_condition(&self, condition: &Condition) -> bool {
        let number = match condition.xmas {
            Xmas::X => self.x,
            Xmas::M => self.m,
            Xmas::A => self.a,
            Xmas::S => self.s,
        };
        match condition.ord {
            GreaterOrLesser::Greater => number > condition.value,
            GreaterOrLesser::Lesser => number < condition.value,
        }
    }

    fn value(&self) -> u64 {
        self.x as u64 + self.m as u64 + self.a as u64 + self.s as u64
    }
}

mod parsers {
    use std::collections::HashMap;

    use nom::{
        bytes::complete::{tag, take},
        character::complete::{alpha1, line_ending, multispace1, u16},
        multi::{separated_list0, separated_list1},
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use super::*;

    fn piece(input: &str) -> IResult<&str, Piece> {
        let (input, _) = tag("{x=")(input)?;
        let (input, x) = u16(input)?;
        let (input, _) = tag(",m=")(input)?;
        let (input, m) = u16(input)?;
        let (input, _) = tag(",a=")(input)?;
        let (input, a) = u16(input)?;
        let (input, _) = tag(",s=")(input)?;
        let (input, s) = u16(input)?;
        let (input, _) = tag("}")(input)?;

        let piece = Piece { x, m, a, s };
        Ok((input, piece))
    }

    fn condition(input: &str) -> IResult<&str, Condition> {
        let (input, xmas) = xmas(input)?;
        let (input, ord) = greater_or_lesser(input)?;
        let (input, value) = u16(input)?;
        let condition = Condition { xmas, ord, value };
        Ok((input, condition))
    }

    fn condition_and_name(input: &str) -> IResult<&str, (Condition, String)> {
        let (input, condition) = condition(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, name) = alpha1(input)?;

        Ok((input, (condition, name.to_string())))
    }

    fn xmas(input: &str) -> IResult<&str, Xmas> {
        take(1_usize).parse_from_str().parse(input)
    }

    fn greater_or_lesser(input: &str) -> IResult<&str, GreaterOrLesser> {
        take(1_usize).parse_from_str().parse(input)
    }

    fn workflow(input: &str) -> IResult<&str, WorkFlow> {
        let (input, conditions) = separated_list0(tag(","), condition_and_name)(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, default) = alpha1(input)?;

        let workflow = WorkFlow {
            conditions,
            default: default.to_string(),
        };
        Ok((input, workflow))
    }

    fn named_workflow(input: &str) -> IResult<&str, (String, WorkFlow)> {
        let (input, name) = alpha1(input)?;
        let (input, _) = tag("{")(input)?;
        let (input, workflow) = workflow(input)?;
        let (input, _) = tag("}")(input)?;

        Ok((input, (name.to_string(), workflow)))
    }

    pub fn part1(input: &str) -> IResult<&str, (HashMap<String, WorkFlow>, Vec<Piece>)> {
        let (input, workflows) = separated_list1(line_ending, named_workflow)(input)?;
        let (input, _) = multispace1(input)?;
        let (input, pieces) = separated_list1(line_ending, piece)(input)?;

        Ok((input, (HashMap::from_iter(workflows.into_iter()), pieces)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(19114)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(167409079868000)
    }
}

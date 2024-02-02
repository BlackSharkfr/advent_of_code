use std::collections::{HashMap, VecDeque};

use aoc::Aoc;
use day08::lcm;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u64;
    const DAY_NUMBER: u8 = 20;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> Self::OUTPUT {
        let mut nodes = parsers::part1(input);
        let mut low_count = 0;
        let mut high_count = 0;
        for _ in 0..1000 {
            let mut pulses = VecDeque::from([Button::press()]);
            while let Some(pulse) = pulses.pop_front() {
                match &pulse.pulse {
                    PulseType::Low => low_count += 1,
                    PulseType::High => high_count += 1,
                }
                let Some(node) = nodes.get_mut(&pulse.destination) else {
                    continue;
                };
                for pulse in node.pulse(&pulse) {
                    pulses.push_back(pulse);
                }
            }
        }
        low_count * high_count
    }

    fn part2(input: &str) -> Self::OUTPUT {
        let mut nodes = parsers::part1(input);
        let last_to_rx = nodes
            .iter()
            .find(|(_, node)| node.destination.contains(&"rx".into()))
            .unwrap();
        let last_name = last_to_rx.0.clone();
        let NodeType::Conjunction(inputs) = &last_to_rx.1.nodetype else {
            panic!("Wong node type")
        };
        let mut number_presses =
            HashMap::<String, Option<u64>>::from_iter(inputs.keys().map(|key| (key.clone(), None)));

        for n in 1..=10000 {
            let mut pulses = VecDeque::from([Button::press()]);
            while let Some(pulse) = pulses.pop_front() {
                let Some(node) = nodes.get_mut(&pulse.destination) else {
                    continue;
                };
                for pulse in node.pulse(&pulse) {
                    pulses.push_back(pulse);
                }
                if pulse.destination == last_name && pulse.pulse == PulseType::High {
                    let number = number_presses.get_mut(&pulse.origin).unwrap();
                    if number.is_none() {
                        *number = Some(n)
                    }
                }
            }
            if number_presses.values().all(|n| n.is_some()) {
                break;
            }
        }
        let numbers = number_presses
            .values()
            .map(|n| n.unwrap())
            .collect::<Vec<_>>();
        numbers.into_iter().reduce(lcm).unwrap()
    }
}

struct Button {}
impl Button {
    fn press() -> Pulse {
        Pulse {
            origin: "button".into(),
            pulse: PulseType::Low,
            destination: "broadcaster".into(),
        }
    }
}

#[derive(Debug)]
struct Node {
    nodetype: NodeType,
    destination: Vec<String>,
}
impl Node {
    fn pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        match &mut self.nodetype {
            NodeType::Broadcaster => self
                .destination
                .iter()
                .cloned()
                .map(|destination| Pulse {
                    origin: pulse.destination.clone(),
                    pulse: pulse.pulse.clone(),
                    destination,
                })
                .collect(),
            NodeType::FlipFlop(state) if pulse.pulse == PulseType::Low => {
                state.swap();
                self.destination
                    .iter()
                    .cloned()
                    .map(|destination| Pulse {
                        origin: pulse.destination.clone(),
                        pulse: state.clone(),
                        destination,
                    })
                    .collect()
            }
            NodeType::FlipFlop(_) => Vec::new(),
            NodeType::Conjunction(inputs) => {
                let input = inputs.get_mut(&pulse.origin).unwrap();
                *input = pulse.pulse.clone();
                let out_pulse = match inputs.values().all(|p| *p == PulseType::High) {
                    true => PulseType::Low,
                    false => PulseType::High,
                };
                self.destination
                    .iter()
                    .cloned()
                    .map(|destination| Pulse {
                        origin: pulse.destination.clone(),
                        pulse: out_pulse.clone(),
                        destination,
                    })
                    .collect()
            }
        }
    }
}

#[derive(Debug)]
enum NodeType {
    Broadcaster,
    FlipFlop(PulseType),
    Conjunction(HashMap<String, PulseType>),
}

#[derive(Debug)]
struct Pulse {
    origin: String,
    pulse: PulseType,
    destination: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum PulseType {
    Low,
    High,
}
impl PulseType {
    fn swap(&mut self) {
        *self = match self {
            PulseType::Low => PulseType::High,
            PulseType::High => PulseType::Low,
        }
    }
}

mod parsers {
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::alpha1, multi::separated_list1,
        IResult, Parser,
    };

    use super::*;

    pub fn part1(input: &str) -> HashMap<String, Node> {
        let mut output = HashMap::new();
        let mut input_nodes = HashMap::<String, Vec<String>>::new();
        for line in input.lines() {
            let (name, node) = node(line).unwrap().1;
            for dest in &node.destination {
                let inputs = input_nodes.entry(dest.clone()).or_insert(Vec::new());
                inputs.push(name.clone());
            }
            output.insert(name, node);
        }
        for (name, node) in &mut output {
            if let NodeType::Conjunction(inputs) = &mut node.nodetype {
                for input in input_nodes.get(name).unwrap() {
                    inputs.insert(input.clone(), PulseType::Low);
                }
            }
        }
        output
    }

    fn node(input: &str) -> IResult<&str, (String, Node)> {
        alt((broadcaster, flip_flop, conjunction))(input)
    }

    fn broadcaster(input: &str) -> IResult<&str, (String, Node)> {
        let (input, _) = tag("broadcaster -> ")(input)?;
        let (input, destination) = separated_list1(tag(", "), alpha1.map(String::from))(input)?;
        let node = Node {
            nodetype: NodeType::Broadcaster,
            destination,
        };
        Ok((input, ("broadcaster".into(), node)))
    }

    fn flip_flop(input: &str) -> IResult<&str, (String, Node)> {
        let (input, _) = tag("%")(input)?;
        let (input, name) = alpha1(input)?;
        let (input, _) = tag(" -> ")(input)?;
        let (input, destination) = separated_list1(tag(", "), alpha1.map(String::from))(input)?;
        let node = Node {
            nodetype: NodeType::FlipFlop(PulseType::Low),
            destination,
        };
        Ok((input, (name.into(), node)))
    }

    fn conjunction(input: &str) -> IResult<&str, (String, Node)> {
        let (input, _) = tag("&")(input)?;
        let (input, name) = alpha1(input)?;
        let (input, _) = tag(" -> ")(input)?;
        let (input, destination) = separated_list1(tag(", "), alpha1.map(String::from))(input)?;
        let node = Node {
            nodetype: NodeType::Conjunction(HashMap::new()),
            destination,
        };
        Ok((input, (name.into(), node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(32000000)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(0)
    }
}

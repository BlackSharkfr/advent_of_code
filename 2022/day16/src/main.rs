use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    str::FromStr,
    time::Instant,
};

fn main() {
    let input = include_str!("input.txt");
    let time_start = Instant::now();
    let rooms = parse_rooms(input);
    let time_parse = Instant::now();
    let nodes = simplify_nodes(&rooms);
    let time_nodes = Instant::now();
    let pt1 = part1(&nodes);
    let time_pt1 = Instant::now();
    let pt2 = part2(&nodes);
    let time_pt2 = Instant::now();
    println!(
        "Parse input - time: {} µs",
        time_parse.duration_since(time_start).as_micros()
    );
    println!(
        "Simplify nodes - time: {} µs",
        time_nodes.duration_since(time_parse).as_micros()
    );
    println!(
        "Part 1 - value: {pt1}, time: {} ms",
        time_pt1.duration_since(time_nodes).as_millis()
    );
    println!(
        "Part 2 - value: {pt2}, time: {} ms",
        time_pt2.duration_since(time_pt1).as_millis()
    );
}

type Rooms = HashMap<String, Room>;
type Nodes = HashMap<String, Node>;

fn part1(nodes: &Nodes) -> usize {
    Search::init(&nodes, 1, 30).run()
}

fn part2(nodes: &Nodes) -> usize {
    Search::init(&nodes, 2, 26).run()
}

fn parse_rooms(input: &str) -> Rooms {
    input
        .lines()
        .map(|line| {
            let room = line.parse::<Room>().unwrap();
            (room.id.clone(), room)
        })
        .collect()
}

fn simplify_nodes(rooms: &Rooms) -> Nodes {
    rooms
        .par_iter()
        .map(|(_, departure)| {
            let connections = rooms
                .values()
                .filter(|destination| destination.id != departure.id && destination.flow != 0)
                .map(|destination| {
                    let mut queue = VecDeque::from([(departure.id.as_str(), 0)]);
                    let mut visited = Vec::new();
                    loop {
                        let (current_id, connect_time) = queue.pop_front().unwrap();
                        if current_id == destination.id.as_str() {
                            return Connection {
                                id: current_id.to_string(),
                                connect_time,
                            };
                        }
                        visited.push(current_id);
                        let current = rooms.get(current_id).unwrap();
                        let connect_time = connect_time + 1;
                        for id in current
                            .connections
                            .iter()
                            .filter(|id| !visited.contains(&id.as_str()))
                        {
                            queue.push_back((id.as_str(), connect_time))
                        }
                    }
                })
                .collect();
            (
                departure.id.clone(),
                Node {
                    id: departure.id.clone(),
                    flow: departure.flow,
                    connections,
                },
            )
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Room {
    id: String,
    flow: usize,
    connections: Vec<String>,
}
impl FromStr for Room {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Valve AA has flow rate=0; tunnels lead to valves BB, CC, DD
        // 0     1  2   3    4       5       6    7  8      9..
        let words = s.split_whitespace().collect::<Vec<_>>();
        let id = words[1].to_string();
        let flow = words[4]
            .strip_prefix("rate=")
            .unwrap()
            .strip_suffix(';')
            .unwrap()
            .parse()
            .unwrap();
        let connections = words[9..]
            .iter()
            .map(|word| word.replace(',', ""))
            .collect();

        Ok(Room {
            id,
            flow,
            connections,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    id: String,
    flow: usize,
    connections: Vec<Connection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Connection {
    id: String,
    connect_time: usize,
}

#[derive(Clone)]
struct Search<'a> {
    nodes: &'a Nodes,
    visited: HashSet<&'a str>,
    actors: Vec<Actor<'a>>,
    flow: usize,
}
impl Debug for Search<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Flow : {}, Visited : {:?}\nActors : {:?}",
            self.flow, self.visited, self.actors
        )
    }
}

#[derive(Clone, Debug)]
struct Actor<'a> {
    time: usize,
    node: &'a Node,
}

impl<'a> Search<'a> {
    fn init(nodes: &'a Nodes, number_of_actors: usize, time_available: usize) -> Self {
        let start_node = nodes.get("AA").unwrap();
        Self {
            actors: vec![
                Actor {
                    time: time_available,
                    node: start_node
                };
                number_of_actors
            ],
            nodes,
            flow: 0,
            visited: [start_node.id.as_str()].into(),
        }
    }
    fn run(mut self) -> usize {
        self.actors.sort_by_key(|actor| actor.time);
        let Some(actor) = self.actors.last() else { return self.flow };

        actor
            .node
            .connections
            .par_iter()
            .filter(|connection| {
                actor.time > connection.connect_time + 2
                    && !self.visited.contains(&connection.id.as_str())
            })
            .map(|connection| {
                let mut next = self.clone();
                let mut actor = next.actors.last_mut().unwrap();
                actor.time -= connection.connect_time + 1;
                actor.node = next.nodes.get(&connection.id).unwrap();
                next.visited.insert(&actor.node.id);
                next.flow += actor.time * actor.node.flow;
                next.run()
            })
            .max()
            .unwrap_or_else(|| {
                self.actors.pop();
                self.run()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE: &str = include_str!("sample.txt");
    #[test]
    fn test_part1() {
        let rooms = parse_rooms(SAMPLE);
        let nodes = simplify_nodes(&rooms);
        assert_eq!(part1(&nodes), 1651)
    }
    #[test]
    fn test_part2() {
        let rooms = parse_rooms(SAMPLE);
        let nodes = simplify_nodes(&rooms);
        assert_eq!(part2(&nodes), 1707)
    }
}

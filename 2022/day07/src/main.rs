use std::str::FromStr;

const TOTAL_SYSTEM: usize = 70000000;
const REQUIRED_FREE: usize = 30000000;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 : {}", part1(input));
    println!("Part 2 : {}", part2(input));
}

fn part1(input: &str) -> usize {
    let items = parse_items(input);
    let (size, forced) = items.sum_part1();
    if size <= 100_000 {
        size + forced
    } else {
        forced
    }
}

fn parse_items(input: &str) -> Item {
    let mut items = Item::new();
    let mut hierarchy = Vec::new();
    for data in input.lines().map(|line| line.parse::<DataType>().unwrap()) {
        match data {
            DataType::CdInit => (),
            DataType::CdBack => {
                hierarchy
                    .pop()
                    .expect("Hierarchy should have contained at least one item");
            }
            DataType::CdNode(node) => hierarchy.push(node),
            DataType::Ls => (),
            DataType::Item(item) => {
                let dir = items.get_mut(&hierarchy).unwrap();
                let Node::Dir(dir) = &mut dir.node else { panic!("Cannot add item '{}' to Node '{}' it is not a directory", item.name, dir.name) };
                dir.push(item);
            }
        }
    }
    items
}

fn part2(input: &str) -> usize {
    let items = parse_items(input);
    let free = TOTAL_SYSTEM - items.size();
    let to_delete = REQUIRED_FREE - free;
    let sizes = items.dir_sizes();
    sizes
        .into_iter()
        .filter(|size| *size >= to_delete)
        .min()
        .unwrap()
}

enum Node {
    Dir(Vec<Item>),
    File(usize),
}

struct Item {
    name: String,
    node: Node,
}
impl Item {
    fn new() -> Self {
        Self {
            name: "/".to_string(),
            node: Node::Dir(Vec::new()),
        }
    }

    fn size(&self) -> usize {
        match &self.node {
            Node::Dir(contents) => contents.iter().map(|item| item.size()).sum(),
            Node::File(size) => *size,
        }
    }

    fn get_mut(&mut self, hierarchy: &Vec<String>) -> Option<&mut Self> {
        let mut current_dir = self;
        for dir in hierarchy {
            match &mut current_dir.node {
                Node::Dir(dirs) => current_dir = dirs.iter_mut().find(|item| item.name == *dir)?,
                Node::File(_) => return None,
            }
        }
        Some(current_dir)
    }

    /// Size and size forced up
    fn sum_part1(&self) -> (usize, usize) {
        match &self.node {
            Node::File(size) => (*size, 0),
            Node::Dir(dir) => {
                let mut forced = 0;
                let mut current = 0;
                for item in dir {
                    let (size, force) = item.sum_part1();
                    current += size;
                    forced += force;
                }
                if current <= 100_000 {
                    forced += current
                }
                (current, forced)
            }
        }
    }

    fn dir_sizes(&self) -> Vec<usize> {
        match &self.node {
            Node::File(size) => vec![*size],
            Node::Dir(dir) => {
                let mut sizes = dir
                    .iter()
                    .filter(|item| item.is_dir())
                    .flat_map(|item| item.dir_sizes())
                    .collect::<Vec<_>>();
                sizes.push(self.size());
                sizes
            }
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self.node, Node::Dir(_))
    }
}

enum DataType {
    CdInit,
    CdBack,
    CdNode(String),
    Ls,
    Item(Item),
}

impl FromStr for DataType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        match words.next() {
            Some("$") => match words.next() {
                Some("cd") => match words.next() {
                    Some("/") => Ok(Self::CdInit),
                    Some("..") => Ok(Self::CdBack),
                    Some(name) if !name.is_empty() => Ok(Self::CdNode(name.to_string())),
                    _ => Err("'$ cd' without name")?,
                },
                Some("ls") => Ok(Self::Ls),
                Some(other) => Err(format!("Unknown command '{other}'")),
                None => Err("Command missing")?,
            },
            Some("dir") => {
                let name = words.next().ok_or("Directory name missing")?.to_string();
                Ok(Self::Item(Item {
                    name,
                    node: Node::Dir(Vec::new()),
                }))
            }
            Some(size) => {
                let size = size
                    .parse::<usize>()
                    .map_err(|e| format!("Failed to parse file size, found '{size}'. {e}"))?;
                let name = words.next().ok_or("File name missing")?.to_string();
                Ok(Self::Item(Item {
                    name,
                    node: Node::File(size),
                }))
            }
            _ => Err("Empty Line")?,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 24933642);
    }
}

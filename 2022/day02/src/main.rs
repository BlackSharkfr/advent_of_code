use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    println!("Part1 : {}", part1(input));
    println!("Part2 : {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            (words.next().unwrap(), words.next().unwrap())
        })
        .map(|(opponent, you)| {
            (
                opponent.parse::<Move>().unwrap(),
                you.parse::<Move>().unwrap(),
            )
        })
        .map(|(opponent, you)| {
            let value = you.value();
            let game = Game::play(opponent, you).value();
            value + game
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            (words.next().unwrap(), words.next().unwrap())
        })
        .map(|(opponent, you)| {
            (
                opponent.parse::<Move>().unwrap(),
                you.parse::<Game>().unwrap(),
            )
        })
        .map(|(opponent, you)| {
            let game = you.value();
            let you = you.choice(opponent).value();
            game + you
        })
        .sum()
}

#[derive(Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
#[derive(Clone, Copy)]
enum Game {
    Win,
    Loose,
    Draw,
}
impl Move {
    fn value(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}
impl FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}
impl Game {
    fn play(their_move: Move, your_move: Move) -> Self {
        match (their_move, your_move) {
            (Move::Paper, Move::Scissors)
            | (Move::Rock, Move::Paper)
            | (Move::Scissors, Move::Rock) => Game::Win,
            _ if their_move == your_move => Game::Draw,
            _ => Game::Loose,
        }
    }
    fn value(self) -> u32 {
        match self {
            Game::Win => 6,
            Game::Loose => 0,
            Game::Draw => 3,
        }
    }
    fn choice(self, opponent: Move) -> Move {
        match self {
            Game::Win => match opponent {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            Game::Loose => match opponent {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Game::Draw => opponent,
        }
    }
}
impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Game::Loose),
            "Y" => Ok(Game::Draw),
            "Z" => Ok(Game::Win),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 12);
    }
}

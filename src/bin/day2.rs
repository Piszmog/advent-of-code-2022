use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let start = std::time::Instant::now();

    let file = File::open("./resources/day2.txt").unwrap();
    let input = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut parts = line.split_whitespace();
            Round::new(parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    let part_1_result = part_1(&input);
    println!("Part 1: {}", part_1_result);

    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}

fn part_1(input: &Vec<Round>) -> u32 {
    let mut total = 0;
    for round in input {
        let result = round.result();
        total += result as u32;
        total += round.user.points();
    }
    total
}

fn part_2(input: &Vec<Round>) -> u32 {
    let mut total = 0;
    for round in input {
        let result = round.result();
        total += result as u32;
        total += round.user.points();
    }
    total
}

struct Round {
    opponent: Choice,
    user: Choice,
}

impl Round {
    fn new(opponent: &str, user: &str) -> Round {
        Round {
            opponent: Choice::to_enum(opponent),
            user: Choice::to_enum(user),
        }
    }

    fn result(&self) -> Result {
        match (&self.opponent, &self.user) {
            (Choice::Rock, Choice::Rock) => Result::Draw,
            (Choice::Rock, Choice::Paper) => Result::Win,
            (Choice::Rock, Choice::Scissors) => Result::Loss,
            (Choice::Paper, Choice::Rock) => Result::Loss,
            (Choice::Paper, Choice::Paper) => Result::Draw,
            (Choice::Paper, Choice::Scissors) => Result::Win,
            (Choice::Scissors, Choice::Rock) => Result::Win,
            (Choice::Scissors, Choice::Paper) => Result::Loss,
            (Choice::Scissors, Choice::Scissors) => Result::Draw,
        }
    }
}

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn opponent(&self) -> &str {
        match self {
            Choice::Rock => "A",
            Choice::Paper => "B",
            Choice::Scissors => "C",
        }
    }

    fn user(&self) -> &str {
        match self {
            Choice::Rock => "X",
            Choice::Paper => "Y",
            Choice::Scissors => "Z",
        }
    }

    fn to_enum(val: &str) -> Choice {
        match val {
            "A" => Choice::Rock,
            "X" => Choice::Rock,
            "B" => Choice::Paper,
            "Y" => Choice::Paper,
            "C" => Choice::Scissors,
            "Z" => Choice::Scissors,
            _ => panic!("Invalid choice"),
        }
    }

    fn points(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

enum Result {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "A Y
B X
C Z".lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                Round::new(parts.next().unwrap(), parts.next().unwrap())
            })
            .collect();
        assert_eq!(part_1(&input), 15);
    }

    #[test]
    fn part2() {
        let input = "A Y
B X
C Z".lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                Round::new(parts.next().unwrap(), parts.next().unwrap())
            })
            .collect();
        assert_eq!(part_2(&input), 12);
    }
}

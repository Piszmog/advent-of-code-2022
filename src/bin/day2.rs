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

    let part_2_result = part_2(&input);
    println!("Part 2: {}", part_2_result);

    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}

fn part_1(input: &Vec<Round>) -> u32 {
    let mut total = 0;
    for round in input {
        let result = round.result();
        total += result as u32 + round.user.points();
    }
    total
}

fn part_2(input: &Vec<Round>) -> u32 {
    let mut total = 0;
    for round in input {
        let result = round.user.to_result();
        let required_user_choice = round.required_user_choice();
        total += result as u32 + required_user_choice.points();
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
            opponent: Choice::from_str(opponent),
            user: Choice::from_str(user),
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

    fn required_user_choice(&self) -> Choice {
        match (&self.opponent, &self.user.to_result()) {
            (Choice::Rock, Result::Win) => Choice::Paper,
            (Choice::Rock, Result::Loss) => Choice::Scissors,
            (Choice::Rock, Result::Draw) => Choice::Rock,
            (Choice::Paper, Result::Win) => Choice::Scissors,
            (Choice::Paper, Result::Loss) => Choice::Rock,
            (Choice::Paper, Result::Draw) => Choice::Paper,
            (Choice::Scissors, Result::Win) => Choice::Rock,
            (Choice::Scissors, Result::Loss) => Choice::Paper,
            (Choice::Scissors, Result::Draw) => Choice::Scissors,
        }
    }
}

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_str(val: &str) -> Choice {
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

    fn to_result(&self) -> Result {
        match self {
            Choice::Rock => Result::Loss,
            Choice::Paper => Result::Draw,
            Choice::Scissors => Result::Win,
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
    fn test_part_1() {
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
    fn test_part_2() {
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

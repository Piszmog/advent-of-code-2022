use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::slice::SliceIndex;

const POINTS: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

fn main() {
    let start = std::time::Instant::now();

    let file = File::open("./resources/day3.txt").unwrap();
    let input = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|l| l.to_string())
        .collect();

    let part_1_result = part_1(&input);
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Part 2: {}", part_2_result);

    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}

fn part_1(input: &Vec<String>) -> u32 {
    let mut total = 0;
    for rucksack in input {
        let len = rucksack.len();
        let (left, right) = rucksack.split_at(len / 2);
        let mut map = HashMap::new();
        for c in left.chars() {
            map.insert(c, true);
        }
        for c in right.chars() {
            if *map.get(&c).unwrap_or(&false) {
                map.insert(c, false);
                total += POINTS.iter().position(|&r| r == c).unwrap() as u32 + 1;
            }
        }
    }
    total
}

fn part_2(input: &Vec<String>) -> u32 {
    let mut total = 0;
    let mut index = 0;
    while index < input.len() {
        let runsack_1 = input.get(index).unwrap();
        let runsack_2 = input.get(index + 1).unwrap();
        let runsack_3 = input.get(index + 2).unwrap();

        let mut map = HashMap::new();
        for c in runsack_1.chars() {
            map.insert(c, 1);
        }

        for c in runsack_2.chars() {
            if map.contains_key(&c) {
                map.insert(c, 2);
            }
        }

        for c in runsack_3.chars() {
            if map.contains_key(&c) && map.get(&c).unwrap() == &2 {
                total += POINTS.iter().position(|&r| r == c).unwrap() as u32 + 1;
                break;
            }
        }

        index += 3;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        assert_eq!(part_1(&input), 157);
    }

    #[test]
    fn test_part_2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        assert_eq!(part_2(&input), 70);
    }
}

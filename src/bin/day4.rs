use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let start = std::time::Instant::now();

    let file = File::open("./resources/day4.txt").unwrap();
    let input = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.split(",").map(|s| s.to_string()).collect())
        .map(|line: Vec<String>| {
            let mut l = line.first().unwrap().split('-');
            let mut r = line.last().unwrap().split('-');
            (
                (l.next().unwrap().parse::<u8>().unwrap(), l.next().unwrap().parse::<u8>().unwrap()),
                (r.next().unwrap().parse::<u8>().unwrap(), r.next().unwrap().parse::<u8>().unwrap()),
            )
        })
        .collect();

    let part_1_result = part_1(&input);
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Part 2: {}", part_2_result);

    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}

fn part_1(input: &Vec<((u8, u8), (u8, u8))>) -> u16 {
    let mut contained_pairs = 0;
    for pair in input {
        let ((a, b), (c, d)) = *pair;
        if a >= c && b <= d {
            contained_pairs += 1;
        } else if c >= a && d <= b {
            contained_pairs += 1;
        }
    }
    contained_pairs
}

fn part_2(input: &Vec<((u8, u8), (u8, u8))>) -> u16 {
    let mut overlapping_pairs = 0;
    for pair in input {
        let ((a, b), (c, d)) = *pair;
        if (a <= c && c <= b) || (a <= d && d <= b) {
            overlapping_pairs += 1;
        } else if (c <= a && a <= d) || (c <= b && b <= d) {
            overlapping_pairs += 1;
        }
    }
    overlapping_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .lines()
            .map(|l| l.split(','))
            .map(|mut l| (l.next().unwrap(), l.next().unwrap()))
            .map(|(l, r)| {
                let mut l = l.split('-');
                let mut r = r.split('-');
                (
                    (l.next().unwrap().parse::<u8>().unwrap(), l.next().unwrap().parse::<u8>().unwrap()),
                    (r.next().unwrap().parse::<u8>().unwrap(), r.next().unwrap().parse::<u8>().unwrap()),
                )
            })
            .collect();
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn test_part_2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .lines()
            .map(|l| l.split(','))
            .map(|mut l| (l.next().unwrap(), l.next().unwrap()))
            .map(|(l, r)| {
                let mut l = l.split('-');
                let mut r = r.split('-');
                (
                    (l.next().unwrap().parse::<u8>().unwrap(), l.next().unwrap().parse::<u8>().unwrap()),
                    (r.next().unwrap().parse::<u8>().unwrap(), r.next().unwrap().parse::<u8>().unwrap()),
                )
            })
            .collect();
        assert_eq!(part_2(&input), 4);
    }
}

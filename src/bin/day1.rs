use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let start = std::time::Instant::now();
    let file = File::open("./resources/day1.txt").unwrap();
    let input = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            if line.is_empty() {
                0
            } else {
                line.parse::<u32>().unwrap()
            }
        })
        .collect();

    let part_1_result = part_1(&input);
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Part 2: {}", part_2_result);

    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}

fn part_1(input: &Vec<u32>) -> u32 {
    let mut max_cals = 0;
    let mut current_elf = 0;
    for i in input {
        if i == &0 {
            if current_elf > max_cals {
                max_cals = current_elf;
            }
            current_elf = 0;
        }
        current_elf += i;
    }
    if current_elf > max_cals {
        max_cals = current_elf;
    }
    max_cals
}

fn part_2(input: &Vec<u32>) -> u32 {
    let mut current_elf = 0;
    let mut elf_1 = 0;
    let mut elf_2 = 0;
    let mut elf_3 = 0;
    for i in input {
        if i == &0 {
            if current_elf > elf_1 {
                (elf_1, current_elf) = (current_elf, elf_1);
            }
            if current_elf > elf_2 {
                (elf_2, current_elf) = (current_elf, elf_2);
            }
            if current_elf > elf_3 {
                elf_3 = current_elf;
            }
            current_elf = 0;
        }
        current_elf += i;
    }
    if current_elf > elf_1 {
        (elf_1, current_elf) = (current_elf, elf_1);
    }
    if current_elf > elf_2 {
        (elf_2, current_elf) = (current_elf, elf_2);
    }
    if current_elf > elf_3 {
        elf_3 = current_elf;
    }
    elf_1 + elf_2 + elf_3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".lines()
            .map(|line| {
                if line.is_empty() {
                    0
                } else {
                    line.parse::<u32>().unwrap()
                }
            })
            .collect();
        assert_eq!(part_1(&input), 24000);
    }

    #[test]
    fn test_part_2() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".lines()
            .map(|line| {
                if line.is_empty() {
                    0
                } else {
                    line.parse::<u32>().unwrap()
                }
            })
            .collect();
        assert_eq!(part_2(&input), 45000);
    }
}

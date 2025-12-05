use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut buf = [0u8; 4];

    input
        .trim_ascii()
        .lines()
        .map(|line| {
            line.trim_ascii()
                .chars()
                .map(|c| c.encode_utf8(&mut buf).parse().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u8>]) -> u64 {
    input.iter().map(|bank| largest_joltage(bank, 2)).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u8>]) -> u64 {
    input.iter().map(|bank| largest_joltage(bank, 12)).sum()
}

const fn compare_batt(batt1: u8, batt2: u8) -> Ordering {
    if batt2 > batt1 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn largest_joltage(bank: &[u8], count: usize) -> u64 {
    let mut joltage = 0;
    let mut batt_idx = 0;

    for offset in 1..=count {
        let up_to_idx = bank.len() - count + offset;

        batt_idx = bank[..up_to_idx]
            .iter()
            .enumerate()
            .skip(batt_idx)
            .max_by(|(_, batt1), (_, batt2)| compare_batt(**batt1, **batt2))
            .map(|x| x.0)
            .unwrap();

        joltage *= 10;
        joltage += u64::from(bank[batt_idx]);

        batt_idx += 1;
    }

    joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 357);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&parse(include_str!("../input/2025/day3.txt"))),
            17_452
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 3_121_910_778_619);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2025/day3.txt"))),
            173_300_819_005_913
        );
    }
}

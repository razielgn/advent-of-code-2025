use aoc_runner_derive::{aoc, aoc_generator};

const DIAL_START: i32 = 50;
const DIAL_WRAP: i32 = 100;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| match line.split_at(1) {
            ("L", n) => -n.parse::<i32>().unwrap(),
            ("R", n) => n.parse().unwrap(),
            _ => unimplemented!(),
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> usize {
    input
        .iter()
        .scan(DIAL_START, |dial, movement| {
            *dial = (*dial + *movement).rem_euclid(DIAL_WRAP);
            Some(*dial)
        })
        .filter(|dial| *dial == 0)
        .count()
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> usize {
    input
        .iter()
        .fold((0, DIAL_START), |(count, dial), movement| {
            let over_dial = dial + *movement;
            let mut next_count = count
                + usize::try_from((over_dial / DIAL_WRAP).unsigned_abs())
                    .unwrap();

            if dial != 0 && over_dial <= 0 {
                next_count += 1;
            }

            (next_count, over_dial.rem_euclid(DIAL_WRAP))
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn solution1() {
        assert_eq!(part1(&parse(include_str!("../input/2025/day1.txt"))), 964);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn solution2() {
        assert_eq!(part2(&parse(include_str!("../input/2025/day1.txt"))), 5872);
    }
}

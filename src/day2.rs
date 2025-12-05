use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::prelude::*;
use std::ops::RangeInclusive;

type Id = u64;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<RangeInclusive<Id>> {
    input
        .trim_ascii()
        .split(',')
        .map(|line| {
            let (from, to) = line.split_once('-').unwrap();
            from.parse().unwrap()..=to.parse().unwrap()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[RangeInclusive<Id>]) -> u64 {
    input
        .par_iter()
        .cloned()
        .flat_map(|range| range)
        .filter(|id| is_invalid_part1(*id))
        .sum::<u64>()
}

#[aoc(day2, part2)]
fn part2(input: &[RangeInclusive<Id>]) -> u64 {
    input
        .par_iter()
        .cloned()
        .flat_map(|range| range)
        .filter(|id| is_invalid_part2(*id))
        .sum::<u64>()
}

fn is_invalid_part1(id: Id) -> bool {
    RevDigitsIter::new(id, 2)
        .and_then(|mut iter| Some(iter.next()? == iter.next()?))
        .unwrap_or(false)
}

fn is_invalid_part2(id: Id) -> bool {
    [1u32, 2, 3, 5]
        .into_iter()
        .filter_map(|factor| {
            let digits = count_digits(id);

            if digits == 0 {
                return None;
            }

            let chunks = digits / factor;

            if chunks == 1 {
                return None;
            }

            Some(digits / factor)
        })
        .filter_map(|chunks| RevDigitsIter::new(id, chunks))
        .any(|iter| iter.tuple_windows().all(|(a, b)| a == b))
}

struct RevDigitsIter {
    n: u64,
    digits: u32,
    chunks: u32,
    curr: u32,
}

impl RevDigitsIter {
    const fn new(n: u64, chunks: u32) -> Option<Self> {
        let digits = count_digits(n);

        if !digits.is_multiple_of(chunks) {
            return None;
        }

        Some(Self {
            n,
            digits,
            chunks,
            curr: 0,
        })
    }
}

impl Iterator for RevDigitsIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            if self.curr == 0 {
                self.curr += 1;

                return Some(0);
            }

            return None;
        }

        let mut chunk = 0;

        for _ in 0..self.digits / self.chunks {
            chunk *= 10;
            chunk += self.n % 10;

            self.n /= 10;
        }

        self.curr += 1;

        Some(chunk)
    }
}

const fn count_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;

    loop {
        n /= 10;
        count += 1;

        if n == 0 {
            break count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1_227_775_554);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&parse(include_str!("../input/2025/day2.txt"))),
            18_952_700_150
        );
    }

    #[test]
    fn rev_digits_iterator() {
        macro_rules! run {
            ($n:expr, $chunks:expr) => {
                RevDigitsIter::new($n, $chunks).unwrap().collect::<Vec<_>>()
            };
        }

        assert_eq!(vec![0], run!(0, 1));
        assert_eq!(vec![21], run!(12, 1));
        assert_eq!(vec![321], run!(123, 1));
        assert_eq!(vec![2, 1], run!(12, 2));
        assert_eq!(vec![43, 21], run!(1_234, 2));
        assert_eq!(vec![654, 321], run!(123_456, 2));
    }

    #[test]
    fn is_invalid_part2_() {
        assert!(!is_invalid_part2(1));
        assert!(!is_invalid_part2(11_12));
        assert!(is_invalid_part2(10_10));
        assert!(is_invalid_part2(1_1));
        assert!(is_invalid_part2(1_1_1_1_1_1_1));
        assert!(is_invalid_part2(1_188_511_885));
        assert!(is_invalid_part2(12_12_12_12_12));
        assert!(is_invalid_part2(123_123_123));
        assert!(is_invalid_part2(1234_1234));
        assert!(is_invalid_part2(21_21_21_21_21));
        assert!(is_invalid_part2(3859_3859));
        assert!(is_invalid_part2(446_446));
        assert!(is_invalid_part2(565_656));
        assert!(!is_invalid_part2(6_758_902_779));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 4_174_379_265);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2025/day2.txt"))),
            28_858_486_244
        );
    }
}

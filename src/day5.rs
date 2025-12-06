use crate::rangeset::RangeSet;
use aoc_runner_derive::{aoc, aoc_generator};

type Id = u64;

#[aoc_generator(day5)]
fn parse(input: &str) -> (RangeSet<Id>, Vec<Id>) {
    let input = input.trim_ascii();
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            let (from, to) = line.split_once('-').unwrap();
            from.parse::<Id>().unwrap()..to.parse::<Id>().unwrap() + 1
        })
        .fold(RangeSet::default(), |mut acc, range| {
            acc.insert_range(&range);
            acc
        });

    let ingredients = ingredients
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ingredients)
}

#[aoc(day5, part1)]
fn part1((ranges, ingredients): &(RangeSet<Id>, Vec<Id>)) -> usize {
    ingredients
        .iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

#[aoc(day5, part2)]
fn part2((ranges, _ingredients): &(RangeSet<Id>, Vec<Id>)) -> u64 {
    ranges.iter().map(|r| r.end - r.start).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn solution1() {
        assert_eq!(part1(&parse(include_str!("../input/2025/day5.txt"))), 615);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 14);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2025/day5.txt"))),
            353_716_783_056_994
        );
    }
}

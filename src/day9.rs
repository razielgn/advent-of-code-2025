use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Rect<'a> {
    from: &'a Point,
    to: &'a Point,
}

impl<'a> Rect<'a> {
    const fn new(from: &'a Point, to: &'a Point) -> Self {
        Self { from, to }
    }

    const fn area(&self) -> u64 {
        (self.from.x.abs_diff(self.to.x) + 1)
            * (self.from.y.abs_diff(self.to.y) + 1)
    }

    fn is_inside(&self, polygon: &[Point]) -> bool {
        let Rect { from, to } = self;

        polygon.iter().circular_tuple_windows().all(
            |(segment_start, segment_end)| {
                let left = from.x.max(to.x) <= segment_start.x.min(segment_end.x);
                let right =
                    from.x.min(to.x) >= segment_start.x.max(segment_end.x);
                let above =
                    from.y.max(to.y) <= segment_start.y.min(segment_end.y);
                let below =
                    from.y.min(to.y) >= segment_start.y.max(segment_end.y);

                left || right || above || below
            },
        )
    }
}

type Polygon = Vec<Point>;

#[aoc_generator(day9)]
fn parse(input: &str) -> Polygon {
    input
        .trim_ascii()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();

            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Point]) -> u64 {
    input
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Rect::new(a, b).area())
        .max()
        .unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &[Point]) -> u64 {
    let rects = {
        let mut rects = input
            .iter()
            .tuple_combinations()
            .map(|(a, b)| Rect::new(a, b))
            .collect_vec();
        rects.par_sort_unstable_by_key(|rect| u64::MAX - rect.area());
        rects
    };

    rects
        .into_iter()
        .find(|rect| rect.is_inside(input))
        .map(|rect| rect.area())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 50);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&parse(include_str!("../input/2025/day9.txt"))),
            4_743_645_488
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 24);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2025/day9.txt"))),
            1_529_011_204,
        );
    }
}

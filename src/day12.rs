use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use ndarray::prelude::*;

const SHAPE_COUNT: usize = 6;

struct Shape(Array2<u8>);

impl Shape {
    fn area(&self) -> usize {
        self.0.iter().filter(|c| **c > 0).count()
    }
}

type Shapes = [Shape; SHAPE_COUNT];

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    quantities: [usize; SHAPE_COUNT],
}

impl Region {
    const fn area(&self) -> usize {
        self.width * self.height
    }

    fn can_fit(&self, shapes: &Shapes) -> bool {
        self.quantities
            .iter()
            .zip(shapes.iter())
            .map(|(quantity, shape)| quantity * shape.area())
            .sum::<usize>()
            <= self.area()
    }
}

type Regions = Box<[Region]>;

#[aoc_generator(day12)]
fn parse(input: &str) -> (Shapes, Regions) {
    let shapes = input
        .trim_ascii()
        .lines()
        .take(5 * SHAPE_COUNT)
        .chunks(5)
        .into_iter()
        .map(|group| {
            Shape(
                Array2::from_shape_vec(
                    (3, 3),
                    group
                        .skip(1)
                        .take(3)
                        .flat_map(|line| line.chars().map(|c| u8::from(c == '#')))
                        .collect_vec(),
                )
                .unwrap(),
            )
        })
        .collect_array()
        .unwrap();

    let regions = input
        .trim_ascii()
        .lines()
        .skip(5 * SHAPE_COUNT)
        .map(|line| {
            let (dim, quantities) = line.split_once(": ").unwrap();

            let (width, height) = dim
                .split('x')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();

            let quantities = quantities
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect_array()
                .unwrap();

            Region {
                width,
                height,
                quantities,
            }
        })
        .collect_vec()
        .into_boxed_slice();

    (shapes, regions)
}

#[aoc(day12, part1)]
fn part1((shapes, regions): &(Shapes, Regions)) -> usize {
    regions
        .iter()
        .filter(|region| region.can_fit(shapes))
        .count()
}

#[aoc(day12, part2)]
fn part2(_input: &(Shapes, Regions)) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn solution1() {
        assert_eq!(part1(&parse(include_str!("../input/2025/day12.txt"))), 406);
    }
}

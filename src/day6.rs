use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use ndarray::prelude::*;

#[derive(Debug)]
enum Op {
    Sum,
    Mul,
}

type Grid = Array2<u64>;
type Ops = Vec<Op>;

#[aoc_generator(day6)]
fn parse(input: &str) -> String {
    // NOTE: cargo-aoc does not allow returning refs from `parse` :(
    input.into()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u64 {
    solve(parse_grid_horizontally(input))
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    solve(parse_grid_vertically(input))
}

fn parse_grid_horizontally(input: &str) -> (Grid, Ops) {
    let rows_count = input.lines().count() - 1;
    let cols_count = input.lines().next().unwrap().split_whitespace().count();

    let grid = Grid::from_shape_vec(
        (rows_count, cols_count),
        input
            .trim_ascii()
            .split_whitespace()
            .take(rows_count * cols_count)
            .map(|n| n.parse().unwrap())
            .collect_vec(),
    )
    .unwrap();

    (grid, parse_ops(input))
}

fn transpose<T>(mut iters: Vec<std::vec::IntoIter<T>>) -> impl Iterator<Item = Vec<T>> {
    std::iter::from_fn(move || iters.iter_mut().map(Iterator::next).collect())
}

fn parse_grid_vertically(input: &str) -> (Grid, Ops) {
    let rows_count = input.lines().count() - 1;
    let ops = {
        let mut ops = parse_ops(input);
        ops.reverse();
        ops
    };

    let inverted_rows = input
        .lines()
        .take(rows_count)
        .map(|line| line.chars().rev().collect_vec().into_iter())
        .collect_vec();

    let mut grid = Grid::zeros((4, 0));
    let mut curr_col = Vec::with_capacity(4);

    for col in transpose(inverted_rows) {
        if col.iter().all(char::is_ascii_whitespace) {
            push_col(&mut curr_col, &mut grid);

            continue;
        }

        let n = col
            .iter()
            .filter(|c| !c.is_ascii_whitespace())
            .fold(0u64, |mut acc, c| {
                acc *= 10;
                acc += u64::from(c.to_digit(10).unwrap());
                acc
            });

        curr_col.push(n);
    }

    push_col(&mut curr_col, &mut grid);

    (grid, ops)
}

fn push_col(col: &mut Vec<u64>, grid: &mut Grid) {
    for _ in 0..(grid.nrows() - col.len()) {
        col.push(0);
    }

    grid.push_column(Array1::from_iter(col.drain(..)).view())
        .unwrap();
}

fn parse_ops(input: &str) -> Ops {
    input
        .trim_ascii()
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| match s {
            "+" => Op::Sum,
            "*" => Op::Mul,
            _ => unreachable!(),
        })
        .collect_vec()
}

fn solve((grid, ops): (Grid, Ops)) -> u64 {
    // eprintln!("{grid:?}");

    grid.t()
        .rows()
        .into_iter()
        .zip(ops)
        .map(|(row, op)| match op {
            Op::Sum => row.sum(),
            Op::Mul => row.iter().filter(|n| **n != 0).product(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 4_277_556);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&parse(include_str!("../input/2025/day6.txt"))),
            8_108_520_669_952
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 3_263_827);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2025/day6.txt"))),
            11_708_563_470_209
        );
    }
}

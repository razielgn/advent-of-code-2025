use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::prelude::*;

type Grid = Array2<u8>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Grid {
    let input = input.trim_ascii();
    let shape = (
        input.lines().count(),
        input.lines().next().unwrap().trim_ascii().len(),
    );

    let vec = input
        .trim_ascii()
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| u8::from(c != '.'))
        .collect();

    pad(&Grid::from_shape_vec(shape, vec).unwrap())
}

#[aoc(day4, part1)]
fn part1(input: &Grid) -> usize {
    input
        .windows((3, 3))
        .into_iter()
        .filter(|w| w[(1, 1)] == 1)
        // PERF: benchmarks unintuitively show that this is faster than `w.sum() < 5`
        .filter(|w| w.into_iter().filter(|cell| **cell == 1).count() < 5)
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut rolls = 0;

    loop {
        let next = next_grid(&grid);

        let diff = (&next ^ grid).into_iter().filter(|cell| *cell == 1).count();

        if diff == 0 {
            break rolls;
        }

        rolls += diff;
        grid = next;
    }
}

fn pad(grid: &Grid) -> Grid {
    let (rows, cols) = (grid.nrows(), grid.ncols());

    let mut padded = Grid::zeros((rows + 2, cols + 2));

    #[allow(clippy::reversed_empty_ranges)]
    padded.slice_mut(s![1..-1, 1..-1]).assign(grid);

    padded
}

fn next_grid(grid: &Grid) -> Grid {
    pad(&Grid::from_shape_vec(
        (grid.nrows() - 2, grid.ncols() - 2),
        grid.windows((3, 3))
            .into_iter()
            .map(|w| {
                let center = w[(1, 1)];

                if center == 0 {
                    return 0;
                }

                // PERF: benchmarks unintuitively show that this is faster than `w.sum() < 5`
                u8::from(w.into_iter().filter(|cell| **cell == 1).count() >= 5)
            })
            .collect(),
    )
    .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 13);
    }

    #[test]
    fn solution1() {
        assert_eq!(part1(&parse(include_str!("../input/2025/day4.txt"))), 1_560);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 43);
    }

    #[test]
    fn solution2() {
        assert_eq!(part2(&parse(include_str!("../input/2025/day4.txt"))), 9_609);
    }
}

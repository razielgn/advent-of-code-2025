use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::{FxHashMap, FxHashSet};

type Collisions = usize;
type Timelines = usize;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    const fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    const fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    const fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Debug)]
struct StepState<'a> {
    diagram: &'a Diagram,
    beams: FxHashMap<Pos, Timelines>,
    collisions: Collisions,
}

impl<'a> StepState<'a> {
    fn new(diagram: &'a Diagram) -> Self {
        Self {
            diagram,
            beams: FxHashMap::from_iter([(diagram.init, 1)]),
            collisions: 0,
        }
    }
}

impl Iterator for StepState<'_> {
    type Item = (Collisions, Timelines);

    fn next(&mut self) -> Option<Self::Item> {
        let prev_collisions = self.collisions;

        while prev_collisions == self.collisions {
            let mut next_beams = FxHashMap::default();
            next_beams.reserve(self.beams.len() * 2);

            for (prev_pos, prev_count) in self.beams.drain() {
                let pos = prev_pos.down();

                if pos.y == self.diagram.height {
                    return None;
                }

                if self.diagram.splitters.contains(&pos) {
                    next_beams
                        .entry(pos.left())
                        .and_modify(|count| *count += prev_count)
                        .or_insert(prev_count);
                    next_beams
                        .entry(pos.right())
                        .and_modify(|count| *count += prev_count)
                        .or_insert(prev_count);

                    self.collisions += 1;
                } else {
                    next_beams
                        .entry(pos)
                        .and_modify(|count| *count += prev_count)
                        .or_insert(prev_count);
                }
            }

            self.beams = next_beams;
        }

        Some((self.collisions, self.beams.values().sum()))
    }
}

#[derive(Debug)]
struct Diagram {
    init: Pos,
    splitters: FxHashSet<Pos>,
    height: usize,
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Diagram {
    let input = input.trim_ascii();

    let splitters = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim_ascii()
                .chars()
                .enumerate()
                .filter(|(_x, c)| *c == '^')
                .map(move |(x, _)| Pos { x, y })
        })
        .collect();

    let init = input
        .lines()
        .next()
        .unwrap()
        .trim_ascii()
        .chars()
        .enumerate()
        .filter(|(_x, c)| *c == 'S')
        .map(|(x, _c)| Pos { x, y: 0 })
        .next()
        .unwrap();

    let height = input.lines().count();

    Diagram {
        init,
        splitters,
        height,
    }
}

#[aoc(day7, part1)]
fn part1(input: &Diagram) -> usize {
    StepState::new(input).last().unwrap().0
}

#[aoc(day7, part2)]
fn part2(input: &Diagram) -> usize {
    StepState::new(input).last().unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 21);
    }

    #[test]
    fn solution1() {
        assert_eq!(part1(&parse(include_str!("../input/2025/day7.txt"))), 1_553);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 40);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2025/day7.txt"))),
            15_811_946_526_915
        );
    }
}

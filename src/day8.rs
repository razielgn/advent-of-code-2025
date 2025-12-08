use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32,
}

impl JunctionBox {
    fn distance(&self, other: &Self) -> OrderedFloat<f64> {
        let dx = f64::from(self.x.abs_diff(other.x));
        let dy = f64::from(self.y.abs_diff(other.y));
        let dz = f64::from(self.z.abs_diff(other.z));

        OrderedFloat((dz.mul_add(dz, dx.mul_add(dx, dy * dy))).sqrt())
    }
}

type Circuit<'a> = FxHashSet<&'a JunctionBox>;

#[derive(Debug, Default, derive_more::Deref, derive_more::DerefMut)]
struct Decoration<'a> {
    #[deref]
    #[deref_mut]
    circuits: Vec<Circuit<'a>>,

    last_two: Option<(&'a JunctionBox, &'a JunctionBox)>,
}

impl<'a> Decoration<'a> {
    fn new(jbs: &'a [JunctionBox], connections: usize) -> Self {
        let mut decoration = Decoration::default();

        let mut combinations = jbs
            .iter()
            .tuple_combinations()
            .map(|(jb1, jb2)| (jb1, jb2, jb1.distance(jb2)))
            .collect_vec();

        combinations.par_sort_unstable_by_key(|(_jb1, _jb2, distance)| *distance);

        for (jb, nearest_jb, _distance) in
            combinations.into_iter().take(connections)
        {
            decoration.attach(jb, nearest_jb);
        }

        decoration.sort();
        decoration
    }

    fn part1_answer(&self) -> usize {
        self.iter().map(FxHashSet::len).take(3).product()
    }

    fn attach(&mut self, from: &'a JunctionBox, to: &'a JunctionBox) {
        let positions = self
            .iter()
            .positions(|circuit| circuit.contains(&to) || circuit.contains(&from))
            .collect_vec();

        match positions.as_slice() {
            // Junction boxes are new, we make a new circuit.
            [] => {
                self.push(FxHashSet::from_iter([from, to]));

                self.record_last_two(from, to);
            }
            // One of the two junction boxes is already in a circuit, so append to it.
            [idx] => {
                let circuit = &mut self[*idx];

                match (circuit.contains(&from), circuit.contains(&to)) {
                    (true, true) => {}
                    (false, true) => {
                        circuit.insert(from);

                        self.record_last_two(from, to);
                    }
                    (true, false) => {
                        circuit.insert(to);

                        self.record_last_two(from, to);
                    }
                    (false, false) => unreachable!(),
                }
            }
            // Merge two circuits
            [idx1, idx2] => {
                let mut circuit1 = self.remove(*idx1.max(idx2));
                let mut circuit2 = self.remove(*idx1.min(idx2));

                circuit1.extend(circuit2.drain());

                self.push(circuit1);

                self.record_last_two(from, to);
            }
            _ => unimplemented!(),
        }
    }

    fn sort(&mut self) {
        self.sort_unstable_by_key(|c| usize::MAX - c.len()); // reverse order
    }

    fn last_two_x_coords_mul(&self) -> u32 {
        self.last_two.map(|(a, b)| a.x * b.x).unwrap()
    }

    const fn record_last_two(
        &mut self,
        from: &'a JunctionBox,
        to: &'a JunctionBox,
    ) {
        self.last_two = Some((from, to));
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<JunctionBox> {
    input
        .trim_ascii()
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();

            JunctionBox { x, y, z }
        })
        .collect()
}

#[cfg(test)]
fn example1(input: &[JunctionBox]) -> usize {
    Decoration::new(input, 10).part1_answer()
}

#[aoc(day8, part1)]
fn part1(input: &[JunctionBox]) -> usize {
    Decoration::new(input, 1000).part1_answer()
}

#[aoc(day8, part2)]
fn part2(input: &[JunctionBox]) -> u32 {
    Decoration::new(input, usize::MAX).last_two_x_coords_mul()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn part1_example() {
        assert_eq!(example1(&parse(EXAMPLE)), 40);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&parse(include_str!("../input/2025/day8.txt"))),
            105_952
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 25_272);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2025/day8.txt"))),
            975_931_446
        );
    }
}

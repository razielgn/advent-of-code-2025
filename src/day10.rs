use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::prelude::*;
use std::iter;

type Lights = u16;

#[derive(Debug)]
struct Machine {
    indicator_light_diagram: Lights,
    button_wiring_schematics: Vec<Lights>,
    joltage_reqs: Vec<u16>,
}

impl Machine {
    fn fewest_button_presses(&self) -> usize {
        (1..=self.button_wiring_schematics.len())
            .filter_map(|k| {
                for combinations in
                    self.button_wiring_schematics.iter().combinations(k)
                {
                    let mut lights = 0;

                    for (counter, combination) in combinations
                        .iter()
                        .enumerate()
                        .map(|(idx, combination)| (idx + 1, combination))
                    {
                        lights ^= *combination;

                        if lights == self.indicator_light_diagram {
                            return Some(counter);
                        }
                    }
                }

                None
            })
            .min()
            .unwrap()
    }

    fn fewest_button_presses_joltage(&self) -> u64 {
        use z3::{Optimize, SatResult, ast::Int};

        let opt = Optimize::new();

        let vars = (0..self.button_wiring_schematics.len())
            .map(|idx| Int::fresh_const(&format!("n{idx}")))
            .collect_vec();

        for var in &vars {
            opt.assert(&var.ge(0));
        }

        for (joltage_idx, joltage) in self.joltage_reqs.iter().enumerate() {
            let mut equation = Int::from(0);

            for (button_idx, button) in
                self.button_wiring_schematics.iter().enumerate()
            {
                if iter_set_bits(*button).contains(&joltage_idx) {
                    equation += vars.get(button_idx).unwrap();
                }
            }

            opt.assert(&equation.eq(u32::from(*joltage)));
        }

        let sum_of_vars = vars.iter().sum::<Int>();

        opt.minimize(&sum_of_vars);

        assert_eq!(SatResult::Sat, opt.check(&[]));

        opt.get_model()
            .unwrap()
            .eval(&sum_of_vars, true)
            .unwrap()
            .as_u64()
            .unwrap()
    }
}

fn iter_set_bits(n: u16) -> impl Iterator<Item = usize> {
    iter::successors(
        Some(n),
        |&state| {
            if state == 0 { None } else { Some(state >> 1) }
        },
    )
    .enumerate()
    .filter(|(_idx, b)| (b & 1) == 1)
    .map(|(idx, _)| idx)
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Machine> {
    input
        .trim_ascii()
        .lines()
        .map(|line| {
            let (indicator_light_diagram, rest) = line.split_once("] ").unwrap();

            let indicator_light_diagram = indicator_light_diagram
                .trim_start_matches('[')
                .chars()
                .enumerate()
                .filter(|(_idx, c)| *c == '#')
                .fold(0u16, |acc, (idx, _c)| acc | 0b1 << idx);

            let (button_wiring_schematics, joltage_reqs) =
                rest.split_once(" {").unwrap();

            let button_wiring_schematics = button_wiring_schematics
                .split(' ')
                .map(|s| {
                    s.trim_start_matches('(')
                        .trim_end_matches(')')
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .fold(0, |acc, pos| acc | 0b1 << pos)
                })
                .collect();

            let joltage_reqs = joltage_reqs
                .trim_end_matches('}')
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            Machine {
                indicator_light_diagram,
                button_wiring_schematics,
                joltage_reqs,
            }
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> usize {
    input.par_iter().map(Machine::fewest_button_presses).sum()
}

#[aoc(day10, part2)]
fn part2(input: &[Machine]) -> u64 {
    input
        .par_iter()
        .map(Machine::fewest_button_presses_joltage)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 7);
    }

    #[test]
    fn solution1() {
        assert_eq!(part1(&parse(include_str!("../input/2025/day10.txt"))), 538);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 33);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2025/day10.txt"))),
            20_298
        );
    }

    #[test]
    fn iter_set_bits_() {
        assert_eq!(Vec::<usize>::new(), iter_set_bits(0).collect_vec());
        assert_eq!(vec![3], iter_set_bits(8).collect_vec());
        assert_eq!(vec![0, 1, 2, 4, 5], iter_set_bits(55).collect_vec());
    }
}

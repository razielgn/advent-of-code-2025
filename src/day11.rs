use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::directed::count_paths::count_paths;
use rustc_hash::FxHashMap;

type Inventory = FxHashMap<String, Vec<String>>;

#[aoc_generator(day11)]
fn parse(input: &str) -> Inventory {
    let mut inventory: FxHashMap<_, _> = input
        .trim_ascii()
        .lines()
        .map(|line| {
            let (src, dests) = line.split_once(": ").unwrap();

            (src.to_owned(), dests.split(' ').map(Into::into).collect())
        })
        .collect();

    inventory.insert("out".to_owned(), vec![]);
    inventory
}

#[aoc(day11, part1)]
fn part1(input: &Inventory) -> usize {
    count_paths(
        "you",
        |&id| input[id].iter().map(String::as_str),
        |&id| id == "out",
    )
}

#[aoc(day11, part2)]
fn part2(input: &Inventory) -> usize {
    count_paths(
        ("svr", false, false),
        |&(id, fft_visited, dac_visited)| {
            input[id].iter().map(String::as_str).map(move |s| {
                (s, fft_visited || s == "fft", dac_visited || s == "dac")
            })
        },
        |&node| node == ("out", true, true),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 5);
    }

    #[test]
    fn solution1() {
        assert_eq!(part1(&parse(include_str!("../input/2025/day11.txt"))), 749);
    }

    const EXAMPLE_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_2)), 2);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2025/day11.txt"))),
            420_257_875_695_750
        );
    }
}

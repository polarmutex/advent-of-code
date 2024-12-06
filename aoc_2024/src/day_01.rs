use aoc_lib::hash::*;
use aoc_lib::iter::ChunkOps;
use common::{solution, Answer};
use itertools::Itertools;

solution!("Historian Hysteria", 1);

type Input = (Vec<u32>, Vec<u32>);

fn parse(input: &str) -> Input {
    let (mut left, mut right) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<u32>().unwrap());
        right.push(parts.next().unwrap().parse::<u32>().unwrap());
    }
    (left, right)
}

fn part_1(input: &str) -> Answer {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    println!("finished 1 1");
    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u32>()
        .into()
}

fn part_2(input: &str) -> Answer {
    let (left, mut right) = parse(input);

    let mut map = FastMap::with_capacity(1_000);
    right.iter().for_each(|v| *map.entry(v).or_insert(0) += 1);

    left.iter()
        .map(|left| map.get(left).unwrap_or(&0) * left)
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(CASE), 11.into());
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(CASE), 31.into());
    }
}

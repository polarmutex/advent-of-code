use aoc_runner_macros::{aoc, generator, solver, solution};
use std::collections::HashMap;

type Input = (Vec<u32>, Vec<u32>);

#[aoc(2024, day1)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (mut left, mut right) = (Vec::new(), Vec::new());

        for line in input.lines() {
            let mut parts = line.split_whitespace();
            left.push(parts.next().unwrap().parse::<u32>().unwrap());
            right.push(parts.next().unwrap().parse::<u32>().unwrap());
        }
        (left, right)
    }

    #[solver(part1, main)]
    pub fn solve_part_1(input: Input) -> u32 {
        let (mut left, mut right) = input;

        left.sort();
        right.sort();

        left.into_iter()
            .zip(right)
            .map(|(left, right)| left.abs_diff(right))
            .sum::<u32>()
    }

    #[solver(part2, main)]
    pub fn solve_part_2(input: Input) -> u32 {
        let (left, right) = input;

        let mut map = HashMap::with_capacity(1_000);
        right.iter().for_each(|v| *map.entry(v).or_insert(0) += 1);

        left.iter()
            .map(|left| map.get(left).unwrap_or(&0) * left)
            .sum::<u32>()
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> u32 {
        let parsed = input_generator(input);
        solve_part_1(parsed)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> u32 {
        let parsed = input_generator(input);
        solve_part_2(parsed)
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner_macros::aoc_case;

    #[aoc_case(11, 31)]
    const CASE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
}

use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::Parser;

type Input = Vec<Vec<i32>>;

#[aoc(2023, day9)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        separated_list1(line_ending::<&str, nom::error::Error<&str>>, many1(terminated(complete::i32, space0)))
            .parse(input)
            .unwrap()
            .1
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> i32 {
        data.into_iter()
            .map(|d| {
                let mut nums = d;
                let mut sum = 0;
                while !nums.iter().all(|n| n == &0) {
                    if let Some(last) = nums.last() {
                        sum += last;
                    }
                    for i in 0..nums.len() - 1 {
                        nums[i] = nums[i + 1] - nums[i];
                    }
                    nums.pop();
                }
                sum
            })
            .sum::<i32>()
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> i32 {
        data.into_iter()
            .map(|d| {
                let mut nums = d.into_iter().rev().collect_vec();
                let mut values = vec![];
                while !nums.iter().all(|n| n == &0) {
                    if let Some(last) = nums.last() {
                        values.push(*last);
                    }
                    for i in 0..nums.len() - 1 {
                        nums[i] -= nums[i + 1];
                    }
                    nums.pop();
                }
                values.into_iter().rev().fold(0, |acc, num| num - acc)
            })
            .sum::<i32>()
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> i32 {
        let data = input_generator(input);
        solve_part_1(data)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> i32 {
        let data = input_generator(input);
        solve_part_2(data)
    }
}

#[cfg(test)]
mod test {
    use aoc_runner_macros::aoc_case;
    use indoc::indoc;

    #[aoc_case(114, 2)]
    const EXAMPLE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[cfg(feature = "real-input")]
    #[aoc_case(1887980197, 990)]
    const REAL: &str = include_str!("../input/2023/day9.txt");
}
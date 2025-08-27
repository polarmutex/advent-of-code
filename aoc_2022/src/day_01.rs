use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use std::cmp::Reverse;

type Input = Vec<u32>;

#[aoc(2022, day1)]
pub mod solutions {
    use super::*;

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        let groups: Vec<u32> = data
            .trim()
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(|line| line.parse::<u32>().unwrap())
                    .sum()
            })
            .collect();
        Ok(("", groups))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u32 {
        input.iter().max().copied().expect("At least one elf")
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u32 {
        input.iter()
            .sorted_unstable_by_key(|&&cals| Reverse(cals))
            .take(3)
            .sum::<u32>()
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 24000);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 45000);
    }

}

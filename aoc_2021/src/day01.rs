use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;


type Input = Vec<u32>;

#[aoc(2021, day1)]
pub mod solutions {
    use super::*;

    fn parse(input: &str) -> nom::IResult<&str, Input> {
        let nums: Vec<u32> = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect();
        Ok(("", nums))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u32 {
        input
            .iter()
            .tuple_windows()
            .filter(|&(&a, &b)| b > a)
            .count() as u32
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u32 {
        input
            .iter()
            .tuple_windows()
            .map(|(&a, &b, &c)| a + b + c)
            .tuple_windows()
            .filter(|&(a, b)| b > a)
            .count() as u32
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
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 7);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 5);
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        Ok(())
    }
}

use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::IResult;

type Input = Vec<u32>;

#[aoc(2021, day19)]
pub mod solutions {
    use super::*;

    fn parse(_input: &str) -> nom::IResult<&str, Input> {
        let i = vec![];
        Ok(("", i))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(_input: &Input) -> u64 {
        todo!()
    }

    #[solver(part2, gen)]
    pub fn solve_part2(_input: &Input) -> u64 {
        todo!()
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    #[ignore]
    fn test_part_1() {
        assert_eq!(solutions::part_1(EXAMPLE), 79);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(solutions::part_2(EXAMPLE), 3621);
    }
}

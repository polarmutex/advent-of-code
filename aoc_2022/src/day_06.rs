use ahash::AHashSet;
use aoc_runner_macros::{aoc, generator, solver, solution};

type Input = Vec<char>;

fn solve(input: &[char], window: usize) -> usize {
    input
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(window)
        .find(|w| w.iter().map(|p| p.1).collect::<AHashSet<_>>().len() == window)
        .unwrap()[0]
        .0
        + window
}

#[aoc(2022, day6)]
pub mod solutions {
    use super::*;

    fn parse(input: &str) -> nom::IResult<&str, Input> {
        let chars = input.trim().chars().collect();
        Ok(("", chars))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> usize {
        solve(input, 4)
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> usize {
        solve(input, 14)
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> usize {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> usize {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 11);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 26);
    }
}

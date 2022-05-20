use advent_of_code_traits::{days::Day1, ParseInput, Solution};

use crate::AdventOfCode2021;

impl ParseInput<Day1> for AdventOfCode2021 {
    type Parsed = Vec<u32>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.lines().filter_map(|l| l.parse().ok()).collect()
    }
}

impl Solution<Day1> for AdventOfCode2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<u32>) -> Self::Part1Output {
        input.windows(2).filter(|w| w[0] < w[1]).count()
    }

    fn part2(input: &Vec<u32>) -> Self::Part2Output {
        input
            .windows(3)
            .map(|w| w.into_iter().sum())
            .collect::<Vec<u32>>()
            .as_slice()
            .windows(2)
            .filter(|w| w[0] < w[1])
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const EXAMPLE: &str = "
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
        ";

    #[test]
    fn test_part1() {
        let input = <AdventOfCode2021 as ParseInput<Day1>>::parse_input(EXAMPLE);
        assert_eq!(<AdventOfCode2021 as Solution<Day1>>::part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let input = <AdventOfCode2021 as ParseInput<Day1>>::parse_input(EXAMPLE);
        assert_eq!(<AdventOfCode2021 as Solution<Day1>>::part2(&input), 5);
    }

    #[test]
    fn test_answers() {
        let input_file = "data/2021/day01_github.txt";
        let input_str =
            read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
        let input = <AdventOfCode2021 as ParseInput<Day1>>::parse_input(&input_str);
        assert_eq!(<AdventOfCode2021 as Solution<Day1>>::part1(&input), 1448);
        assert_eq!(<AdventOfCode2021 as Solution<Day1>>::part2(&input), 1471);
    }
}

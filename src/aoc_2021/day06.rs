use advent_of_code_traits::{days::Day6, ParseInput, Solution};

use crate::AdventOfCode2021;

impl ParseInput<Day6> for AdventOfCode2021 {
    type Parsed = Vec<u32>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.split(',').filter_map(|n| n.parse().ok()).collect()
    }
}

impl Solution<Day6> for AdventOfCode2021 {
    type Part1Output = usize;
    type Part2Output = u64;

    fn part1(input: &Vec<u32>) -> Self::Part1Output {
        let mut state: Vec<u32> = input.clone();
        for _ in 0..80 {
            let mut new_fishes = vec![];
            state = state
                .iter()
                .map(|v| {
                    if *v == 0 {
                        new_fishes.push(8);
                        6
                    } else {
                        v - 1
                    }
                })
                .collect();
            state.extend(new_fishes);
        }
        state.len()
    }

    fn part2(input: &Vec<u32>) -> Self::Part2Output {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const EXAMPLE: &str = r#"3,4,3,1,2"#;

    #[test]
    fn test_part1() {
        let input = <AdventOfCode2021 as ParseInput<Day6>>::parse_input(EXAMPLE);
        assert_eq!(<AdventOfCode2021 as Solution<Day6>>::part1(&input), 5934);
    }

    #[test]
    fn test_part2() {
        let input = <AdventOfCode2021 as ParseInput<Day6>>::parse_input(EXAMPLE);
        assert_eq!(
            <AdventOfCode2021 as Solution<Day6>>::part2(&input),
            26984457539
        );
    }

    #[test]
    fn test_answers() {
        let input_file = "data/2021/day06_github.txt";
        let input_str =
            read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
        let input = <AdventOfCode2021 as ParseInput<Day6>>::parse_input(&input_str);
        assert_eq!(<AdventOfCode2021 as Solution<Day6>>::part1(&input), 388739);
        assert_eq!(
            <AdventOfCode2021 as Solution<Day6>>::part2(&input),
            1741362314973
        );
    }
}

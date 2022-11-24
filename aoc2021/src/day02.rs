#[allow(unused_imports)]
use aoc_traits::{days::Day2, ParseInput, Part1, Part2, Solution};

use super::AdventOfCode2021;

impl aoc_traits::MissingPartOne<Day2> for AdventOfCode2021<Day2> {}
/*
impl<'a> ParseInput<'a, Day2, Part1> for AdventOfCode2021<Day2> {
    type Parsed = Vec<u32>;

    fn parse_input(&'a self, _input: &'a str) -> Self::Parsed {
        vec![1, 2, 3]
    }
}

impl<'a> Solution<'a, Day2, Part1> for AdventOfCode2021<Day2> {
    type Input = Vec<u32>;
    type Output = u32;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.iter().sum()
    }
}
*/

impl aoc_traits::MissingPartTwo<Day2> for AdventOfCode2021<Day2> {}
/*
impl<'a> ParseInput<'a, Day2, Part2> for AdventOfCode2021<Day2> {
    type Parsed = &'a str;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed {
        input
    }
}

impl<'a> Solution<'a, Day2, Part2> for AdventOfCode2021<Day2> {
    type Input = &'a str;
    type Output = u32;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.len() as u32
    }
}
*/

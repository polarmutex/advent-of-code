#[allow(unused_imports)]
use aoc_traits::{days::Day12, ParseInput, Part1, Part2, Solution};

use super::AdventOfCode2021;

impl aoc_traits::MissingPartOne<Day12> for AdventOfCode2021<Day12> {}
/*
impl<'a> ParseInput<'a, Day12, Part1> for AdventOfCode2021<Day12> {
    type Parsed = Vec<u32>;

    fn parse_input(&'a self, _input: &'a str) -> Self::Parsed {
        vec![1, 2, 3]
    }
}

impl<'a> Solution<'a, Day12, Part1> for AdventOfCode2021<Day12> {
    type Input = Vec<u32>;
    type Output = u32;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.iter().sum()
    }
}
*/

impl aoc_traits::MissingPartTwo<Day12> for AdventOfCode2021<Day12> {}
/*
impl<'a> ParseInput<'a, Day12, Part2> for AdventOfCode2021<Day12> {
    type Parsed = &'a str;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed {
        input
    }
}

impl<'a> Solution<'a, Day12, Part2> for AdventOfCode2021<Day12> {
    type Input = &'a str;
    type Output = u32;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.len() as u32
    }
}
*/

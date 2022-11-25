#[allow(unused_imports)]
use framework::traits::{Day5, MissingPartOne, MissingPartTwo, ParseInput, Part1, Part2, Solution};

use super::AdventOfCode;

impl MissingPartOne<Day5> for AdventOfCode<Day5> {}
/*
impl<'a> ParseInput<'a, Day5, Part1> for AdventOfCode<Day5> {
    type Parsed = Vec<u32>;

    fn parse_input(&'a self, _input: &'a str) -> Self::Parsed {
        vec![1, 2, 3]
    }
}

impl<'a> Solution<'a, Day5, Part1> for AdventOfCode<Day5> {
    type Input = Vec<u32>;
    type Output = u32;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.iter().sum()
    }
}
*/

impl MissingPartTwo<Day5> for AdventOfCode<Day5> {}
/*
impl<'a> ParseInput<'a, Day5, Part2> for AdventOfCode<Day5> {
    type Parsed = &'a str;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed {
        input
    }
}

impl<'a> Solution<'a, Day5, Part2> for AdventOfCode<Day5> {
    type Input = &'a str;
    type Output = u32;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.len() as u32
    }
}
*/

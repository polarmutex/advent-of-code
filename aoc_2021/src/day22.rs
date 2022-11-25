#[allow(unused_imports)]
use framework::traits::{
    Day22, MissingPartOne, MissingPartTwo, ParseInput, Part1, Part2, Solution,
};

use super::AdventOfCode;

impl MissingPartOne<Day22> for AdventOfCode<Day22> {}
/*
impl<'a> ParseInput<'a, Day22, Part1> for AdventOfCode<Day22> {
    type Parsed = Vec<u32>;

    fn parse_input(&'a self, _input: &'a str) -> Self::Parsed {
        vec![1, 2, 3]
    }
}

impl<'a> Solution<'a, Day22, Part1> for AdventOfCode<Day22> {
    type Input = Vec<u32>;
    type Output = u32;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.iter().sum()
    }
}
*/

impl MissingPartTwo<Day22> for AdventOfCode<Day22> {}
/*
impl<'a> ParseInput<'a, Day22, Part2> for AdventOfCode<Day22> {
    type Parsed = &'a str;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed {
        input
    }
}

impl<'a> Solution<'a, Day22, Part2> for AdventOfCode<Day22> {
    type Input = &'a str;
    type Output = u32;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.len() as u32
    }
}
*/

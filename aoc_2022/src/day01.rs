use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use nom::character::complete::line_ending;
use nom::character::complete::u32;
use nom::multi::{fold_many1, separated_list1};
use nom_supreme::ParserExt;
use std::cmp::Reverse;

boilerplate!(
    Day,
    1,
    "\
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
",
    "data/01.txt"
);

impl Solution for Day {
    type Parsed = Vec<u32>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 24000;
    const ANSWER_1: Self::Answer = 68802;
    const EXAMPLE_ANSWER_2: Self::Answer = 45000;
    const ANSWER_2: Self::Answer = 205370;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        separated_list1(
            line_ending,
            fold_many1(
                u32.terminated(line_ending),
                || 0,
                |total, item| total + item,
            ),
        )(data)
    }

    fn part1(data: Self::Parsed) -> Self::Answer {
        data.into_iter().max().expect("At least one elf")
    }

    fn part2(data: Self::Parsed) -> Self::Answer {
        data.into_iter()
            .sorted_unstable_by_key(|&cals| Reverse(cals))
            .take(3)
            .sum()
    }
}

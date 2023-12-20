use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use lending_iterator::prelude::*;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::Parser;
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    9,
    "\
",
    "data/09.txt"
);

impl Solution for Day {
    type Parsed = Vec<Vec<i32>>;
    type Answer = i32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        let (input, t) =
            separated_list1(line_ending, many1(terminated(complete::i32, space0))).parse(input)?;
        Ok((input, t))
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        dbg!(&data);
        data.iter()
            .map(|d| {
                let mut nums = d.clone();
                std::iter::from_fn(move || {
                    if nums.iter().all(|n| n == &0) {
                        None
                    } else {
                        let mut it = nums.windows_mut();
                        while let Some(&mut [ref mut left, right]) = it.next() {
                            *left = right - *left;
                        }
                        dbg!(&nums);
                        nums.pop()
                    }
                })
                .sum::<i32>()
            })
            .sum()
    }

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
        data.iter()
            .map(|d| {
                let mut nums = d.clone().into_iter().rev().collect_vec();
                std::iter::from_fn(move || {
                    if nums.iter().all(|n| n == &0) {
                        None
                    } else {
                        let mut it = nums.windows_mut();
                        while let Some(&mut [ref mut left, right]) = it.next() {
                            *left -= right;
                        }
                        dbg!(&nums);
                        nums.pop()
                    }
                })
                .collect_vec()
                .into_iter()
                .rev()
                .fold(0, |acc, num| num - acc)
            })
            .sum()
    }
}

tests! {
     const EXAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 114);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 1887980197);
    add_test!(part2_example, test_part2_example, EXAMPLE => 2);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 990);
}

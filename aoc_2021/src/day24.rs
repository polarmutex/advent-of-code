use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    24,
    "\
",
    "data/24.txt"
);

impl Solution for Day {
    type Parsed = Vec<u32>;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 13579246899999;
    const ANSWER_1: Self::Answer = 94399898949959;
    const EXAMPLE_ANSWER_2: Self::Answer = 0;
    const ANSWER_2: Self::Answer = 21176121611511;

    fn parse(_input: &str) -> IResult<Self::Parsed> {
        let i = vec![];
        Ok(("", i))
    }

    fn part1(_input: Self::Parsed) -> Self::Answer {
        todo!()
    }

    fn part2(_input: Self::Parsed) -> Self::Answer {
        todo!()
    }
}

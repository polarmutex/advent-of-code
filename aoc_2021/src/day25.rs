use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    25,
    "\
",
    "data/25.txt"
);

impl Solution for Day {
    type Parsed = Vec<u32>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 58;
    const ANSWER_1: Self::Answer = 424;
    const EXAMPLE_ANSWER_2: Self::Answer = 0;
    const ANSWER_2: Self::Answer = 0;

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

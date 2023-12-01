use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    19,
    "\
",
    "data/19.txt"
);

impl Solution for Day {
    type Parsed = Vec<u32>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 79;
    const ANSWER_1: Self::Answer = 350;
    const EXAMPLE_ANSWER_2: Self::Answer = 3621;
    const ANSWER_2: Self::Answer = 10895;

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

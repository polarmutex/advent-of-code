use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    23,
    "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
",
    "data/23.txt"
);

impl Solution for Day {
    type Parsed = Vec<u32>;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 12521;
    const ANSWER_1: Self::Answer = 14350;
    const EXAMPLE_ANSWER_2: Self::Answer = 44169;
    const ANSWER_2: Self::Answer = 49742;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        Ok((input, vec![]))
    }

    fn part1(_input: Self::Parsed) -> Self::Answer {
        todo!()
    }

    fn part2(_input: Self::Parsed) -> Self::Answer {
        todo!()
    }
}

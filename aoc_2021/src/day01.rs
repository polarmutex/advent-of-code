use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;

boilerplate!(
    Day,
    1,
    "\
199
200
208
210
200
207
240
269
260
263",
    "data/01.txt"
);

impl Solution for Day {
    type Parsed = Vec<u32>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 7;
    const ANSWER_1: Self::Answer = 1448;
    const EXAMPLE_ANSWER_2: Self::Answer = 5;
    const ANSWER_2: Self::Answer = 1471;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let nums: Vec<u32> = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect();
        Ok(("", nums))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        input
            .iter()
            .tuple_windows()
            //Return an iterator over all contiguous windows producing tuples of a
            // specific size (up to 12). tuple_windows clones the iterator elements
            // so that they can be part of successive windows, this makes it most
            // suited for iterators of references and other values that are cheap to copy.
            .filter(|&(&a, &b)| b > a)
            .count() as u32
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        input
            .iter()
            .tuple_windows()
            .map(|(&a, &b, &c)| a + b + c)
            .tuple_windows()
            .filter(|&(a, b)| b > a)
            .count() as u32
    }
}

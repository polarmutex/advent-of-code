use ahash::AHashSet;
use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    6,
    "\
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
",
    "data/06.txt"
);

fn solve(input: &[char], window: usize) -> usize {
    input
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(window)
        .find(|w| w.iter().map(|p| p.1).collect::<AHashSet<_>>().len() == window)
        .unwrap()[0]
        .0
        + window
}

impl Solution for Day {
    type Parsed = Vec<char>;
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 11;
    const ANSWER_1: Self::Answer = 1042;
    const EXAMPLE_ANSWER_2: Self::Answer = 26;
    const ANSWER_2: Self::Answer = 2980;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let chars = input.chars().collect();
        Ok(("", chars))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        solve(&input, 4)
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        solve(&input, 14)
    }
}

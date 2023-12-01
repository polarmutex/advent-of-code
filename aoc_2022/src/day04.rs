use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::u32;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::ops::RangeInclusive;

boilerplate!(
    Day,
    4,
    "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
",
    "data/04.txt"
);

type RangesPair = [RangeInclusive<u32>; 2];

fn sections(input: &str) -> IResult<RangeInclusive<u32>> {
    let (input, (start, end)) = separated_pair(u32, tag("-"), u32)(input)?;
    Ok((input, start..=end))
}
fn line(input: &str) -> IResult<RangesPair> {
    let (input, (start, end)) = separated_pair(sections, tag(","), sections)(input)?;
    Ok((input, [start, end]))
}
fn section_assignments(input: &str) -> IResult<Vec<RangesPair>> {
    let (input, ranges) = separated_list1(newline, line)(input)?;

    Ok((input, ranges))
}

impl Solution for Day {
    type Parsed = Vec<RangesPair>;
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 2;
    const ANSWER_1: Self::Answer = 573;
    const EXAMPLE_ANSWER_2: Self::Answer = 4;
    const ANSWER_2: Self::Answer = 867;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        section_assignments(data)
    }

    fn part1(data: Self::Parsed) -> Self::Answer {
        data.iter()
            .filter(|[range_a, range_b]| {
                let a_contains_b = range_a.clone().all(|num| range_b.contains(&num));
                let b_contains_a = range_b.clone().all(|num| range_a.contains(&num));
                a_contains_b || b_contains_a
            })
            .count()
    }

    fn part2(data: Self::Parsed) -> Self::Answer {
        data.iter()
            .filter(|[range_a, range_b]| {
                let a_contains_b = range_a.clone().any(|num| range_b.contains(&num));
                let b_contains_a = range_b.clone().any(|num| range_a.contains(&num));
                a_contains_b || b_contains_a
            })
            .count()
    }
}

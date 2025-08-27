use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::u32;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::ops::RangeInclusive;

type Input = Vec<[RangeInclusive<u32>; 2]>;

fn sections(input: &str) -> nom::IResult<&str, RangeInclusive<u32>> {
    let (input, (start, end)) = separated_pair(u32, tag("-"), u32)(input)?;
    Ok((input, start..=end))
}

fn line(input: &str) -> nom::IResult<&str, [RangeInclusive<u32>; 2]> {
    let (input, (start, end)) = separated_pair(sections, tag(","), sections)(input)?;
    Ok((input, [start, end]))
}

#[aoc(2022, day4)]
pub mod solutions {
    use super::*;

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        separated_list1(newline, line)(data)
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> usize {
        input.iter()
            .filter(|[range_a, range_b]| {
                let a_contains_b = range_a.clone().all(|num| range_b.contains(&num));
                let b_contains_a = range_b.clone().all(|num| range_a.contains(&num));
                a_contains_b || b_contains_a
            })
            .count()
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> usize {
        input.iter()
            .filter(|[range_a, range_b]| {
                let a_contains_b = range_a.clone().any(|num| range_b.contains(&num));
                let b_contains_a = range_b.clone().any(|num| range_a.contains(&num));
                a_contains_b || b_contains_a
            })
            .count()
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> usize {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> usize {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 2);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 4);
    }

}

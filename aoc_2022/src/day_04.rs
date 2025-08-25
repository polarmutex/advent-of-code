use common::{solution, Answer};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::u32;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::ops::RangeInclusive;

solution!("Camp Cleanup", 4);

type Input = Vec<[RangeInclusive<u32>; 2]>;

fn sections(input: &str) -> nom::IResult<&str, RangeInclusive<u32>> {
    let (input, (start, end)) = separated_pair(u32, tag("-"), u32)(input)?;
    Ok((input, start..=end))
}

fn line(input: &str) -> nom::IResult<&str, [RangeInclusive<u32>; 2]> {
    let (input, (start, end)) = separated_pair(sections, tag(","), sections)(input)?;
    Ok((input, [start, end]))
}

fn parse(data: &str) -> nom::IResult<&str, Input> {
    separated_list1(newline, line)(data)
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data.iter()
        .filter(|[range_a, range_b]| {
            let a_contains_b = range_a.clone().all(|num| range_b.contains(&num));
            let b_contains_a = range_b.clone().all(|num| range_a.contains(&num));
            a_contains_b || b_contains_a
        })
        .count();
    
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data.iter()
        .filter(|[range_a, range_b]| {
            let a_contains_b = range_a.clone().any(|num| range_b.contains(&num));
            let b_contains_a = range_b.clone().any(|num| range_a.contains(&num));
            a_contains_b || b_contains_a
        })
        .count();
    
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
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
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 2.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 4.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 4)?;
        assert_eq!(super::part_1(input.as_str())?, 573.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 4)?;
        assert_eq!(super::part_2(input.as_str())?, 867.into());
        Ok(())
    }
}

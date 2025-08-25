use common::{solution, Answer};
use itertools::Itertools;
use lending_iterator::prelude::*;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::Parser;

solution!("Mirage Maintenance", 9);

type Input = Vec<Vec<i32>>;

#[tracing::instrument(skip(input))]
fn parse(input: &str) -> nom::IResult<&str, Input> {
    let (input, t) =
        separated_list1(line_ending, many1(terminated(complete::i32, space0))).parse(input)?;
    Ok((input, t))
}

#[tracing::instrument(skip(input))]
fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    dbg!(&data);
    let result = data.iter()
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
        .sum::<i32>();
        
    Ok(result.into())
}

#[tracing::instrument(skip(input))]
fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data.iter()
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
        .sum::<i32>();
        
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 114.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 2.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 9)?;
        assert_eq!(super::part_1(input.as_str())?, 1887980197.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 9)?;
        assert_eq!(super::part_2(input.as_str())?, 990.into());
        Ok(())
    }
}

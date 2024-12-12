use core::panic;

use common::{solution, Answer};
use itertools::Itertools;
use miette::miette;
use nom::character::complete;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::IResult;
use tracing::instrument;

solution!("Red-Nosed Reports", 2);

type Report = Vec<u32>;
fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(
        complete::line_ending,
        separated_list1(space1, complete::u32),
    )(input)
}

#[tracing::instrument(skip(input))]
fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, reports) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    // dbg!(&reports);
    Ok(reports.iter().filter(|r| check(r).is_ok()).count().into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, reports) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    Ok(reports
        .iter()
        .filter(|r| {
            if check(r).is_err() {
                for i in 0..r.len() {
                    let mut nr = (*r).clone();
                    nr.remove(i);
                    if check(&nr).is_ok() {
                        return true;
                    } else {
                        continue;
                    }
                }
                false
            } else {
                true
            }
        })
        .count()
        .into())
}

enum Direction {
    Increasing,
    Decreasing,
}

#[instrument(ret)]
fn check(report: &Report) -> Result<(), String> {
    let mut dir: Option<Direction> = None;
    for (a, b) in report.iter().tuple_windows() {
        let diff = *b as i32 - *a as i32;
        match dir {
            None => match diff.signum() {
                -1 => {
                    dir = Some(Direction::Decreasing);
                    if (1..=3).contains(&diff.abs()) {
                        continue;
                    } else {
                        return Err(format!("{}, {} diff is {}", a, b, diff));
                    }
                }
                0 => return Err(format!("{}, {} diff is {}", a, b, diff)),
                1 => {
                    dir = Some(Direction::Increasing);
                    if (1..=3).contains(&diff.abs()) {
                        continue;
                    } else {
                        return Err(format!("{}, {} diff is {}", a, b, diff));
                    }
                }
                _ => panic!("valu should always be -1, 0, 1"),
            },
            Some(Direction::Increasing) => match diff.signum() {
                -1 => return Err(format!("{}, {} diff is {}", a, b, diff)),
                0 => return Err(format!("{}, {} diff is {}", a, b, diff)),
                1 => {
                    if (1..=3).contains(&diff.abs()) {
                        continue;
                    } else {
                        return Err(format!("{}, {} diff is {}", a, b, diff));
                    }
                }
                _ => panic!("valu should always be -1, 0, 1"),
            },
            Some(Direction::Decreasing) => match diff.signum() {
                -1 => {
                    if (1..=3).contains(&diff.abs()) {
                        continue;
                    } else {
                        return Err(format!("{}, {} diff is {}", a, b, diff));
                    }
                }
                0 => return Err(format!("{}, {} diff is {}", a, b, diff)),
                1 => return Err(format!("{}, {} diff is {}", a, b, diff)),
                _ => panic!("valu should always be -1, 0, 1"),
            },
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const CASE: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    // #[test_log::test]
    fn part_1_case() -> miette::Result<()> {
        assert_eq!(super::part_1(CASE)?, 2.into());
        Ok(())
    }

    #[test]
    // #[test_log::test]
    fn part_2_case() -> miette::Result<()> {
        assert_eq!(super::part_2(CASE)?, 4.into());
        Ok(())
    }

    #[test]
    // #[test_log::test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2024, 2)?;
        assert_eq!(super::part_1(input.as_str())?, 321.into());
        Ok(())
    }

    #[test]
    // #[test_log::test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2024, 2)?;
        assert_eq!(super::part_2(input.as_str())?, 386.into());
        Ok(())
    }
}

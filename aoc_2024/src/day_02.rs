use aoc_runner_macros::{aoc, generator, solver, solution};
use core::panic;
use itertools::Itertools;
use nom::character::complete;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::IResult;
use tracing::instrument;

type Report = Vec<u32>;

#[aoc(2024, day2)]
pub mod solutions {
    use super::*;

    fn parse(input: &str) -> IResult<&str, Vec<Report>> {
        separated_list1(
            complete::line_ending,
            separated_list1(space1, complete::u32),
        )(input)
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Vec<Report> {
        let (_, reports) = parse(input).unwrap();
        reports
    }

    #[solver(part1, main)]
    pub fn solve_part_1(reports: Vec<Report>) -> usize {
        reports.iter().filter(|r| check(r).is_ok()).count()
    }

    #[solver(part2, main)]
    pub fn solve_part_2(reports: Vec<Report>) -> usize {
        reports
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
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> usize {
        let reports = input_generator(input);
        solve_part_1(reports)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> usize {
        let reports = input_generator(input);
        solve_part_2(reports)
    }
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
mod tests {
    use aoc_runner_macros::aoc_case;

    #[aoc_case(2, 4)]
    const CASE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
}

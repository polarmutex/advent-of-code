use common::{solution, Answer};
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::{branch::alt, character::complete::char, sequence::separated_pair};

solution!("Rock Paper Scissors", 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

use Outcome::*;
use RockPaperScissor::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Round {
    opp: RockPaperScissor,
    me: Outcome,
}

type Input = Vec<Round>;

fn parse(data: &str) -> nom::IResult<&str, Input> {
    separated_list1(
        line_ending,
        map(
            separated_pair(
                map(alt((char('A'), char('B'), char('C'))), |c: char| match c {
                    'A' => Rock,
                    'B' => Paper,
                    'C' => Scissor,
                    _ => unreachable!(),
                }),
                char(' '),
                map(alt((char('X'), char('Y'), char('Z'))), |c: char| match c {
                    'X' => Lose,
                    'Y' => Draw,
                    'Z' => Win,
                    _ => unreachable!(),
                }),
            ),
            |(x, y)| Round { me: y, opp: x },
        ),
    )(data)
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut score = 0;
    for round in data {
        let mut round_score = 0;
        let selection = match round.me {
            Lose => Rock,
            Draw => Paper,
            Win => Scissor,
        };
        match round.me {
            Lose => round_score += 1,
            Draw => round_score += 2,
            Win => round_score += 3,
        }
        if selection == Rock && round.opp == Scissor {
            round_score += 6;
        }
        if selection == Scissor && round.opp == Paper {
            round_score += 6;
        }
        if selection == Paper && round.opp == Rock {
            round_score += 6;
        }

        if selection == Rock && round.opp == Rock {
            round_score += 3;
        }
        if selection == Scissor && round.opp == Scissor {
            round_score += 3;
        }
        if selection == Paper && round.opp == Paper {
            round_score += 3;
        }

        score += round_score;
    }
    Ok(score.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut score = 0;
    for round in data {
        let mut round_score = 0;

        let selection = match round.me {
            Lose => match round.opp {
                Rock => Scissor,
                Paper => Rock,
                Scissor => Paper,
            },
            Draw => match round.opp {
                Rock => Rock,
                Paper => Paper,
                Scissor => Scissor,
            },
            Win => match round.opp {
                Rock => Paper,
                Paper => Scissor,
                Scissor => Rock,
            },
        };
        match selection {
            Rock => round_score += 1,
            Paper => round_score += 2,
            Scissor => round_score += 3,
        }
        if round.me == Win {
            round_score += 6;
        }
        if round.me == Draw {
            round_score += 3;
        }

        score += round_score;
    }
    Ok(score.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        A Y
        B X
        C Z
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 15.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 12.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 2)?;
        assert_eq!(super::part_1(input.as_str())?, 11873.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 2)?;
        assert_eq!(super::part_2(input.as_str())?, 12014.into());
        Ok(())
    }
}

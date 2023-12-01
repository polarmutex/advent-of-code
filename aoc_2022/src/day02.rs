use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::{branch::alt, character::complete::char, sequence::separated_pair};

boilerplate!(
    Day,
    2,
    "\
A Y
B X
C Z
",
    "data/02.txt"
);

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

impl Solution for Day {
    type Parsed = Vec<Round>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 15;
    const ANSWER_1: Self::Answer = 11873;
    const EXAMPLE_ANSWER_2: Self::Answer = 12;
    const ANSWER_2: Self::Answer = 12014;

    fn parse(data: &str) -> IResult<Self::Parsed> {
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

    fn part1(data: Self::Parsed) -> Self::Answer {
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
        score
    }

    fn part2(data: Self::Parsed) -> Self::Answer {
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
        score
    }
}

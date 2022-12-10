use crate::prelude::*;

day!(2, parse => part1, part2);

//fn input_parser() -> impl Parser<char, Vec<Vec<u32>>, Error = Simple<char>> {
//}

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

struct Round {
    opp: RockPaperScissor,
    me: Outcome,
}

fn parse(input: &str) -> ParseResult<Vec<Round>> {
    Ok(input
        .trim()
        .split('\n')
        .map(|r| Round {
            opp: match r.chars().next().unwrap() {
                'A' => Rock,
                'B' => Paper,
                'C' => Scissor,
                _ => unreachable!(),
            },
            me: match r.chars().nth(2).unwrap() {
                'X' => Lose,
                'Y' => Draw,
                'Z' => Win,
                _ => unreachable!(),
            },
        })
        .collect())
}

fn part1(input: &[Round]) -> u32 {
    let mut score = 0;
    for round in input {
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

fn part2(input: &[Round]) -> u32 {
    let mut score = 0;
    for round in input {
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

tests! {
    const EXAMPLE: &str = "\
A Y
B X
C Z
";
    const INPUT: &str = include_str!("data/02.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 15); // 8 + 1 + 6
    simple_tests!(parse, part1, part1_input_test, INPUT => 11873);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 12);
    simple_tests!(parse, part2, part2_input_test, INPUT => 12014);
}

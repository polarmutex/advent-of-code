use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::{branch::alt, character::complete::char, sequence::separated_pair};

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
pub struct Round {
    opp: RockPaperScissor,
    me: Outcome,
}

type Input = Vec<Round>;

#[aoc(2022, day2)]
pub mod solutions {
    use super::*;

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

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u32 {
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

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u32 {
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

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        A Y
        B X
        C Z
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 15);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 12);
    }

}

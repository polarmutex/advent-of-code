use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::anychar;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;

#[derive(Debug, Clone)]
pub enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}
fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discarded, ins)| ins))(input)
}

#[derive(PartialEq, Eq)]
pub enum State {
    Process,
    DontProcess,
}

#[aoc(2024, day3)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Vec<Instruction> {
        let (_, instructions) = parse(input).unwrap();
        instructions
    }

    #[solver(part1, main)]
    pub fn solve_part_1(instructions: Vec<Instruction>) -> u32 {
        instructions
            .iter()
            .fold(0, |ans, ins| match ins {
                Instruction::Mul(l, r) => ans + (l * r),
                _ => ans,
            })
    }

    #[solver(part2, main)]
    pub fn solve_part_2(instructions: Vec<Instruction>) -> u32 {
        instructions
            .iter()
            .fold((0, State::Process), |(ans, state), ins| match ins {
                Instruction::Mul(l, r) => match state {
                    State::Process => (ans + (l * r), state),
                    State::DontProcess => (ans, state),
                },
                Instruction::Do => (ans, State::Process),
                Instruction::Dont => (ans, State::DontProcess),
            })
            .0
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> u32 {
        let instructions = input_generator(input);
        solve_part_1(instructions)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> u32 {
        let instructions = input_generator(input);
        solve_part_2(instructions)
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner_macros::aoc_case;
    use super::solutions::*;

    #[aoc_case(161, 161)]
    const CASE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    
    const CASE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    
    #[test]
    fn part_2_custom() {
        assert_eq!(part_2(CASE2), 48);
    }
}

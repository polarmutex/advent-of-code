use crate::prelude::*;

day!(2, parse => part1, part2);

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn input_parser() -> impl Parser<char, Vec<Instruction>, Error = Simple<char>> {
    let number = c::text::int(10).map(|s: String| s.parse().unwrap());
    let instruction =
        c::text::ident()
            .then_ignore(just(' '))
            .then(number)
            .map(|(direction, count)| match direction.as_str() {
                "forward" => Instruction::Forward(count),
                "down" => Instruction::Down(count),
                "up" => Instruction::Up(count),
                _ => unreachable!(),
            });
    instruction.separated_by(c::text::newline())
}

fn parse(input: &str) -> ParseResult<Vec<Instruction>> {
    Ok(input_parser().parse(input).unwrap())
}

fn part1(input: &[Instruction]) -> MulSubmission<u32> {
    let mut hpos = 0;
    let mut depth = 0;
    for instr in input {
        match instr {
            Instruction::Forward(amount) => hpos += amount,
            Instruction::Down(amount) => depth += amount,
            Instruction::Up(amount) => depth -= amount,
        }
    }
    MulSubmission(hpos, depth)
}

fn part2(input: &[Instruction]) -> MulSubmission<u32> {
    let mut aim = 0;
    let mut hpos = 0;
    let mut depth = 0;
    for instr in input {
        match instr {
            Instruction::Forward(amount) => {
                hpos += amount;
                depth += aim * amount;
            }
            Instruction::Down(amount) => aim += amount,
            Instruction::Up(amount) => aim -= amount,
        }
    }
    MulSubmission(hpos, depth)
}

tests! {
    const EXAMPLE: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";
    const INPUT: &str = include_str!("../../data/2021/02.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => MulSubmission(15,10));
    simple_tests!(parse, part1, part1_input_test, INPUT => MulSubmission(1909,655));
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => MulSubmission(15,60));
    simple_tests!(parse, part2, part2_input_test, INPUT => MulSubmission(1909,760194));
}

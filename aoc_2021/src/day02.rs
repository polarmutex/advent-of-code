use crate::prelude::*;

day!(2, parse => part1, part2);

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let (instr, num) = input.split_once(' ').expect("instruction to be found");
        let num = num.parse::<u32>().expect("number to be found");
        match instr {
            "forward" => Ok(Instruction::Forward(num)),
            "down" => Ok(Instruction::Down(num)),
            "up" => Ok(Instruction::Up(num)),
            _ => anyhow::bail!("Could not match instruction"),
        }
    }
}

fn parse(input: &str) -> ParseResult<Vec<Instruction>> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| line.parse::<Instruction>().expect("valid instructions"))
        .collect();
    Ok(instructions)
    //Ok(input_parser().parse(input).unwrap())
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

use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    2,
    "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
",
    "data/02.txt"
); //, "data/example/01.txt", "data/input/01.txt");

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

impl Solution for Day {
    type Parsed = Vec<Instruction>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 150;
    const ANSWER_1: Self::Answer = 1250395;
    const EXAMPLE_ANSWER_2: Self::Answer = 900;
    const ANSWER_2: Self::Answer = 1451210346;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let instructions: Vec<Instruction> = input
            .lines()
            .map(|line| line.parse::<Instruction>().expect("valid instructions"))
            .collect();
        Ok(("", instructions))
        //Ok(input_parser().parse(input).unwrap())
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let mut hpos = 0;
        let mut depth = 0;
        for instr in input {
            match instr {
                Instruction::Forward(amount) => hpos += amount,
                Instruction::Down(amount) => depth += amount,
                Instruction::Up(amount) => depth -= amount,
            }
        }
        hpos * depth
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
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
        hpos * depth
    }
}

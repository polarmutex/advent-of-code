use aoc_runner_macros::{aoc, generator, solver, solution};


#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::str::FromStr for Instruction {
    type Err = miette::Error;
    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let (instr, num) = input.split_once(' ').ok_or_else(|| miette::miette!("Invalid instruction format"))?;
        let num = num.parse::<u32>().map_err(|e| miette::miette!("Invalid number: {}", e))?;
        match instr {
            "forward" => Ok(Instruction::Forward(num)),
            "down" => Ok(Instruction::Down(num)),
            "up" => Ok(Instruction::Up(num)),
            _ => Err(miette::miette!("Unknown instruction: {}", instr)),
        }
    }
}

type Input = Vec<Instruction>;

#[aoc(2021, day2)]
pub mod solutions {
    use super::*;

    fn parse(input: &str) -> nom::IResult<&str, Input> {
        let instructions: Result<Vec<Instruction>, _> = input
            .lines()
            .map(|line| line.parse::<Instruction>())
            .collect();
        
        match instructions {
            Ok(instr) => Ok(("", instr)),
            Err(_e) => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::MapRes))),
        }
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u32 {
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

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u32 {
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
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 150);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 900);
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        Ok(())
    }
}
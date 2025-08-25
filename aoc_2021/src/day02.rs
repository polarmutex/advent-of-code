use common::{solution, Answer};

solution!("Dive!", 2);

#[derive(Debug, Copy, Clone)]
enum Instruction {
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

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, instructions) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut hpos = 0;
    let mut depth = 0;
    for instr in instructions {
        match instr {
            Instruction::Forward(amount) => hpos += amount,
            Instruction::Down(amount) => depth += amount,
            Instruction::Up(amount) => depth -= amount,
        }
    }
    Ok((hpos * depth).into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, instructions) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut aim = 0;
    let mut hpos = 0;
    let mut depth = 0;
    for instr in instructions {
        match instr {
            Instruction::Forward(amount) => {
                hpos += amount;
                depth += aim * amount;
            }
            Instruction::Down(amount) => aim += amount,
            Instruction::Up(amount) => aim -= amount,
        }
    }
    Ok((hpos * depth).into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
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
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 150.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 900.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2021, 2)?;
        assert_eq!(super::part_1(input.as_str())?, 1250395.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2021, 2)?;
        assert_eq!(super::part_2(input.as_str())?, 1451210346.into());
        Ok(())
    }
}

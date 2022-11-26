use crate::prelude::*;

day!(2, parse => part1, part2);

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

type Instruction = (Direction, u32);

fn part1(input: &[Instruction]) -> Result<u32> {
    let mut hpos = 0;
    let mut depth = 0;
    for &(direction, amount) in input {
        let amount = amount as u32;
        match direction {
            Direction::Forward => hpos += amount,
            Direction::Down => depth += amount,
            Direction::Up => depth -= amount,
        }
    }
    Ok(hpos * depth)
}

fn part2(input: &[Instruction]) -> Result<u32> {
    let mut aim = 0;
    let mut hpos = 0;
    let mut depth = 0;
    for &(direction, amount) in input {
        let amount = amount as u32;
        match direction {
            Direction::Forward => {
                hpos += amount;
                depth += aim * amount;
            }
            Direction::Down => aim += amount,
            Direction::Up => aim -= amount,
        }
    }
    Ok(hpos * depth)
}

fn parse(input: &[u8]) -> ParseResult<Vec<Instruction>> {
    use parsers::*;
    let direction = token((b"forward ", Direction::Forward))
        .or(token((b"down ", Direction::Down)))
        .or(token((b"up ", Direction::Up)));
    let instruction = direction.and(number::<u32>());
    instruction.sep_by(token(b'\n')).parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 150);
    input_tests!(2021, 2, parse, part1, part1_input_test, 1250395);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 900);
    input_tests!(2021, 2, parse, part2, part2_input_test, 1451210346);
}

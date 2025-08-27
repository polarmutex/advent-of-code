use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::IResult;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
};

type Input = Vec<Command>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Command {
    On((Point, Point)),
    Off((Point, Point)),
}
fn command(input: &str) -> IResult<&str, Command> {
    let (input, action) = alt((tag("on"), tag("off")))(input)?;
    let (input, _) = tag(" x=")(input)?;
    let (input, (low_x, high_x)) = separated_pair(complete::i32, tag(".."), complete::i32)(input)?;
    let (input, _) = tag(",y=")(input)?;

    let (input, (low_y, high_y)) = separated_pair(complete::i32, tag(".."), complete::i32)(input)?;
    let (input, _) = tag(",z=")(input)?;
    let (input, (low_z, high_z)) = separated_pair(complete::i32, tag(".."), complete::i32)(input)?;

    let cmd = match action {
        "on" => Command::On((
            Point {
                x: low_x,
                y: low_y,
                z: low_z,
            },
            Point {
                x: high_x,
                y: high_y,
                z: high_z,
            },
        )),
        "off" => Command::Off((
            Point {
                x: low_x,
                y: low_y,
                z: low_z,
            },
            Point {
                x: high_x,
                y: high_y,
                z: high_z,
            },
        )),
        _ => {
            panic!("action")
        }
    };
    Ok((input, cmd))
}

#[aoc(2021, day22)]
pub mod solutions {
    use super::*;

    fn parse(input: &str) -> nom::IResult<&str, Input> {
        let (input, commands) = separated_list1(newline, command)(input)?;
        Ok((input, commands))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(_input: &Input) -> u64 {
        todo!()
    }

    #[solver(part2, gen)]
    pub fn solve_part2(_input: &Input) -> u64 {
        todo!()
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    #[ignore]
    fn test_part_1() {
        assert_eq!(solutions::part_1(EXAMPLE), 590784);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(solutions::part_2(EXAMPLE), 2758514936282235);
    }
}

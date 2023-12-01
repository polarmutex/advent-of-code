use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
};

boilerplate!(
    Day,
    22,
    "\
",
    "data/22.txt"
);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Debug)]
enum Command {
    On((Point, Point)),
    Off((Point, Point)),
}
fn command(input: &str) -> IResult<Command> {
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

impl Solution for Day {
    type Parsed = Vec<Command>;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 590784;
    const ANSWER_1: Self::Answer = 615869;
    const EXAMPLE_ANSWER_2: Self::Answer = 2758514936282235;
    const ANSWER_2: Self::Answer = 1323862415207825;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let (input, commands) = separated_list1(newline, command)(input)?;
        Ok((input, commands))
    }

    fn part1(_input: Self::Parsed) -> Self::Answer {
        todo!()
    }

    fn part2(_input: Self::Parsed) -> Self::Answer {
        todo!()
    }
}

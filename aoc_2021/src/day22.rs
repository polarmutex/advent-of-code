use common::{solution, Answer};
use nom::IResult;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
};

solution!("Reactor Reboot", 22);

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

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, commands) = separated_list1(newline, command)(input)?;
    Ok((input, commands))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, _input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    todo!()
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, _input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    #[ignore]
    fn test_part_1() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_1(EXAMPLE).unwrap(), Answer::Number(590784));
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_2(EXAMPLE).unwrap(), Answer::Number(2758514936282235));
    }
}

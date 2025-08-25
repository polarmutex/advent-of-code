use common::{solution, Answer};
use nom::IResult;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
};

solution!("Dirac Dice", 21);

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Player {
    id: u64,
    start_position: u64,
    position: u64,
    score: u64,
}

fn player(input: &str) -> IResult<&str, Player> {
    let (input, _) = tag("Player ")(input)?;
    let (input, id) = complete::u64(input)?;
    let (input, _) = tag(" starting position: ")(input)?;
    let (input, start_position) = complete::u64(input)?;

    Ok((
        input,
        Player {
            id,
            start_position: start_position,
            position: start_position,
            score: 0,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Player>> {
    let (input, players) = separated_list1(newline, player)(input)?;
    Ok((input, players))
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

    const EXAMPLE: &str = "\
Player 2 starting position: 8
Player 1 starting position: 4
";

    #[test]
    #[ignore]
    fn test_part_1() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_1(EXAMPLE).unwrap(), Answer::Number(739785));
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_2(EXAMPLE).unwrap(), Answer::Number(444356092776315));
    }
}

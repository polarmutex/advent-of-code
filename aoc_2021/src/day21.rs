use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
};

boilerplate!(
    Day,
    21,
    "\
Player 2 starting position: 8
Player 1 starting position: 4
",
    "data/21.txt"
);

#[derive(Clone, Debug)]
struct Player {
    id: u64,
    start_position: u64,
    position: u64,
    score: u64,
}

fn player(input: &str) -> IResult<Player> {
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

impl Solution for Day {
    type Parsed = Vec<Player>;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 739785;
    const ANSWER_1: Self::Answer = 506466;
    const EXAMPLE_ANSWER_2: Self::Answer = 444356092776315;
    const ANSWER_2: Self::Answer = 632979211251440;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let (input, players) = separated_list1(newline, player)(input)?;
        Ok((input, players))
    }

    fn part1(_input: Self::Parsed) -> Self::Answer {
        todo!()
    }

    fn part2(_input: Self::Parsed) -> Self::Answer {
        todo!()
    }
}

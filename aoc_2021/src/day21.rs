use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::IResult;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
};

type Input = Vec<Player>;

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

#[aoc(2021, day21)]
pub mod solutions {
    use super::*;

    fn parse(input: &str) -> nom::IResult<&str, Input> {
        let (input, players) = separated_list1(newline, player)(input)?;
        Ok((input, players))
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

    const EXAMPLE: &str = "\
Player 2 starting position: 8
Player 1 starting position: 4
";

    #[test]
    #[ignore]
    fn test_part_1() {
        assert_eq!(solutions::part_1(EXAMPLE), 739785);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(solutions::part_2(EXAMPLE), 444356092776315);
    }
}

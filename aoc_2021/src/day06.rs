use aoc_runner_macros::{aoc, generator, solver, solution};


type Input = Vec<u64>;

#[aoc(2021, day6)]
pub mod solutions {
    use super::*;

    fn parse(input: &str) -> nom::IResult<&str, Input> {
        let fish: Result<Vec<u64>, _> = input
            .split(',')
            .map(|num| num.parse::<u64>())
            .collect();
        
        match fish {
            Ok(f) => Ok(("", f)),
            Err(_) => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::MapRes))),
        }
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u64 {
        sim_fish(80, input)
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u64 {
        sim_fish(256, input)
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

fn sim_fish(days: u32, input: &[u64]) -> u64 {
    let mut laternfish_timer_state: Vec<u64> = vec![0; 10];

    // initial
    for fish in input.iter() {
        let fish = *fish as usize;
        laternfish_timer_state[fish] += 1;
    }

    for _day in 0..days {
        let new_fish = laternfish_timer_state[0];
        laternfish_timer_state[0] = 0;
        for fish_state in 1..9 {
            laternfish_timer_state[fish_state - 1] += laternfish_timer_state[fish_state];
            laternfish_timer_state[fish_state] = 0;
        }
        laternfish_timer_state[6] += new_fish;
        laternfish_timer_state[8] = new_fish;
    }
    laternfish_timer_state.iter().sum()
}

#[cfg(test)]
mod test {

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 5934);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 26984457539);
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
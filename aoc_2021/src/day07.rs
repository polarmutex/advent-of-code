use aoc_runner_macros::{aoc, generator, solver, solution};


type Input = Vec<u32>;

#[aoc(2021, day7)]
pub mod solutions {
    use super::*;

    fn parse(input: &str) -> nom::IResult<&str, Input> {
        let crabs: Result<Vec<u32>, _> = input
            .split(',')
            .map(|val| val.parse::<u32>())
            .collect();
        
        match crabs {
            Ok(c) => Ok(("", c)),
            Err(_) => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::MapRes))),
        }
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u32 {
        let (low, high) = input.iter().fold((1_000, 0), |mut bounds, crab| {
            if bounds.0 > *crab {
                bounds.0 = *crab
            }
            if bounds.1 < *crab {
                bounds.1 = *crab
            }
            bounds
        });

        (low..high)
            .map(|pos| input.iter().map(|crab| crab.abs_diff(pos)).sum::<u32>())
            .min()
            .unwrap()
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u32 {
        let (low, high) = input.iter().fold((1_000, 0), |mut bounds, crab| {
            if bounds.0 > *crab {
                bounds.0 = *crab
            }
            if bounds.1 < *crab {
                bounds.1 = *crab
            }
            bounds
        });

        (low..high)
            .map(|pos| {
                input
                    .iter()
                    .map(|crab| {
                        let d = crab.abs_diff(pos);
                        d * (d + 1) / 2
                    })
                    .sum::<u32>()
            })
            .min()
            .unwrap()
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

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 37);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 168);
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
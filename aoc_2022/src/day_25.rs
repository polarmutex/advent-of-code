use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Snafu {
    val: String,
}
impl FromStr for Snafu {
    type Err = miette::Error;
    fn from_str(input: &str) -> Result<Snafu, Self::Err> {
        let val = Snafu {
            val: input.to_string(),
        };
        Ok(val)
    }
}
impl Snafu {
    pub fn to_decimal(&self) -> i64 {
        self.val.chars().rev().enumerate().fold(0, |acc, (i, c)| {
            let digit = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            };
            acc + 5_i64.pow(i as u32) * digit
        })
    }
    pub fn to_snafu(val: i64) -> Self {
        let mut s = String::from("");
        let mut val = val;
        while val > 0 {
            let (digit, c) = match val % 5 {
                0 => (0, '0'),
                1 => (1, '1'),
                2 => (2, '2'),
                3 => (-2, '='),
                4 => (-1, '-'),
                _ => unreachable!(),
            };
            s.insert(0, c);
            val -= digit;
            val /= 5;
        }
        Self { val: s }
    }
}

type Input = Vec<Snafu>;

fn parse(data: &str) -> nom::IResult<&str, Input> {
    let snafus: Vec<Snafu> = data
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();
    Ok(("", snafus))
}

#[aoc(2022, day25)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> String {
        let sum: i64 = input.iter().map(|snafu| snafu.to_decimal()).sum();
        Snafu::to_snafu(sum).val
    }

    #[solver(part2, gen)]
    pub fn solve_part2(_input: &Input) -> String {
        String::new()
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> String {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> String {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod test {

    const EXAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), "2=-1=0");
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), "");
    }
}

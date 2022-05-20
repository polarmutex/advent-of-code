use advent_of_code_traits::{days::Day2, ParseInput, Solution};

use crate::AdventOfCode2021;

pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl ParseInput<Day2> for AdventOfCode2021 {
    type Parsed = Vec<Instruction>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let action = iter.next().unwrap();
                let steps = iter.next().unwrap().parse().unwrap();
                match action {
                    "forward" => Instruction::Forward(steps),
                    "down" => Instruction::Down(steps),
                    "up" => Instruction::Up(steps),
                    _ => panic!("Unknown line {}", line),
                }
            })
            .collect()
    }
}

impl Solution<Day2> for AdventOfCode2021 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Instruction>) -> Self::Part1Output {
        let (x, y) = input
            .iter()
            .fold((0, 0), |(x, y), instruction| match instruction {
                Instruction::Down(steps) => (x, y + steps),
                Instruction::Up(steps) => (x, y - steps),
                Instruction::Forward(steps) => (x + steps, y),
            });
        x * y
    }

    fn part2(input: &Vec<Instruction>) -> Self::Part2Output {
        let (x, y, _) =
            input
                .iter()
                .fold((0, 0, 0), |(x, y, aim), instruction| match instruction {
                    Instruction::Down(steps) => (x, y, aim + steps),
                    Instruction::Up(steps) => (x, y, aim - steps),
                    Instruction::Forward(steps) => (x + steps, y + aim * steps, aim),
                });
        x * y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn test_part1() {
        let input = <AdventOfCode2021 as ParseInput<Day2>>::parse_input(EXAMPLE);
        assert_eq!(<AdventOfCode2021 as Solution<Day2>>::part1(&input), 150);
    }

    #[test]
    fn test_part2() {
        let input = <AdventOfCode2021 as ParseInput<Day2>>::parse_input(EXAMPLE);
        assert_eq!(<AdventOfCode2021 as Solution<Day2>>::part2(&input), 900);
    }

    #[test]
    fn test_answers() {
        let input_file = "data/2021/day02_github.txt";
        let input_str =
            read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
        let input = <AdventOfCode2021 as ParseInput<Day2>>::parse_input(&input_str);
        assert_eq!(<AdventOfCode2021 as Solution<Day2>>::part1(&input), 1250395);
        assert_eq!(
            <AdventOfCode2021 as Solution<Day2>>::part2(&input),
            1451210346
        );
    }
}

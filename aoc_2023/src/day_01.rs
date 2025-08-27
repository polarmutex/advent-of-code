use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::character::complete::alphanumeric1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;

type Input = Vec<String>;

#[aoc(2023, day1)]
pub mod solutions {
    use super::*;

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        let line_parser = map(alphanumeric1, String::from);
        separated_list1(line_ending, line_parser)(data)
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u32 {
        data.iter()
            .map(|l| process_calibration_line(l, false))
            .sum::<u32>()
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> u32 {
        data.iter()
            .map(|l| process_calibration_line(l, true))
            .sum::<u32>()
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part_1(data)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part_2(data)
    }
}

fn process_calibration_line(line: &String, convert_num_str: bool) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let sub_line = &line[index..];
        let result = if convert_num_str && sub_line.starts_with("one") {
            Some(1)
        } else if convert_num_str && sub_line.starts_with("two") {
            Some(2)
        } else if convert_num_str && sub_line.starts_with("three") {
            Some(3)
        } else if convert_num_str && sub_line.starts_with("four") {
            Some(4)
        } else if convert_num_str && sub_line.starts_with("five") {
            Some(5)
        } else if convert_num_str && sub_line.starts_with("six") {
            Some(6)
        } else if convert_num_str && sub_line.starts_with("seven") {
            Some(7)
        } else if convert_num_str && sub_line.starts_with("eight") {
            Some(8)
        } else if convert_num_str && sub_line.starts_with("nine") {
            Some(9)
        } else {
            sub_line.chars().next().unwrap().to_digit(10)
        };
        result
    });
    let first = it.next().expect("should be a number");
    match it.last() {
        Some(last) => first * 10 + last,
        None => first * 10 + first,
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner_macros::aoc_case;
    use super::solutions::*;

    #[aoc_case(142, 142)]
    const EXAMPLE1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    
    #[test]
    fn part_2_custom() {
        assert_eq!(part_2(EXAMPLE2), 281);
    }

    #[test]
    fn line_tests() {
        assert_eq!(super::process_calibration_line(&"two1nine".to_string(), true), 29);
        assert_eq!(super::process_calibration_line(&"eightwothree".to_string(), true), 83);
        assert_eq!(super::process_calibration_line(&"abcone2threexyz".to_string(), true), 13);
        assert_eq!(super::process_calibration_line(&"xtwone3four".to_string(), true), 24);
        assert_eq!(super::process_calibration_line(&"4nineeightseven2".to_string(), true), 42);
        assert_eq!(super::process_calibration_line(&"zoneight234".to_string(), true), 14);
        assert_eq!(super::process_calibration_line(&"7pqrstsixteen".to_string(), true), 76);
    }
}
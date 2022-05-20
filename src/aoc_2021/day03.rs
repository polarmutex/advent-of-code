use advent_of_code_traits::{days::Day3, ParseInput, Solution};

use crate::AdventOfCode2021;

const BIT_WIDTH: usize = 12;

fn most_common_bits(input: &[String]) -> Vec<bool> {
    let mut col_zeros_count = [0; BIT_WIDTH];
    for line in input {
        for (i, c) in line.chars().enumerate() {
            match c {
                '1' => col_zeros_count[i] -= 1,
                '0' => col_zeros_count[i] += 1,
                _ => (),
            }
        }
    }
    col_zeros_count
        .into_iter()
        .map(|zero_count| if zero_count > 0 { false } else { true })
        .collect()
}

fn partition_by_one(input: &[String], bit_pos: usize) -> (Vec<String>, Vec<String>) {
    input
        .iter()
        .cloned()
        .partition(|line| line.as_bytes()[bit_pos] == b'1')
}

fn most_common_one_fold(input: &[String], bit_pos: usize) -> String {
    if input.len() == 1 {
        return input[0].clone();
    }
    let (ones, zeros) = partition_by_one(input, bit_pos);
    let remains = if ones.len() >= zeros.len() {
        ones
    } else {
        zeros
    };
    most_common_one_fold(&remains, bit_pos + 1)
}

fn least_common_zero_fold(input: &[String], bit_pos: usize) -> String {
    if input.len() == 1 {
        return input[0].clone();
    }
    let (ones, zeros) = partition_by_one(input, bit_pos);
    let remains = if zeros.len() <= ones.len() {
        zeros
    } else {
        ones
    };
    least_common_zero_fold(&remains, bit_pos + 1)
}

impl ParseInput<Day3> for AdventOfCode2021 {
    type Parsed = Vec<String>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.lines().map(|line| line.chars().collect()).collect()
    }
}

impl Solution<Day3> for AdventOfCode2021 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<String>) -> Self::Part1Output {
        let s = most_common_bits(input)
            .into_iter()
            .map(|flag| if flag { '1' } else { '0' })
            .collect::<String>();
        let gamma_rate = u16::from_str_radix(&s, 2).unwrap();
        let epsilon_rate = !gamma_rate & 0x0FFF;
        gamma_rate as u32 * epsilon_rate as u32
    }

    fn part2(input: &Vec<String>) -> Self::Part2Output {
        let oxygen_generator_rating = most_common_one_fold(input, 0);
        let co2_scrubber_rating = least_common_zero_fold(input, 0);
        let oxygen_generator_rating = u16::from_str_radix(&oxygen_generator_rating, 2).unwrap();
        let co2_scrubber_rating = u16::from_str_radix(&co2_scrubber_rating, 2).unwrap();
        oxygen_generator_rating as u32 * co2_scrubber_rating as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const EXAMPLE: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

    #[test]
    fn test_part1() {
        let input = <AdventOfCode2021 as ParseInput<Day3>>::parse_input(EXAMPLE);
        assert_eq!(<AdventOfCode2021 as Solution<Day3>>::part1(&input), 198);
    }

    #[test]
    fn test_part2() {
        let input = <AdventOfCode2021 as ParseInput<Day3>>::parse_input(EXAMPLE);
        assert_eq!(<AdventOfCode2021 as Solution<Day3>>::part2(&input), 230);
    }

    #[test]
    fn test_answers() {
        let input_file = "data/2021/day03_github.txt";
        let input_str =
            read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
        let input = <AdventOfCode2021 as ParseInput<Day3>>::parse_input(&input_str);
        assert_eq!(<AdventOfCode2021 as Solution<Day3>>::part1(&input), 845186);
        assert_eq!(<AdventOfCode2021 as Solution<Day3>>::part2(&input), 4636702);
    }
}

use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::alphanumeric1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::combinator::eof;
use nom::multi::fold_many1;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::Parser;
use nom_supreme::tag::complete::tag;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Maps {
    pub directions: Vec<Direction>,
    pub maps: BTreeMap<String, (String, String)>,
}

fn parse_maps(input: &str) -> nom::IResult<&str, BTreeMap<String, (String, String)>> {
    fold_many1(
        terminated(
            separated_pair(
                alphanumeric1.map(ToString::to_string),
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(
                        alphanumeric1.map(ToString::to_string),
                        tag(", "),
                        alphanumeric1.map(ToString::to_string),
                    ),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<String, (String, String)>, (k, v)| {
            acc.insert(k, v);
            acc
        },
    )
    .parse(input)
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

type Input = Maps;

#[aoc(2023, day8)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (input, directions) = many1(alt((
            complete::char::<&str, nom::error::Error<&str>>('R').map(|_| Direction::Right),
            complete::char::<&str, nom::error::Error<&str>>('L').map(|_| Direction::Left),
        )))
        .parse(input).unwrap();
        let (input, _) = multispace1::<&str, nom::error::Error<&str>>(input).unwrap();
        let (_, maps) = parse_maps(input).unwrap();
        Maps { directions, maps }
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u64 {
        let mut current_node = String::from("AAA");
        data.directions
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(i, direction)| {
                if current_node == *"ZZZ" {
                    return Some(i as u64);
                }
                let node = data.maps.get(&current_node).expect("to get something");
                match direction {
                    Direction::Right => current_node = node.1.clone(),
                    Direction::Left => current_node = node.0.clone(),
                }
                None
            })
            .expect("an answer")
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> u64 {
        // starting ghost positions
        let starting_nodes = data
            .maps
            .iter()
            .filter(|(k, _)| k.ends_with('A'))
            .collect_vec();

        // each ghost "should" cycle to end node, find length of each ghost end cycle
        let cycles = starting_nodes
            .iter()
            .map(|n| {
                let mut current_node = n.0;
                data.directions
                    .iter()
                    .cycle()
                    .enumerate()
                    .find_map(|(i, direction)| {
                        if current_node.ends_with('Z') {
                            return Some(i as u64);
                        }
                        let node = data.maps.get(current_node).expect("to get something");
                        match direction {
                            Direction::Right => current_node = &node.1,
                            Direction::Left => current_node = &node.0,
                        }
                        None
                    })
                    .expect("an answer")
            })
            .collect_vec();

        lcm(&cycles)
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part_1(data)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part_2(data)
    }
}

#[cfg(test)]
mod test {
    use super::solutions::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        RL
        
        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    const EXAMPLE2: &str = indoc! {"
        LLR
        
        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};



    #[test]
    fn test_example1_part1() {
        let data = input_generator(EXAMPLE);
        assert_eq!(2, solve_part_1(data));
    }

    #[test]
    fn test_example2_part1() {
        let data = input_generator(EXAMPLE2);
        assert_eq!(6, solve_part_1(data));
    }

    // Note: EXAMPLE3 part2 test would be:
    // let data = input_generator(EXAMPLE3);
    // assert_eq!(6, solve_part_2(data));
}
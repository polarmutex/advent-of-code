use common::{solution, Answer};
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

solution!("Haunted Wasteland", 8);

type Input = Maps;

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Maps {
    directions: Vec<Direction>,
    maps: BTreeMap<String, (String, String)>,
}

impl Maps {}

#[tracing::instrument(skip(input))]
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

#[tracing::instrument(skip(input))]
fn parse(input: &str) -> nom::IResult<&str, Input> {
    let (input, directions) = many1(alt((
        complete::char('R').map(|_| Direction::Right),
        complete::char('L').map(|_| Direction::Left),
    )))
    .parse(input)?;
    let (input, _) = multispace1(input)?;
    let (input, maps) = parse_maps(input)?;
    Ok((input, Maps { directions, maps }))
}

#[tracing::instrument(skip(input))]
fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    dbg!(&data);
    let mut current_node = String::from("AAA");
    let result = data.directions
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
        .expect("an answer");
        
    Ok(result.into())
}

#[tracing::instrument(skip(input))]
fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    // starting ghost positions
    let starting_nodes = data
        .maps
        .clone()
        .into_iter()
        .filter(|(k, _)| k.ends_with('A'))
        .collect_vec();

    // each ghost "should" cycle to end node, find length of each ghost end cycle
    let cycles = starting_nodes
        .iter()
        .map(|n| {
            let mut current_node = &n.0;
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
    dbg!(&cycles);

    let result = lcm(&cycles);
    
    Ok(result.into())
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

#[cfg(test)]
mod test {
    use common::load_raw;
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

    const EXAMPLE3: &str = indoc! {"
        LR
        
        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 2.into());
        Ok(())
    }

    #[test]
    fn part_1_example2() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE2)?, 6.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE3)?, 6.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 8)?;
        assert_eq!(super::part_1(input.as_str())?, 21409.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 8)?;
        assert_eq!(super::part_2(input.as_str())?, 21165830176709u64.into());
        Ok(())
    }
}

use std::collections::BTreeMap;

use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
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
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    8,
    "\
",
    "data/08.txt"
);

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
fn parse_maps(input: &str) -> IResult<BTreeMap<String, (String, String)>> {
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

impl Solution for Day {
    type Parsed = Maps;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        let (input, directions) = many1(alt((
            complete::char('R').map(|_| Direction::Right),
            complete::char('L').map(|_| Direction::Left),
        )))
        .parse(input)?;
        let (input, _) = multispace1(input)?;
        let (input, maps) = parse_maps(input)?;
        Ok((input, Maps { directions, maps }))
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        dbg!(&data);
        let mut current_node = String::from("AAA");
        data.directions
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(i, direction)| {
                if current_node == String::from("ZZZ") {
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

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
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

        lcm(&cycles)
    }
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

tests! {
     const EXAMPLE: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

     const EXAMPLE2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

     const EXAMPLE3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 2);
    add_test!(part1_example, test_part1_example2, EXAMPLE2 => 6);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 21409);
    add_test!(part2_example, test_part2_example, EXAMPLE3 => 6);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 21165830176709);
}

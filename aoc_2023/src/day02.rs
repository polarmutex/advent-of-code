use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
// use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use std::collections::BTreeMap;
use std::ops::Not;
use std::str::FromStr;

boilerplate!(
    Day,
    2,
    "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
    "data/02.txt"
);

#[derive(Clone, Debug)]
struct Game {
    id: u32,
    rounds: Vec<Vec<Cube>>,
}
impl Game {
    fn valid_game(&self, cubeset: &BTreeMap<CubeColor, u32>) -> Option<u32> {
        self.rounds
            .iter()
            .any(|r| {
                r.iter()
                    .any(|c| c.amount > *cubeset.get(&c.color).expect("correct cube"))
            })
            .not()
            .then_some(self.id)
    }
    fn minimum_cubeset(&self) -> BTreeMap<CubeColor, u32> {
        let cubeset: BTreeMap<CubeColor, u32> = BTreeMap::new();
        self.rounds.iter().fold(cubeset, |mut res, round| {
            for cube in round {
                res.entry(cube.color)
                    .and_modify(|v| *v = (*v).max(cube.amount))
                    .or_insert(cube.amount);
            }
            res
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CubeColor {
    Blue,
    Red,
    Green,
}
impl FromStr for CubeColor {
    type Err = ();
    fn from_str(input: &str) -> Result<CubeColor, Self::Err> {
        match input {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
struct Cube {
    color: CubeColor,
    amount: u32,
}

fn cube_parser(input: &str) -> IResult<Cube> {
    let (input, (amount, color_str)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    let color = CubeColor::from_str(color_str).unwrap();
    Ok((input, Cube { color, amount }))
}

fn round_parser(input: &str) -> IResult<Vec<Cube>> {
    let (input, round) = separated_list1(tag(", "), cube_parser)(input)?;
    Ok((input, round))
}

fn game_parser(input: &str) -> IResult<Game> {
    let (input, id_str) = preceded(tag("Game "), digit1)(input)?;
    let id = id_str.parse::<u32>().expect("id is a number");
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round_parser))(input)?;
    Ok((input, Game { id, rounds }))
}

impl Solution for Day {
    type Parsed = Vec<Game>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, game_parser)(data)
    }

    fn part1(data: Self::Parsed) -> Self::Answer {
        let colorset = BTreeMap::from([
            (CubeColor::Red, 12),
            (CubeColor::Green, 13),
            (CubeColor::Blue, 14),
        ]);
        data.iter()
            .filter_map(|game| game.valid_game(&colorset))
            .sum()
    }

    fn part2(data: Self::Parsed) -> Self::Answer {
        data.iter()
            .map(|game| game.minimum_cubeset().values().product::<u32>())
            .sum()
    }
}

tests! {
     const EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 8);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 2416);
    add_test!(part2_example, test_part2_example, EXAMPLE => 2286);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 63307);
}

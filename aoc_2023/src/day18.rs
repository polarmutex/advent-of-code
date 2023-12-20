use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use glam::I64Vec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::hex_digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::Parser;
use std::collections::HashSet;
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    18,
    "\
",
    "data/18.txt"
);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Instruction {
    direction: I64Vec2,
    count: u64,
    hex_code: String,
}

fn parse_instruction(input: &str) -> IResult<Instruction> {
    let (input, direction) = alt((
        complete::char('U').map(|_| I64Vec2::new(0, -1)),
        complete::char('D').map(|_| I64Vec2::new(0, 1)),
        complete::char('L').map(|_| I64Vec2::new(-1, 0)),
        complete::char('R').map(|_| I64Vec2::new(1, 0)),
    ))
    .parse(input)?;
    let (input, count) = delimited(space1, complete::u64, space1).parse(input)?;
    let (input, hex_code) = delimited(tag("(#"), hex_digit1, tag(")"))
        .map(ToString::to_string)
        .parse(input)?;
    Ok((
        input,
        Instruction {
            direction,
            count,
            hex_code,
        },
    ))
}

impl Solution for Day {
    type Parsed = Vec<Instruction>;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, parse_instruction).parse(input)
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        dbg!(&data);
        let vertices = data
            .iter()
            .scan(I64Vec2::new(0, 0), |res, next| {
                *res += next.direction * (next.count as i64);
                Some(*res)
            })
            .collect::<Vec<I64Vec2>>();
        dbg!(&vertices);
        let perimeter_len = vertices
            .iter()
            .tuple_windows()
            .map(|(a, b)| {
                let d = (*a - *b).abs();
                d.x + d.y
            })
            .sum::<i64>()
            + {
                let last = vertices.iter().last().unwrap();
                let first = vertices.first().unwrap();
                let d = (*first - *last).abs();
                d.x + d.y
            };
        dbg!(&perimeter_len);
        let area = ((vertices
            .iter()
            .tuple_windows()
            .map(|(a, b)| a.x * b.y - a.y * b.x)
            .sum::<i64>()
            + perimeter_len)
            / 2)
        .abs()
            + 1;
        dbg!(&area);
        area as u64
    }

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
        dbg!(&data);
        let vertices = data
            .iter()
            .map(|i| {
                let corrected_distance =
                    u64::from_str_radix(&i.hex_code[0..5], 16).expect("a number");
                let corrected_direction = match i.hex_code[5..].into() {
                    "0" => I64Vec2::new(1, 0),
                    "1" => I64Vec2::new(0, 1),
                    "2" => I64Vec2::new(-1, 0),
                    "3" => I64Vec2::new(0, -1),
                    _ => panic!("illegal value"),
                };
                Instruction {
                    direction: corrected_direction,
                    count: corrected_distance,
                    hex_code: i.hex_code.clone(),
                }
            })
            .scan(I64Vec2::new(0, 0), |res, next| {
                *res += next.direction * (next.count as i64);
                Some(*res)
            })
            .collect::<Vec<I64Vec2>>();
        dbg!(&vertices);
        let perimeter_len = vertices
            .iter()
            .tuple_windows()
            .map(|(a, b)| {
                let d = (*a - *b).abs();
                d.x + d.y
            })
            .sum::<i64>()
            + {
                let last = vertices.iter().last().unwrap();
                let first = vertices.first().unwrap();
                let d = (*first - *last).abs();
                d.x + d.y
            };
        dbg!(&perimeter_len);
        let area = ((vertices
            .iter()
            .tuple_windows()
            .map(|(a, b)| a.x * b.y - a.y * b.x)
            .sum::<i64>()
            + perimeter_len)
            / 2)
        .abs()
            + 1;
        dbg!(&area);
        area as u64
    }
}

#[allow(dead_code)]
fn print(d: &HashSet<I64Vec2>, size: &I64Vec2) {
    for y in 0..size.y {
        for x in 0..size.x {
            let pt = I64Vec2::new(x, y);
            match d.get(&pt) {
                Some(_) => {
                    print!("#");
                }
                None => print!("."),
            }
        }
        println!();
    }
}

tests! {
     const EXAMPLE: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 62);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 48400);
    add_test!(part2_example, test_part2_example, EXAMPLE => 952408144115);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 72811019847283);
}

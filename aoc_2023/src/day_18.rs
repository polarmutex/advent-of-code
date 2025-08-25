use common::{solution, Answer};
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

solution!("Lavaduct Lagoon", 18);

type Input = Vec<Instruction>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Instruction {
    direction: I64Vec2,
    count: u64,
    hex_code: String,
}

fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
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

fn parse(data: &str) -> nom::IResult<&str, Input> {
    separated_list1(line_ending, parse_instruction).parse(data)
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let vertices = data
        .iter()
        .scan(I64Vec2::new(0, 0), |res, next| {
            *res += next.direction * (next.count as i64);
            Some(*res)
        })
        .collect::<Vec<I64Vec2>>();
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
    let area = ((vertices
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x)
        .sum::<i64>()
        + perimeter_len)
        / 2)
    .abs()
        + 1;
    Ok((area as u64).into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
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
    let area = ((vertices
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x)
        .sum::<i64>()
        + perimeter_len)
        / 2)
    .abs()
        + 1;
    Ok((area as u64).into())
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

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;
    use super::*;

    #[test]
    fn part_1_example() -> miette::Result<()> {
        let input = indoc! {"
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
        "};
        assert_eq!(super::part_1(input)?, 62.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        let input = indoc! {"
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
        "};
        assert_eq!(super::part_2(input)?, 952408144115_u64.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 18)?;
        assert_eq!(super::part_1(input.as_str())?, 48400.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 18)?;
        assert_eq!(super::part_2(input.as_str())?, 72811019847283_u64.into());
        Ok(())
    }
}

use aoc_runner_macros::{aoc, generator, solver, solution};
use glam::IVec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::take_till1;
use nom::character::complete::digit1;
use nom::combinator::iterator;
use nom::IResult as IBaseResult;
use nom::Parser; //allows .map on nom
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::collections::HashSet;

type Input = Vec<Value>;

type Span<'a> = LocatedSpan<&'a str>;
// type SpanIVec2 = LocatedSpan<String, IVec2>;

#[derive(Clone, Debug, PartialEq)]
pub struct SpanWithLoc {
    id: usize,
    fragment: String,
    pos: IVec2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Empty,
    Symbol(SpanWithLoc),
    Number(SpanWithLoc),
}

fn with_xy(span: Span) -> SpanWithLoc {
    //col/location are 1 indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    // span.map_extra(|_| IVec2::new(x, y))
    SpanWithLoc {
        id: span.location_offset(),
        fragment: span.fragment().to_string(),
        pos: IVec2::new(x, y),
    }
}

fn parse_grid(input: Span) -> nom::IResult<Span, Vec<Value>> {
    let mut it = iterator(
        input,
        alt((
            digit1.map(|span| with_xy(span)).map(Value::Number),
            is_not(".\n0123456789")
                .map(|span| with_xy(span))
                .map(Value::Symbol),
            take_till1(|c: char| c.is_ascii_digit() || c != '.' && c != '\n').map(|_| Value::Empty),
        )),
    );

    let parsed = it
        .filter(|value| value != &Value::Empty)
        .collect::<Vec<Value>>();
    let res: IBaseResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

#[aoc(2023, day3)]
pub mod solutions {
    use super::*;

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        let potential_parts = parse_grid(Span::new(data)).unwrap().1;
        Ok(("", potential_parts))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u32 {
        let symbols = data
            .iter()
            .filter_map(|v| match v {
                Value::Empty => None,
                Value::Number(_) => None,
                Value::Symbol(sym) => Some(sym.pos),
            })
            .collect::<HashSet<IVec2>>();
            
        data.iter()
            .filter_map(|v| {
                // return None if non Number
                let Value::Number(num) = v else {
                    return None;
                };

                let surrounding = [IVec2::new(-1, 0), IVec2::new(num.fragment.len() as i32, 0)]
                    .into_iter()
                    .chain((-1..=num.fragment.len() as i32).map(|x| IVec2::new(x, 1)))
                    .chain((-1..=num.fragment.len() as i32).map(|x| IVec2::new(x, -1)))
                    .map(|pos| pos + num.pos)
                    .collect::<Vec<IVec2>>();

                surrounding
                    .iter()
                    .any(|pos| symbols.contains(pos))
                    .then_some(num.fragment.parse::<u32>().expect("should be number"))
            })
            .sum::<u32>()
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> u32 {
        let numbers = data
            .iter()
            .filter_map(|v| match v {
                Value::Empty => None,
                Value::Number(v) => Some(v),
                Value::Symbol(_) => None,
            })
            .flat_map(|v| {
                (v.pos.x..(v.pos.x + v.fragment.len() as i32)).map(move |x| {
                    (
                        IVec2::new(x, v.pos.y),
                        (v.id, v.fragment.clone().parse::<u32>().expect("a number")),
                    )
                })
            })
            .collect::<HashMap<IVec2, (usize, u32)>>();
            
        data.iter()
            .filter_map(|v| {
                let Value::Symbol(sym) = v else {
                    return None;
                };
                if sym.fragment != "*" {
                    return None;
                }

                let matching = [
                    IVec2::new(1, 0),
                    IVec2::new(1, -1),
                    IVec2::new(0, -1),
                    IVec2::new(-1, -1),
                    IVec2::new(-1, 0),
                    IVec2::new(-1, 1),
                    IVec2::new(0, 1),
                    IVec2::new(1, 1),
                ]
                .iter()
                .map(|p| sym.pos + *p)
                .filter_map(|p| numbers.get(&p))
                .unique()
                .map(|(_, n)| *n)
                .collect::<Vec<u32>>();
                (matching.len() == 2).then_some(matching.iter().product::<u32>())
            })
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

#[cfg(test)]
mod tests {
    use aoc_runner_macros::aoc_case;
    

    #[aoc_case(4361, 467835)]
    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
}

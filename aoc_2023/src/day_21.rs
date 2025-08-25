use common::{solution, Answer};
use glam::IVec2;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::opt;
use nom::multi::fold_many1;
use nom::sequence::terminated;
use nom::IResult as IBaseResult;
use nom::Parser;
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::collections::HashSet;
// use itertools::Itertools;
// use nom::character::complete;
// use nom_supreme::ParserExt;
// use tracing::info;

solution!("Step Counter", 21);

type Input = (IVec2, HashSet<IVec2>);

#[allow(dead_code)]
type HeatLossMap = HashMap<IVec2, u32>;

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
struct Map {
    grid: HeatLossMap,
    size: IVec2,
}

type Span<'a> = LocatedSpan<&'a str>;
#[derive(Clone, Debug, PartialEq)]
struct SpanWithLoc {
    id: usize,
    fragment: String,
    pos: IVec2,
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

fn parse_grid(input: Span) -> IBaseResult<Span, (HashSet<IVec2>, IVec2)> {
    fold_many1(
        terminated(
            alt((tag("."), tag("S"))).map(with_xy),
            opt(alt((line_ending, is_a("#")))),
        ),
        || (HashSet::new(), IVec2::splat(0)),
        |(mut set, mut start), next| {
            if next.fragment == *"S" {
                start = next.pos;
            }
            set.insert(next.pos);
            (set, start)
        },
    )
    .parse(input)
}

fn num_steps(start: &IVec2, set: &HashSet<IVec2>, step_count: usize) -> u64 {
    let bounds = IVec2::new(131, 131);
    let mut starting_hashset = HashSet::new();
    starting_hashset.insert(*start);
    std::iter::successors(Some(starting_hashset), |occupied_positions| {
        let mut new_set: HashSet<IVec2> = HashSet::new();

        for pos in occupied_positions.iter() {
            [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
                .into_iter()
                .filter_map(|offset| {
                    let cell = offset + *pos;
                    // set.contains(&cell).then_some(cell)
                    set.contains(&(cell.rem_euclid(bounds))).then_some(cell)
                })
                .for_each(|pos| {
                    new_set.insert(pos);
                });
        }
        Some(new_set)
    })
    .nth(step_count)
    .unwrap()
    .len() as u64
}

fn parse_input(data: &str) -> miette::Result<Input> {
    let (grid, start) = parse_grid(Span::new(data)).map_err(|e| miette::miette!("Parse error: {}", e))?.1;
    Ok((start, grid))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (start, set) = parse_input(input)?;
    let result = num_steps(&start, &set, 64);
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (start, set) = parse_input(input)?;
    let len = 131_usize;

    let x = num_steps(&start, &set, 65);
    let y = num_steps(&start, &set, 65 + len);
    let z = num_steps(&start, &set, 65 + len * 2);

    let goal = 26501365_u64;
    let n = goal / len as u64;
    let result = quad(n, x, y, z);
    Ok(result.into())
}

fn quad(n: u64, a0: u64, a1: u64, a2: u64) -> u64 {
    let b0 = a0;
    let b1 = a1 - a0;
    let b2 = a2 - a1;
    b0 + b1 * n + (n * (n - 1) / 2) * (b2 - b1)
}

#[allow(dead_code)]
fn print(d: &HashSet<IVec2>, size: &IVec2) {
    for y in 0..size.y {
        for x in 0..size.x {
            let pt = IVec2::new(x, y);
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
    use super::*;

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 21)?;
        assert_eq!(super::part_1(input.as_str())?, 3666.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 21)?;
        assert_eq!(super::part_2(input.as_str())?, 609298746763952_u64.into());
        Ok(())
    }
}

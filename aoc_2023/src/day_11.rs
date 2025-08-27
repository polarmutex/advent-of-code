use aoc_runner_macros::{aoc, generator, solver, solution};
use glam::IVec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::iterator;
use nom::IResult as IBaseResult;
use nom::Parser;
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc(2023, day11)]
pub mod solutions {
    use super::*;

type Input = HashMap<IVec2, SpaceType>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpaceType {
    Galaxy,
    Space,
    NewLine,
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

fn parse_grid(input: Span) -> nom::IResult<Span, HashMap<IVec2, SpaceType>> {
    let mut it = iterator(
        input,
        alt((
            tag("#")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, SpaceType::Galaxy)),
            tag(".")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, SpaceType::Space)),
            tag("\n")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, SpaceType::NewLine)),
        )),
    );

    let parsed = it
        .filter(|value| match value.1 {
            SpaceType::Galaxy => true,
            SpaceType::Space => false,
            SpaceType::NewLine => false,
        })
        .collect();
    let res: IBaseResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

fn solve<const EXPANSION: usize>(data: HashMap<IVec2, SpaceType>) -> u64 {
    let width = data
        .iter()
        .fold(0, |res, v| if v.0.x > res { v.0.x } else { res });
    let cols: HashSet<i32> = (0..width).collect();
    let height = data
        .iter()
        .fold(0, |res, v| if v.0.y > res { v.0.y } else { res });
    let rows: HashSet<i32> = (0..height).collect();

    let non_empty_rows: &Vec<i32> = &data
        .iter()
        .fold(rows, |mut acc, (pos, _)| {
            acc.remove(&pos.y);
            acc
        })
        .iter()
        .copied()
        .sorted()
        .collect_vec();
    dbg!(non_empty_rows);
    let non_empty_cols: &Vec<i32> = &data
        .iter()
        .fold(cols, |mut acc, (pos, _)| {
            acc.remove(&pos.x);
            acc
        })
        .iter()
        .copied()
        .sorted()
        .collect_vec();
    dbg!(non_empty_cols);

    let galaxies = data
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let col_exp = (non_empty_cols
                .iter()
                .copied()
                .filter(|p| p < &v.0.x)
                .count()
                * (EXPANSION - 1)) as i32;
            let row_exp = (non_empty_rows
                .iter()
                .copied()
                .filter(|p| p < &v.0.y)
                .count()
                * (EXPANSION - 1)) as i32;
            dbg!(&v.0);
            dbg!(col_exp);
            dbg!(row_exp);
            (i + 1, *v.0 + IVec2::new(col_exp, row_exp))
        })
        .collect_vec();

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            // dbg!(&pair);
            let diff = (pair[0].1 - pair[1].1).abs();
            (diff.x + diff.y).unsigned_abs() as u64
        })
        .sum()
}

    fn parse(input: &str) -> nom::IResult<&str, Input> {
        let grid = parse_grid(Span::new(input)).unwrap().1;
        Ok(("", grid))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u64 {
        solve::<2>(data)
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> u64 {
        solve::<1_000_000>(data)
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
mod tests {
    use aoc_runner_macros::aoc_case;
    

    #[aoc_case(374, 82000210)]
    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
}

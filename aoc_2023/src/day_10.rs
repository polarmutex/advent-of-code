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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    Ground,
    NewLine,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
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

fn parse_grid(input: Span) -> nom::IResult<Span, HashMap<IVec2, PipeType>> {
    let mut it = iterator(
        input,
        alt((
            tag("S")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::Start)),
            tag("|")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::Vertical)),
            tag("-")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::Horizontal)),
            tag("L")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::NorthEast)),
            tag("J")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::NorthWest)),
            tag("7")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::SouthWest)),
            tag("F")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::SouthEast)),
            tag(".")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::Ground)),
            tag("\n")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::NewLine)),
        )),
    );

    let parsed = it.filter(|value| value.1 != PipeType::NewLine).collect();
    let res: IBaseResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

fn find_loop(data: &HashMap<IVec2, PipeType>) -> Vec<IVec2> {
    let start_position = data
        .iter()
        .find_map(|v| {
            if v.1 == &PipeType::Start {
                Some(v.0)
            } else {
                None
            }
        })
        .expect("to find start");
    let north_start = *start_position + IVec2::new(0, -1);
    let north = data
        .get(&north_start)
        .is_some_and(|v| {
            matches!(
                v,
                PipeType::Vertical | PipeType::SouthWest | PipeType::SouthEast
            )
        })
        .then_some((Direction::South, north_start));
    let south_start = *start_position + IVec2::new(0, 1);
    let south = data
        .get(&south_start)
        .is_some_and(|v| {
            matches!(
                v,
                PipeType::Vertical | PipeType::NorthWest | PipeType::NorthEast
            )
        })
        .then_some((Direction::North, south_start));
    let east_start = *start_position + IVec2::new(1, 0);
    let east = data
        .get(&east_start)
        .is_some_and(|v| {
            matches!(
                v,
                PipeType::Horizontal | PipeType::NorthWest | PipeType::SouthWest
            )
        })
        .then_some((Direction::West, east_start));
    let west_start = *start_position + IVec2::new(-1, 0);
    let west = data
        .get(&west_start)
        .is_some_and(|v| {
            matches!(
                v,
                PipeType::Horizontal | PipeType::NorthEast | PipeType::SouthEast
            )
        })
        .then_some((Direction::East, west_start));
    let mut it = [north, south, east, west].into_iter().flatten().map(|v| {
        std::iter::successors(Some(v), |(direction, cur_pos)| {
            if cur_pos == start_position {
                Some((Direction::North, *cur_pos))
            } else {
                let cur_type = data.get(cur_pos).expect("pos");
                let next_direction = match (direction, cur_type) {
                    (Direction::North, PipeType::Vertical) => Direction::North,
                    (Direction::North, PipeType::NorthEast) => Direction::West,
                    (Direction::North, PipeType::NorthWest) => Direction::East,
                    (Direction::South, PipeType::Vertical) => Direction::South,
                    (Direction::South, PipeType::SouthEast) => Direction::West,
                    (Direction::South, PipeType::SouthWest) => Direction::East,
                    (Direction::East, PipeType::Horizontal) => Direction::East,
                    (Direction::East, PipeType::NorthEast) => Direction::South,
                    (Direction::East, PipeType::SouthEast) => Direction::North,
                    (Direction::West, PipeType::Horizontal) => Direction::West,
                    (Direction::West, PipeType::NorthWest) => Direction::South,
                    (Direction::West, PipeType::SouthWest) => Direction::North,
                    value => unreachable!("should be unreachable {:?}", value),
                };
                Some(match next_direction {
                    Direction::North => (Direction::North, *cur_pos + IVec2::new(0, 1)),
                    Direction::South => (Direction::South, *cur_pos + IVec2::new(0, -1)),
                    Direction::East => (Direction::East, *cur_pos + IVec2::new(-1, 0)),
                    Direction::West => (Direction::West, *cur_pos + IVec2::new(1, 0)),
                })
            }
        })
    });
    let path1 = it.next().expect("at least one");
    let mut res = vec![];
    res.extend(
        path1
            .take_while(|v| v.1 != *start_position)
            .map(|v| v.1)
            .collect_vec(),
    );
    res.insert(0, *start_position);
    res
}

type Input = HashMap<IVec2, PipeType>;

#[aoc(2023, day10)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        parse_grid(Span::new(input)).unwrap().1
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u32 {
        (find_loop(&data).len() / 2) as u32
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> u32 {
        let coords = find_loop(&data);

        // shoelace formula
        let area = (coords
            .iter()
            .copied()
            .chain([coords[0]])
            .tuple_windows()
            .map(|(a, b)| {
                isize::try_from(a.x * b.y).expect("") - isize::try_from(b.x * a.y).expect("")
            })
            .sum::<isize>()
            .unsigned_abs()
            / 2) as u32;
        // Pick's theorem: A = i + b/2 - 1
        area + 1 - coords.len() as u32 / 2
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
mod test {
    use super::solutions::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};
    
    const EXAMPLE2: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};







    #[test]
    fn test_example1_part1() {
        let data = input_generator(EXAMPLE);
        assert_eq!(4, solve_part_1(data));
    }

    #[test]
    fn test_example2_part1() {
        let data = input_generator(EXAMPLE2);
        assert_eq!(8, solve_part_1(data));
    }

    // Part 2 only tests - keeping as comments for reference:
    // EXAMPLE3 expects part2=4
    // EXAMPLE4 expects part2=8  
    // EXAMPLE5 expects part2=10
}
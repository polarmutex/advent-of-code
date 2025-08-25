use common::{solution, Answer};
use glam::IVec2;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::iterator;
use nom::IResult as IBaseResult;
use nom::Parser;
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::fmt::Display;
// use nom_supreme::ParserExt;
// use itertools::Itertools;
// use tracing::info;

solution!("Parabolic Reflector Dish", 14);

type Input = RockMap;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    North,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum RockType {
    Rounded,
    Cube,
    Empty,
    NewLine,
}
impl Display for RockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RockType::Rounded => "O",
                RockType::Cube => "#",
                _ => "",
            }
        )
    }
}

type RockState = HashMap<IVec2, RockType>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct RockMap {
    grid: RockState,
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

fn parse_grid(input: Span) -> IBaseResult<Span, RockState> {
    let mut it = iterator(
        input,
        alt((
            tag("O")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, RockType::Rounded)),
            tag("#")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, RockType::Cube)),
            tag(".")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, RockType::Empty)),
            tag("\n")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, RockType::NewLine)),
        )),
    );

    let parsed = it
        .filter(|value| match value.1 {
            RockType::Rounded => true,
            RockType::Cube => true,
            RockType::Empty => true,
            RockType::NewLine => false,
        })
        .collect();
    let res: IBaseResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

fn parse_input(data: &str) -> miette::Result<Input> {
    let grid = parse_grid(Span::new(data)).map_err(|e| miette::miette!("Parse error: {}", e))?.1;
    let size = IVec2::new(
        grid.iter()
            .fold(0, |acc, (pos, _)| if pos.x > acc { pos.x } else { acc })
            .abs() as i32
            + 1,
        grid.iter()
            .fold(0, |acc, (pos, _)| if pos.y > acc { pos.y } else { acc })
            .abs() as i32
            + 1,
    );
    Ok(RockMap {
        size,
        grid: grid
            .into_iter()
            .filter(|(_, t)| *t != RockType::Empty)
            .collect::<HashMap<IVec2, RockType>>(),
    })
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let data = parse_input(input)?;
    let dir = [Direction::North].iter();
    let final_map = dir.fold(data.grid, |old_map, dir| match dir {
        Direction::North => tilt_north(&old_map, &data.size),
    });
    let result: u32 = final_map
        .iter()
        .filter_map(|(p, t)| match t {
            RockType::Rounded => Some((data.size.x - p.y) as u32),
            _ => None,
        })
        .sum::<u32>();
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let data = parse_input(input)?;
    let (spins, cycle_len, mut state) = find_cycle(&data.grid, &data.size);
    let spins_left = (1_000_000_000 - spins) % cycle_len;
    for _ in 0..spins_left {
        state = spin(&state, &data.size);
    }
    let result: u32 = state
        .iter()
        .filter_map(|(p, t)| match t {
            RockType::Rounded => Some((data.size.x - p.y) as u32),
            _ => None,
        })
        .sum::<u32>();
    Ok(result.into())
}

fn find_cycle(d: &RockState, s: &IVec2) -> (usize, usize, RockState) {
    let mut cache = HashMap::new();
    let mut i = 0;
    let mut state = d.clone();
    loop {
        i += 1;
        state = spin(&state, s);
        if let Some(start_cycle) = cache.insert(grid_as_str(&state, s), i) {
            return (i, i - start_cycle, state);
        };
    }
}

fn spin(d: &RockState, s: &IVec2) -> RockState {
    let mut state = d.clone();
    state = tilt_north(&state, s);
    state = tilt_west(&state, s);
    state = tilt_south(&state, s);
    state = tilt_east(&state, s);
    state
}

fn tilt_north(d: &RockState, s: &IVec2) -> RockState {
    let mut next_state = d
        .clone()
        .into_iter()
        .filter(|(_, t)| *t == RockType::Cube)
        .collect::<HashMap<IVec2, RockType>>();
    for x in 0..s.x {
        let mut next_pos = IVec2::new(x, 0);
        for y in 0..s.y {
            match d.get(&IVec2::new(x, y)) {
                Some(RockType::Cube) => {
                    next_pos = IVec2::new(x, y + 1);
                }
                Some(RockType::Rounded) => {
                    next_state.insert(next_pos, RockType::Rounded);
                    next_pos.y += 1;
                }
                _ => {}
            };
        }
    }
    next_state
}

fn tilt_south(d: &RockState, s: &IVec2) -> RockState {
    let mut next_state = d
        .clone()
        .into_iter()
        .filter(|(_, t)| *t == RockType::Cube)
        .collect::<HashMap<IVec2, RockType>>();
    for x in 0..s.x {
        let mut next_pos = IVec2::new(x, s.y - 1);
        for y in (0..s.y).rev() {
            match d.get(&IVec2::new(x, y)) {
                Some(RockType::Cube) => {
                    next_pos = IVec2::new(x, y - 1);
                }
                Some(RockType::Rounded) => {
                    next_state.insert(next_pos, RockType::Rounded);
                    next_pos.y -= 1;
                }
                _ => {}
            };
        }
    }
    next_state
}

fn tilt_west(d: &RockState, s: &IVec2) -> RockState {
    let mut next_state = d
        .clone()
        .into_iter()
        .filter(|(_, t)| *t == RockType::Cube)
        .collect::<HashMap<IVec2, RockType>>();
    for y in 0..s.y {
        let mut next_pos = IVec2::new(0, y);
        for x in 0..s.x {
            match d.get(&IVec2::new(x, y)) {
                Some(RockType::Cube) => {
                    next_pos = IVec2::new(x + 1, y);
                }
                Some(RockType::Rounded) => {
                    next_state.insert(next_pos, RockType::Rounded);
                    next_pos.x += 1;
                }
                _ => {}
            };
        }
    }
    next_state
}

fn tilt_east(d: &RockState, s: &IVec2) -> RockState {
    let mut next_state = d
        .clone()
        .into_iter()
        .filter(|(_, t)| *t == RockType::Cube)
        .collect::<HashMap<IVec2, RockType>>();
    for y in 0..s.y {
        let mut next_pos = IVec2::new(s.x - 1, y);
        for x in (0..s.x).rev() {
            match d.get(&IVec2::new(x, y)) {
                Some(RockType::Cube) => {
                    next_pos = IVec2::new(x - 1, y);
                }
                Some(RockType::Rounded) => {
                    next_state.insert(next_pos, RockType::Rounded);
                    next_pos.x -= 1;
                }
                _ => {}
            };
        }
    }
    next_state
}

#[allow(dead_code)]
fn print(d: &RockState, size: &IVec2) {
    for y in 0..size.y {
        for x in 0..size.x {
            match d.get(&IVec2::new(x, y)) {
                Some(rock) => {
                    print!("{rock}");
                }
                None => print!("."),
            }
        }
        println!();
    }
}

fn grid_as_str(d: &RockState, size: &IVec2) -> String {
    (0..size.y)
        .flat_map(|y| {
            (0..size.x).map(move |x| match d.get(&IVec2::new(x, y)) {
                Some(RockType::Rounded) => "O",
                Some(RockType::Cube) => "#",
                None => ".",
                _ => "",
            })
        })
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;
    use super::*;

    #[test]
    fn part_1_example() -> miette::Result<()> {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};
        assert_eq!(super::part_1(input)?, 136.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};
        assert_eq!(super::part_2(input)?, 64.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 14)?;
        assert_eq!(super::part_1(input.as_str())?, 107053.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 14)?;
        assert_eq!(super::part_2(input.as_str())?, 88371.into());
        Ok(())
    }
}

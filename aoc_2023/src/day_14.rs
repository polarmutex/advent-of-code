use aoc_runner_macros::{aoc, generator, solver, solution};
use glam::IVec2;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::iterator;
use nom::IResult as IBaseResult;
use nom::Parser;
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::fmt::Display;

type Input = RockMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    North,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RockType {
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

pub type RockState = HashMap<IVec2, RockType>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RockMap {
    grid: RockState,
    size: IVec2,
}

#[aoc(2023, day14)]
pub mod solutions {
    use super::*;

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

    fn parse_input(data: &str) -> Input {
        let grid = parse_grid(Span::new(data)).unwrap().1;
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
        RockMap {
            size,
            grid: grid
                .into_iter()
                .filter(|(_, t)| *t != RockType::Empty)
                .collect::<HashMap<IVec2, RockType>>(),
        }
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        parse_input(input)
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u32 {
        let dir = [Direction::North].iter();
        let final_map = dir.fold(data.grid, |old_map, dir| match dir {
            Direction::North => tilt_north(&old_map, &data.size),
        });
        final_map
            .iter()
            .filter_map(|(p, t)| match t {
                RockType::Rounded => Some((data.size.x - p.y) as u32),
                _ => None,
            })
            .sum::<u32>()
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> u32 {
        let (spins, cycle_len, mut state) = find_cycle(&data.grid, &data.size);
        let spins_left = (1_000_000_000 - spins) % cycle_len;
        for _ in 0..spins_left {
            state = spin(&state, &data.size);
        }
        state
            .iter()
            .filter_map(|(p, t)| match t {
                RockType::Rounded => Some((data.size.x - p.y) as u32),
                _ => None,
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
mod tests {
    use aoc_runner_macros::aoc_case;
    

    #[aoc_case(136, 64)]
    const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
}

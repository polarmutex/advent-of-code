use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use glam::IVec2;
// use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
// use nom::character::complete;
use nom::combinator::iterator;
use nom::IResult as IBaseResult;
use nom::Parser;
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::fmt::Display;
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    14,
    "\
",
    "data/14.txt"
);
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

impl Solution for Day {
    type Parsed = RockMap;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        let grid = parse_grid(Span::new(input)).unwrap().1;
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
        Ok((
            "",
            RockMap {
                size,
                grid: grid
                    .into_iter()
                    .filter(|(_, t)| *t != RockType::Empty)
                    .collect::<HashMap<IVec2, RockType>>(),
            },
        ))
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        dbg!(print(&data.grid, &data.size));
        let dir = [Direction::North].iter();
        let final_map = dir.fold(data.grid, |old_map, dir| match dir {
            Direction::North => tilt_north(&old_map, &data.size),
            _ => old_map,
        });
        dbg!(print(&final_map, &data.size));
        final_map
            .iter()
            .filter_map(|(p, t)| match t {
                RockType::Rounded => Some((data.size.x - p.y) as u32),
                _ => None,
            })
            .sum::<u32>()
    }

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
        let (spins, cycle_len, mut state) = find_cycle(&data.grid, &data.size);
        let spins_left = (1_000_000_000 - spins) % cycle_len;
        for _ in 0..spins_left {
            state = spin(&state, &data.size);
        }
        dbg!(spins);
        dbg!(cycle_len);
        // dbg!(print(&data.grid, &data.size));
        // let mut dir = [
        //     Direction::North,
        //     Direction::West,
        //     Direction::South,
        //     Direction::East,
        // ]
        // .iter()
        // .cycle();
        // let (final_map, _) = (0..1_000_000_000).into_iter().fold(
        //     (data.grid, HashMap::<(&Direction, String), RockState>::new()),
        //     |(old_map, mut cache), i| {
        //         // dbg!(print(&old_map, &data.size));
        //         // dbg!(grid_as_str(&old_map, &data.size));
        //         let dir = dir.next().expect("a direction");
        //         let next_state = match cache.get(&(dir, grid_as_str(&old_map, &data.size))) {
        //             Some(cache_state) => {
        //                 dbg!(i);
        //                 std::process::exit(1);
        //                 cache_state.clone()
        //             }
        //             None => {
        //                 let ns = match dir {
        //                     Direction::North => tilt_north(&old_map, &data.size),
        //                     Direction::West => tilt_west(&old_map, &data.size),
        //                     Direction::South => tilt_south(&old_map, &data.size),
        //                     Direction::East => tilt_east(&old_map, &data.size),
        //                 };
        //                 cache.insert((dir, grid_as_str(&old_map, &data.size)), ns.clone());
        //                 ns
        //             }
        //         };
        //         (next_state, cache)
        //     },
        // );
        // // dbg!(print(&final_map, &data.size));
        state
            .iter()
            .filter_map(|(p, t)| match t {
                RockType::Rounded => Some((data.size.x - p.y) as u32),
                _ => None,
            })
            .sum::<u32>()
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
    state = tilt_north(&state, &s);
    state = tilt_west(&state, &s);
    state = tilt_south(&state, &s);
    state = tilt_east(&state, &s);
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
        println!("");
    }
}

fn grid_as_str(d: &RockState, size: &IVec2) -> String {
    (0..size.y)
        .into_iter()
        .flat_map(|y| {
            (0..size.x)
                .into_iter()
                .map(move |x| match d.get(&IVec2::new(x, y)) {
                    Some(RockType::Rounded) => "O",
                    Some(RockType::Cube) => "#",
                    None => ".",
                    _ => "",
                })
        })
        .collect::<String>()
}

tests! {
     const EXAMPLE: &str = "\
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
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 136);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 107053);
    add_test!(part2_example, test_part2_example, EXAMPLE => 64);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 88371);
}

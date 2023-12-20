use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use glam::IVec2;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::IResult as IBaseResult;
use nom::Parser;
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
// use itertools::Itertools;
// use nom::character::complete;
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    16,
    "\
",
    "data/16.txt"
);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn next(&self, pos: &IVec2) -> IVec2 {
        *pos + match self {
            Direction::North => IVec2::new(0, -1),
            Direction::South => IVec2::new(0, 1),
            Direction::East => IVec2::new(1, 0),
            Direction::West => IVec2::new(-1, 0),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum MirrorType {
    Empty,
    ReflectedForward,
    ReflectedBackward,
    SplitterVertical,
    SplitterHorizontal,
}
impl Display for MirrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MirrorType::Empty => ".",
                MirrorType::ReflectedForward => "/",
                MirrorType::ReflectedBackward => r#"\"#,
                MirrorType::SplitterVertical => "|",
                MirrorType::SplitterHorizontal => "-",
            }
        )
    }
}

type MirrorMap = HashMap<IVec2, MirrorType>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Input {
    grid: MirrorMap,
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

fn parse_grid(input: Span) -> IBaseResult<Span, MirrorMap> {
    let (input, grid) = many1(delimited(
        opt(is_a("\n")),
        alt((
            tag("/")
                .map(with_xy)
                .map(|span| (span.pos, MirrorType::ReflectedForward {})),
            tag(r#"\"#)
                .map(with_xy)
                .map(|span| (span.pos, MirrorType::ReflectedBackward {})),
            tag("|")
                .map(with_xy)
                .map(|span| (span.pos, MirrorType::SplitterVertical {})),
            tag("-")
                .map(with_xy)
                .map(|span| (span.pos, MirrorType::SplitterHorizontal {})),
            tag(".")
                .map(with_xy)
                .map(|span| (span.pos, MirrorType::Empty {})),
        )),
        opt(is_a("\n")),
    ))
    .parse(input)?;
    Ok((input, grid.iter().cloned().collect::<MirrorMap>()))
}

impl Solution for Day {
    type Parsed = Input;
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
        Ok(("", Input { size, grid }))
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        dbg!(&data);
        let start = (Direction::East, IVec2::new(-1, 0));
        num_energized(&data, start)
    }

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
        (0..data.size.y)
            .map(|y| (Direction::East, IVec2::new(-1, y)))
            .chain((0..data.size.y).map(|y| (Direction::West, IVec2::new(data.size.x, y))))
            .chain((0..data.size.x).map(|x| (Direction::South, IVec2::new(x, -1))))
            .chain((0..data.size.x).map(|x| (Direction::North, IVec2::new(x, data.size.y))))
            .map(|start| num_energized(&data, start))
            .max()
            .expect("to have a max")
    }
}

fn num_energized(data: &Input, start: (Direction, IVec2)) -> u32 {
    let mut active_beams: Vec<(Direction, IVec2)> = vec![start];
    let mut all: HashSet<(Direction, IVec2)> = HashSet::new();
    let mut energized_cells: HashSet<IVec2> = HashSet::new();
    // dbg!(print(&data.grid, &data.size, &energized_cells));
    loop {
        if active_beams.is_empty() {
            break;
        }
        let mut next_beams: Vec<(Direction, IVec2)> = vec![];
        for (cur_dir, cur_pos) in active_beams.iter() {
            let next_pos = cur_dir.next(cur_pos);
            let next_mirror = data.grid.get(&next_pos);
            // dbg!(&cur_pos, &cur_dir, &next_pos, &next_mirror);
            energized_cells.insert(*cur_pos);

            match (cur_dir, next_mirror) {
                (_, None) => {
                    // outside grid, do nothing
                }
                (dir, Some(m)) => match *m {
                    MirrorType::Empty => {
                        if !all.contains(&(cur_dir.clone(), next_pos)) {
                            next_beams.push((cur_dir.clone(), next_pos));
                            all.insert((cur_dir.clone(), next_pos));
                        }
                    }
                    MirrorType::ReflectedForward => match dir {
                        Direction::North => {
                            if !all.contains(&(Direction::East, next_pos)) {
                                next_beams.push((Direction::East, next_pos));
                                all.insert((Direction::East, next_pos));
                            }
                        }
                        Direction::West => {
                            if !all.contains(&(Direction::South, next_pos)) {
                                next_beams.push((Direction::South, next_pos));
                                all.insert((Direction::South, next_pos));
                            }
                        }
                        Direction::South => {
                            if !all.contains(&(Direction::West, next_pos)) {
                                next_beams.push((Direction::West, next_pos));
                                all.insert((Direction::West, next_pos));
                            }
                        }
                        Direction::East => {
                            if !all.contains(&(Direction::North, next_pos)) {
                                next_beams.push((Direction::North, next_pos));
                                all.insert((Direction::North, next_pos));
                            }
                        }
                    },
                    MirrorType::ReflectedBackward => match dir {
                        Direction::North => {
                            if !all.contains(&(Direction::West, next_pos)) {
                                next_beams.push((Direction::West, next_pos));
                                all.insert((Direction::West, next_pos));
                            }
                        }
                        Direction::East => {
                            if !all.contains(&(Direction::South, next_pos)) {
                                next_beams.push((Direction::South, next_pos));
                                all.insert((Direction::South, next_pos));
                            }
                        }
                        Direction::South => {
                            if !all.contains(&(Direction::East, next_pos)) {
                                next_beams.push((Direction::East, next_pos));
                                all.insert((Direction::East, next_pos));
                            }
                        }
                        Direction::West => {
                            if !all.contains(&(Direction::North, next_pos)) {
                                next_beams.push((Direction::North, next_pos));
                                all.insert((Direction::North, next_pos));
                            }
                        }
                    },
                    MirrorType::SplitterVertical => match dir {
                        Direction::North => {
                            if !all.contains(&(Direction::North, next_pos)) {
                                next_beams.push((Direction::North, next_pos));
                                all.insert((Direction::North, next_pos));
                            }
                        }
                        Direction::West => {
                            if !all.contains(&(Direction::North, next_pos)) {
                                next_beams.push((Direction::North, next_pos));
                                all.insert((Direction::North, next_pos));
                            }
                            if !all.contains(&(Direction::South, next_pos)) {
                                next_beams.push((Direction::South, next_pos));
                                all.insert((Direction::South, next_pos));
                            }
                        }
                        Direction::South => {
                            if !all.contains(&(Direction::South, next_pos)) {
                                next_beams.push((Direction::South, next_pos));
                                all.insert((Direction::South, next_pos));
                            }
                        }
                        Direction::East => {
                            if !all.contains(&(Direction::North, next_pos)) {
                                next_beams.push((Direction::North, next_pos));
                                all.insert((Direction::North, next_pos));
                            }
                            if !all.contains(&(Direction::South, next_pos)) {
                                next_beams.push((Direction::South, next_pos));
                                all.insert((Direction::South, next_pos));
                            }
                        }
                    },
                    MirrorType::SplitterHorizontal => match dir {
                        Direction::North => {
                            if !all.contains(&(Direction::East, next_pos)) {
                                next_beams.push((Direction::East, next_pos));
                                all.insert((Direction::East, next_pos));
                            }
                            if !all.contains(&(Direction::West, next_pos)) {
                                next_beams.push((Direction::West, next_pos));
                                all.insert((Direction::West, next_pos));
                            }
                        }
                        Direction::West => {
                            if !all.contains(&(Direction::West, next_pos)) {
                                next_beams.push((Direction::West, next_pos));
                                all.insert((Direction::West, next_pos));
                            }
                        }
                        Direction::South => {
                            if !all.contains(&(Direction::East, next_pos)) {
                                next_beams.push((Direction::East, next_pos));
                                all.insert((Direction::East, next_pos));
                            }
                            if !all.contains(&(Direction::West, next_pos)) {
                                next_beams.push((Direction::West, next_pos));
                                all.insert((Direction::West, next_pos));
                            }
                        }
                        Direction::East => {
                            if !all.contains(&(Direction::East, next_pos)) {
                                next_beams.push((Direction::East, next_pos));
                                all.insert((Direction::East, next_pos));
                            }
                        }
                    },
                },
            }
        }
        active_beams = next_beams;
    }
    // dbg!(print(&data.grid, &data.size, &energized_cells));
    energized_cells.len() as u32 - 1 // 1 for initial point
}

#[allow(dead_code)]
fn print(d: &MirrorMap, size: &IVec2, e: &HashSet<IVec2>) {
    for y in 0..size.y {
        for x in 0..size.x {
            let pt = IVec2::new(x, y);
            match d.get(&pt) {
                Some(mirror) => {
                    if e.contains(&pt) {
                        print!("#");
                    } else {
                        print!("{mirror}");
                    }
                }
                None => print!("x"),
            }
        }
        println!();
    }
}

tests! {
     const EXAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    add_test!(part1_example, test_part1_example, EXAMPLE => 46);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 7074);
    add_test!(part2_example, test_part2_example, EXAMPLE => 51);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 7530);
}

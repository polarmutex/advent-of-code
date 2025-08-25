use common::{solution, Answer};
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

solution!("The Floor Will Be Lava", 16);

type Input = MirrorInput;

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
struct MirrorInput {
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
    Ok(MirrorInput { size, grid })
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let data = parse_input(input)?;
    let start = (Direction::East, IVec2::new(-1, 0));
    let result = num_energized(&data, start);
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let data = parse_input(input)?;
    let result = (0..data.size.y)
        .map(|y| (Direction::East, IVec2::new(-1, y)))
        .chain((0..data.size.y).map(|y| (Direction::West, IVec2::new(data.size.x, y))))
        .chain((0..data.size.x).map(|x| (Direction::South, IVec2::new(x, -1))))
        .chain((0..data.size.x).map(|x| (Direction::North, IVec2::new(x, data.size.y))))
        .map(|start| num_energized(&data, start))
        .max()
        .expect("to have a max");
    Ok(result.into())
}

fn num_energized(data: &MirrorInput, start: (Direction, IVec2)) -> u32 {
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

#[cfg(test)]
mod test {
    use common::load_raw;
    use super::*;

    #[test]
    fn part_1_example() -> miette::Result<()> {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(super::part_1(input)?, 46.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(super::part_2(input)?, 51.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 16)?;
        assert_eq!(super::part_1(input.as_str())?, 7074.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 16)?;
        assert_eq!(super::part_2(input.as_str())?, 7530.into());
        Ok(())
    }
}

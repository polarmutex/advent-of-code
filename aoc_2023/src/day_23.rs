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
use petgraph::{
    algo,
    prelude::*,
};
use std::collections::HashMap;
use std::fmt::Display;
// use nom_supreme::ParserExt;
// use itertools::Itertools;
// use tracing::info;

solution!("A Long Walk", 23);

type Input = TrailInput;

#[derive(Clone, Debug)]
struct Trail {
    r#type: TrailType,
    pos: IVec2,
}
impl Display for Trail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.r#type {
            TrailType::Slope(Direction::North) => "^",
            TrailType::Slope(Direction::South) => "v",
            TrailType::Slope(Direction::East) => ">",
            TrailType::Slope(Direction::West) => "<",
            TrailType::Flat => ".",
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum TrailType {
    Slope(Direction),
    Flat,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn step(&self, position: &IVec2) -> IVec2 {
        *position
            + match self {
                Direction::North => IVec2::new(0, -1),
                Direction::South => IVec2::new(0, 1),
                Direction::East => IVec2::new(1, 0),
                Direction::West => IVec2::new(-1, 0),
            }
    }
}

// type RockState = HashMap<IVec2, RockType>;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct TrailInput {
    map: HashMap<IVec2, Trail>,
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

fn parse_grid(input: Span) -> IBaseResult<Span, Vec<Trail>> {
    many1(delimited(
        opt(is_a("#\n")),
        alt((
            tag("^").map(|span| Trail {
                r#type: TrailType::Slope(Direction::North),
                pos: with_xy(span).pos,
            }),
            tag(">").map(|span| Trail {
                r#type: TrailType::Slope(Direction::East),
                pos: with_xy(span).pos,
            }),
            tag("v").map(|span| {
                let _located = with_xy(span);
                Trail {
                    r#type: TrailType::Slope(Direction::South),
                    pos: with_xy(span).pos,
                }
            }),
            tag("<").map(|span| {
                let _located = with_xy(span);
                Trail {
                    r#type: TrailType::Slope(Direction::West),
                    pos: with_xy(span).pos,
                }
            }),
            tag(".").map(|span| {
                let _located = with_xy(span);
                Trail {
                    r#type: TrailType::Flat,
                    pos: with_xy(span).pos,
                }
            }),
        )),
        opt(is_a("#\n")),
    ))
    .parse(input)
}

fn parse_input(data: &str) -> miette::Result<Input> {
    let map = parse_grid(Span::new(data)).map_err(|e| miette::miette!("Parse error: {}", e))?.1;
    let size = IVec2::new(
        map.iter()
            .fold(0, |acc, v| if v.pos.x > acc { v.pos.x } else { acc })
            .abs() as i32
            + 1,
        map.iter()
            .fold(0, |acc, v| if v.pos.y > acc { v.pos.y } else { acc })
            .abs() as i32
            + 1,
    );
    Ok(TrailInput {
        size,
        map: map
            .into_iter()
            .map(|v| (v.pos, v))
            .collect::<HashMap<IVec2, Trail>>(),
    })
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let data = parse_input(input)?;
        let start = *data.map.iter().min_by_key(|(_k, v)| v.pos.y).unwrap().0;
        let end = *data.map.iter().max_by_key(|(_k, v)| v.pos.y).unwrap().0;
        let mut graph = DiGraph::<&Trail, u32>::new();
        let node_map: HashMap<IVec2, NodeIndex> = data
            .map
            .iter()
            .map(|(k, v)| (*k, graph.add_node(&v)))
            .collect();
        data
            .map
            .iter()
            .flat_map(|(k, v)| {
                let possible_directions = match v.r#type {
                    TrailType::Slope(direction) => {
                        vec![direction]
                    }
                    TrailType::Flat => vec![
                        Direction::North,
                        Direction::South,
                        Direction::East,
                        Direction::West,
                    ],
                };
                possible_directions.into_iter().filter_map(|dir| {
                    let next_pos = dir.step(k);
                    let can_move_here = data.map.get(&next_pos).is_some();
                    can_move_here.then(|| (node_map[k], node_map[&next_pos], 1))
                })
            })
            .for_each(|(a, b, w)| {
                graph.add_edge(a, b, w);
            });
        let ways =
            algo::all_simple_paths::<Vec<_>, _>(&graph, node_map[&start], node_map[&end], 0, None)
                .max_by(|a, b| a.len().cmp(&b.len()))
                .unwrap();
        Ok(((ways.len() - 1) as u32).into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let data = parse_input(input)?;
        /*timized with edge contraction. For nodes with only two neighbours you can remove it
        (i.e a corridor in the graph) and connect those two nodes instead. This reduces my input
        graph from 9412 nodes to only 36 for in part 2! Now it runs in about 550ms.*/
        let start = *data.map.iter().min_by_key(|(_k, v)| v.pos.y).unwrap().0;
        let end = *data.map.iter().max_by_key(|(_k, v)| v.pos.y).unwrap().0;
        let mut graph = DiGraph::<&Trail, u32>::new();
        let node_map: HashMap<IVec2, NodeIndex> = data
            .map
            .iter()
            .map(|(k, v)| (*k, graph.add_node(&v)))
            .collect();
        data
            .map
            .iter()
            .flat_map(|(k, _v)| {
                let possible_directions = vec![
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::West,
                ];
                possible_directions.into_iter().filter_map(|dir| {
                    let next_pos = dir.step(k);
                    let can_move_here = data.map.get(&next_pos).is_some();
                    can_move_here.then(|| (node_map[k], node_map[&next_pos], 1))
                })
            })
            .for_each(|(a, b, w)| {
                graph.add_edge(a, b, w);
            });
        let ways =
            algo::all_simple_paths::<Vec<_>, _>(&graph, node_map[&start], node_map[&end], 0, None)
                .max_by(|a, b| a.len().cmp(&b.len()))
                .unwrap();
        Ok(((ways.len() - 1) as u32).into())
}

#[allow(dead_code)]
fn print(d: &TrailInput, size: &IVec2) {
    for y in 0..size.y {
        for x in 0..size.x {
            match d.map.get(&IVec2::new(x, y)) {
                Some(rock) => {
                    print!("{rock}");
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

    const EXAMPLE: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 94.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 154.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 23)?;
        assert_eq!(super::part_1(input.as_str())?, 2330.into());
        Ok(())
    }

    // Part 2 is commented out due to long runtime
    // #[test]
    // #[ignore]
    // fn part_2() -> miette::Result<()> {
    //     let input = load_raw(2023, 23)?;
    //     assert_eq!(super::part_2(input.as_str())?, 88371.into());
    //     Ok(())
    // }
}

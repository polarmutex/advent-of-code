use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use glam::IVec2;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::combinator::iterator;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::IResult as IBaseResult;
use nom::Parser;
use nom_locate::LocatedSpan;
use petgraph::{
    algo,
    data::FromElements,
    dot::{Config, Dot},
    prelude::*,
};
use std::collections::HashMap;
use std::fmt::Display;
// use nom_supreme::ParserExt;
// use itertools::Itertools;
// use tracing::info;

boilerplate!(
    Day,
    23,
    "\
",
    "data/23.txt"
);

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
struct Input {
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
                let located = with_xy(span);
                Trail {
                    r#type: TrailType::Slope(Direction::South),
                    pos: with_xy(span).pos,
                }
            }),
            tag("<").map(|span| {
                let located = with_xy(span);
                Trail {
                    r#type: TrailType::Slope(Direction::West),
                    pos: with_xy(span).pos,
                }
            }),
            tag(".").map(|span| {
                let located = with_xy(span);
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

impl Solution for Day {
    type Parsed = Input;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        let map = parse_grid(Span::new(input)).unwrap().1;
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
        Ok((
            "",
            Input {
                size,
                map: map
                    .into_iter()
                    .map(|v| (v.pos, v))
                    .collect::<HashMap<IVec2, Trail>>(),
            },
        ))
    }

    #[tracing::instrument(skip(input))]
    fn part1(input: Self::Parsed) -> Self::Answer {
        dbg!(&input);
        let start = *input.map.iter().min_by_key(|(k, v)| v.pos.y).unwrap().0;
        let end = *input.map.iter().max_by_key(|(k, v)| v.pos.y).unwrap().0;
        let mut graph = DiGraph::<&Trail, u32>::new();
        let node_map: HashMap<IVec2, NodeIndex> = input
            .map
            .iter()
            .map(|(k, v)| (*k, graph.add_node(&v)))
            .collect();
        input
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
                    let can_move_here = input.map.get(&next_pos).is_some();
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
        (ways.len() - 1) as u32
    }

    #[tracing::instrument(skip(input))]
    fn part2(input: Self::Parsed) -> Self::Answer {
        /*timized with edge contraction. For nodes with only two neighbours you can remove it
        (i.e a corridor in the graph) and connect those two nodes instead. This reduces my input
        graph from 9412 nodes to only 36 for in part 2! Now it runs in about 550ms.*/
        let start = *input.map.iter().min_by_key(|(k, v)| v.pos.y).unwrap().0;
        let end = *input.map.iter().max_by_key(|(k, v)| v.pos.y).unwrap().0;
        let mut graph = DiGraph::<&Trail, u32>::new();
        let node_map: HashMap<IVec2, NodeIndex> = input
            .map
            .iter()
            .map(|(k, v)| (*k, graph.add_node(&v)))
            .collect();
        input
            .map
            .iter()
            .flat_map(|(k, v)| {
                let possible_directions = vec![
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::West,
                ];
                possible_directions.into_iter().filter_map(|dir| {
                    let next_pos = dir.step(k);
                    let can_move_here = input.map.get(&next_pos).is_some();
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
        (ways.len() - 1) as u32
    }
}

fn print(d: &Input, size: &IVec2) {
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

tests! {
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

    add_test!(part1_example, test_part1_example, EXAMPLE => 94);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 2330);
    add_test!(part2_example, test_part2_example, EXAMPLE => 154);
    // add_test!(part2, test_part2_input, Day::INPUT_DATA => 88371);
}

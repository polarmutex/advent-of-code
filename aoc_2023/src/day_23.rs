use aoc_runner_macros::{aoc, generator, solver, solution};
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

type Input = TrailInput;

#[derive(Clone, Debug)]
pub struct Trail {
    r#type: TrailType,
    pos: IVec2,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TrailType {
    Slope(Direction),
    Flat,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TrailInput {
    map: HashMap<IVec2, Trail>,
    size: IVec2,
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

#[aoc(2023, day23)]
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

    fn parse_input(data: &str) -> Input {
        let map = parse_grid(Span::new(data)).unwrap().1;
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
        TrailInput {
            size,
            map: map
                .into_iter()
                .map(|v| (v.pos, v))
                .collect::<HashMap<IVec2, Trail>>(),
        }
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        parse_input(input)
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u32 {
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
        (ways.len() - 1) as u32
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> u32 {
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
        (ways.len() - 1) as u32
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
mod tests {
    use aoc_runner_macros::aoc_case;
    

    #[aoc_case(94, 154)]
    const EXAMPLE: &str = "#.#####################
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
#####################.#";
}

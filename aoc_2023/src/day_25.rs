use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};
use petgraph::prelude::*;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use std::collections::HashMap;

#[aoc(2023, day25)]
pub mod solutions {
    use super::*;

type Input = Vec<(String, Vec<String>)>;

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        separated_list1(
            line_ending,
            separated_pair(
                alpha1.map(ToString::to_string),
                tag(": "),
                separated_list1(space1, alpha1.map(ToString::to_string)),
            ),
        )
        .parse(data)
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> usize {
        let mut graph = UnGraph::<&str, u32>::default();
        let all_nodes = data
            .iter()
            .flat_map(|(k, v)| {
                let mut vs = v.clone();
                vs.push(k.clone());
                vs
            })
            .unique()
            .collect::<Vec<String>>();
        let node_map: HashMap<String, NodeIndex> = all_nodes
            .iter()
            .map(|node| (node.clone(), graph.add_node(&node)))
            .collect();
        for (key, values) in data.iter() {
            for node in values {
                graph.add_edge(node_map[key], node_map[node], 1);
            }
        }
        let min: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
            stoer_wagner_min_cut(&graph, |_| Ok(1));
        let (_cut_size, nodes_in_partition) = min.unwrap().unwrap();
        let total_nodes = all_nodes.len();
        (total_nodes - nodes_in_partition.len()) * nodes_in_partition.len()
    }

    #[solver(part2, main)]
    pub fn solve_part_2(_data: Input) -> u32 {
        // Day 25 typically doesn't have a part 2 in Advent of Code
        0
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> usize {
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
mod tests {
    use aoc_runner_macros::aoc_case;
    

    #[aoc_case(54, 0)]
    const EXAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
}

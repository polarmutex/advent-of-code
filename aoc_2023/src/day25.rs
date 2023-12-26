use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
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
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    25,
    "\
",
    "data/25.txt"
);

impl Solution for Day {
    type Parsed = Vec<(String, Vec<String>)>;
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        separated_list1(
            line_ending,
            separated_pair(
                alpha1.map(ToString::to_string),
                tag(": "),
                separated_list1(space1, alpha1.map(ToString::to_string)),
            ),
        )
        .parse(input)
    }

    #[tracing::instrument(skip(input))]
    fn part1(input: Self::Parsed) -> Self::Answer {
        dbg!(&input);
        let mut graph = UnGraph::<&str, u32>::default();
        let all_nodes = input
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
        for (key, values) in input.iter() {
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

    #[tracing::instrument(skip(_input))]
    fn part2(_input: Self::Parsed) -> Self::Answer {
        todo!()
    }
}

tests! {
     const EXAMPLE: &str = "\
jqt: rhn xhk nvd
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
frs: qnr lhk lsr
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 54);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 613870);
    // add_test!(part2_example, test_part2_example, EXAMPLE => 47);
    // add_test!(part2, test_part2_input, Day::INPUT_DATA => 920630818300104);
}

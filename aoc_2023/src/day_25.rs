use common::{solution, Answer};
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

solution!("Snowverload", 25);

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

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
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
    let result = (total_nodes - nodes_in_partition.len()) * nodes_in_partition.len();
    Ok(result.into())
}

fn part_2(_input: &str) -> miette::Result<Answer> {
    // Day 25 typically doesn't have a part 2 in Advent of Code
    Ok(0.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;
    use super::*;

    #[test]
    fn part_1_example() -> miette::Result<()> {
        let input = indoc! {"
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
        "};
        assert_eq!(super::part_1(input)?, 54.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 25)?;
        assert_eq!(super::part_1(input.as_str())?, 613870.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        let input = "dummy input";
        assert_eq!(super::part_2(input)?, 0.into());
        Ok(())
    }
}

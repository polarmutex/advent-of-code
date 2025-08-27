use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::IResult;
use ndarray::Array2;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
};
use petgraph::algo::condensation;
use petgraph::{graphmap::GraphMap, Undirected};


#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Grid {
    data: Vec<Option<u8>>,
    rows: usize,
    cols: usize,
}

fn row(input: &str) -> IResult<&str, Vec<Option<u8>>> {
    let (input, chars) = many1(one_of("0123456789"))(input)?;
    let nums = [None]
        .into_iter()
        .chain(
            chars
                .iter()
                .map(|v| Some(v.to_digit(10).expect("to have succeeded") as u8)),
        )
        .chain([None])
        .collect::<Vec<Option<u8>>>();

    Ok((input, nums))
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
struct Node {
    point: (usize, usize),
    weight: u8,
}

fn insert(
    graph: &mut GraphMap<Node, (), Undirected>,
    heightmap: &Array2<Option<u8>>,
    point: (usize, usize),
    node: &Node,
) {
    let top = heightmap.get(point);
    if let Some(Some(weight)) = top {
        let node_top = Node {
            point,
            weight: *weight,
        };
        graph.add_node(node_top);
        graph.add_edge(*node, node_top, ());
    };
}

fn parse(input: &str) -> IResult<&str, Grid> {
        let (input, outputs) = separated_list1(newline, row)(input)?;
        // dbg!(&outputs);
        let rows = outputs.len();
        let cols = outputs[0].len();

        let v = vec![None; cols];
        let data = v
            .iter()
            .cloned()
            .chain(outputs.into_iter().flatten())
            .chain(v.iter().cloned())
            .collect::<Vec<Option<u8>>>();

        Ok((input, Grid { data, rows, cols }))
    }

#[aoc(2021, day9)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Grid {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Grid) -> u64 {
        let arr = Array2::from_shape_vec((input.rows + 2, input.cols), input.data.clone()).unwrap();
        let results: u32 = arr
            .windows((3, 3))
            .into_iter()
            .filter_map(|points| {
                let top = points[(0, 1)];
                let left = points[(1, 0)];
                let right = points[(1, 2)];
                let bottom = points[(2, 1)];
                let point = points[(1, 1)];
                match [top, left, right, bottom]
                    .iter()
                    .filter(|v| v.is_some())
                    .all(|&v| v > point)
                {
                    true => {
                        point.map(|v| (v + 1) as u32)
                    }
                    false => None,
                }
            })
            .sum();
        results as u64
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Grid) -> u64 {
        let data: Vec<Option<u8>> = input
            .data
            .iter()
            .map(|v| match v {
                Some(val) => match val {
                    9 => None,
                    _ => Some(*val),
                },
                None => None,
            })
            .collect();

        let arr = Array2::from_shape_vec((input.rows + 2, input.cols), data).unwrap();
        let mut graph: GraphMap<Node, (), Undirected> = GraphMap::new();

        for (point, value) in arr.indexed_iter() {
            if let Some(value) = value {
                let node = Node {
                    point,
                    weight: *value,
                };
                graph.add_node(node);
                insert(&mut graph, &arr, (point.0, point.1 + 1), &node);
                insert(&mut graph, &arr, (point.0 + 1, point.1), &node);
                insert(&mut graph, &arr, (point.0 - 1, point.1), &node);
                insert(&mut graph, &arr, (point.0, point.1 - 1), &node);
            }
        }
        let condensed_graph = condensation::<Node, (), Undirected, u32>(graph.into_graph(), false);
        let mut sums = condensed_graph
            .node_weights()
            .map(|basin| basin.len())
            .collect::<Vec<usize>>();
        sums.sort();
        sums.reverse();
        let mut finals = sums.iter();
        (finals.next().unwrap() * finals.next().unwrap() * finals.next().unwrap()) as u64
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_part_1() {
        assert_eq!(solutions::part_1(EXAMPLE), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solutions::part_2(EXAMPLE), 1134);
    }
}

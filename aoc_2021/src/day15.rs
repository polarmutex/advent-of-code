use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::IResult;
use ndarray::{concatenate, Array2, Axis};
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
};
use petgraph::{algo::dijkstra, graphmap::GraphMap, Undirected};

type Input = Array2<u8>;

fn row(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, chars) = many1(one_of("0123456789"))(input)?;
    let nums = chars
        .iter()
        .map(|v| v.to_digit(10).expect("to have succeeded") as u8)
        .collect::<Vec<u8>>();

    Ok((input, nums))
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
struct Node {
    point: (usize, usize),
    weight: u8,
}

#[aoc(2021, day15)]
pub mod solutions {
    use super::*;

fn insert(
    graph: &mut GraphMap<Node, u32, Undirected>,
    heightmap: &Array2<u8>,
    point: (usize, usize),
    node: &Node,
) -> () {
    let top = heightmap.get(point);
    if let Some(weight) = top {
        let node_top = Node {
            point: point,
            weight: *weight,
        };
        graph.add_node(node_top);
        graph.add_edge(node.clone(), node_top, *weight as u32);
    };
}

fn algo(array: Array2<u8>) -> u32 {
    let mut graph: GraphMap<Node, u32, Undirected> = GraphMap::new();
    for (point, value) in array.indexed_iter() {
        let node = Node {
            point: point,
            weight: *value,
        };
        graph.add_node(node);
        insert(&mut graph, &array, (point.0, point.1 + 1), &node);
        if point.0 != 0 {
            insert(&mut graph, &array, (point.0 - 1, point.1), &node);
        };
    }
    // dbg!(graph);
    let mut it = array.indexed_iter();
    let next_it = it.next().unwrap();
    let start = Node {
        point: next_it.0,
        weight: *next_it.1,
    };
    let last_it = it.last().unwrap();
    let end = Node {
        point: last_it.0,
        weight: *last_it.1,
    };
    let result = dijkstra(&graph, start, Some(end), |edge| edge.1.weight as u32);

    *result.get(&end).unwrap()
}

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, outputs) = separated_list1(newline, row)(input).unwrap();
        let nrows = outputs.len();
        let ncols = outputs[0].len();

        let data = outputs.into_iter().flatten().collect::<Vec<u8>>();

        let arr = Array2::from_shape_vec((nrows, ncols), data).unwrap();
        arr
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u32 {
        algo(input.clone())
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u32 {
        let row_1_arrs = (0..5_u8)
            .map(|i| {
                input.mapv(|weight| {
                    let m = (weight + i) % 10;

                    let res = match m {
                        0 => 1,
                        n => n,
                    };
                    if weight + i > 10 {
                        res + 1
                    } else {
                        res
                    }
                })
            })
            .collect::<Vec<_>>();
        let row_1_views: Vec<_> = row_1_arrs.iter().map(|v| v.view()).collect();
        let row_1 = concatenate(Axis(1), &row_1_views[..]).unwrap();

        let col_arrs = (0..5_u8)
            .map(|i| {
                row_1.mapv(|weight| {
                    let m = (weight + i) % 10;

                    let res = match m {
                        0 => 1,
                        n => n,
                    };
                    if weight + i > 10 {
                        res + 1
                    } else {
                        res
                    }
                })
            })
            .collect::<Vec<_>>();
        let col_views: Vec<_> = col_arrs.iter().map(|v| v.view()).collect();
        let full_grid = concatenate(Axis(0), &col_views[..]);

        algo(full_grid.unwrap())
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod tests {


    const EXAMPLE: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn test_part_1() {
        let _input = super::solutions::input_generator(EXAMPLE);
    }

    #[test]
    fn test_part_2() {
        let _input = super::solutions::input_generator(EXAMPLE);
    }
}

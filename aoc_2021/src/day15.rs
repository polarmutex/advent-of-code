use common::{solution, Answer};
use nom::IResult;
use ndarray::{concatenate, Array2, Axis};
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
};
use petgraph::{algo::dijkstra, graphmap::GraphMap, Undirected};

solution!("Chiton", 15);

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

fn parse(input: &str) -> IResult<&str, Array2<u8>> {
        let (input, outputs) = separated_list1(newline, row)(input)?;
        let nrows = outputs.len();
        let ncols = outputs[0].len();

        let data = outputs.into_iter().flatten().collect::<Vec<u8>>();

        let arr = Array2::from_shape_vec((nrows, ncols), data).unwrap();
        Ok((input, arr))
    }

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
        Ok((algo(input) as u64).into())
    }

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
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

        Ok((algo(full_grid.unwrap()) as u64).into())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_1(EXAMPLE).unwrap(), Answer::Number(40));
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_2(EXAMPLE).unwrap(), Answer::Number(315));
    }
}

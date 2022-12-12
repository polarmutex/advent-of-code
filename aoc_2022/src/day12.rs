use crate::prelude::*;
use framework::algorithms::dijkstra;
use framework::grid::Grid;

day!(12, parse => part1, part2);

struct Input {
    grid: Grid<char>,
    start: Coord2d<usize>,
    end: Coord2d<usize>,
}

fn parse(input: &str) -> ParseResult<Input> {
    let vec = input
        .lines()
        .flat_map(|row| row.chars().collect_vec())
        .collect_vec();
    let width = input.lines().next().expect("at least one row").len();

    let mut grid = Grid { vec, width };
    let start = (0..grid.width())
        .flat_map(|x| (0..grid.height()).map(move |y| Coord2d::from((x, y))))
        .filter(|&pos| grid[pos] == 'S')
        .collect_vec();
    assert!(start.len() == 1);
    let start = start[0];

    let end = (0..grid.width())
        .flat_map(|x| (0..grid.height()).map(move |y| Coord2d::from((x, y))))
        .filter(|&pos| grid[pos] == 'E')
        .collect_vec();
    assert!(end.len() == 1);
    let end = end[0];

    *grid.get_mut(start).expect("start node is valid") = 'a';
    *grid.get_mut(end).expect("end node is valid") = 'z';

    let input = Input { grid, start, end };
    Ok(input)
}

fn part1(input: &Input) -> usize {
    for (pos, val) in input.grid.iter() {
        print!("{}", val);
        if pos.x == input.grid.width() - 1 {
            println!()
        }
    }

    let answer = dijkstra(
        [input.start],
        |&node| node,
        |&node| node == input.end,
        move |node| {
            println!("pt {} {}", node.x, node.y);
            let char_val = *input.grid.get(node).unwrap();
            input.grid.neighbors_plus(node).filter(move |pt| {
                let n_val = *input.grid.get(*pt).unwrap();
                let result = (n_val as u32) <= ((char_val as u32) + 1);
                if result {
                    println!("adding {} ({} {})", n_val, pt.x, pt.y);
                }
                result
            })
        },
    )
    .expect("to find shortest path");
    answer.0
}

fn part2(input: &Input) -> usize {
    let a_nodes = (0..input.grid.width())
        .flat_map(|x| (0..input.grid.height()).map(move |y| Coord2d::from((x, y))))
        .filter(|&pos| input.grid[pos] == 'a')
        .map(Coord2d::from)
        .collect_vec();

    let answer = dijkstra(
        a_nodes,
        |&node| node,
        |&node| node == input.end,
        move |node| {
            println!("pt {} {}", node.x, node.y);
            let char_val = *input.grid.get(node).unwrap();
            input.grid.neighbors_plus(node).filter(move |pt| {
                let n_val = *input.grid.get(*pt).unwrap();
                let result = (n_val as u32) <= ((char_val as u32) + 1);
                if result {
                    println!("adding {} ({} {})", n_val, pt.x, pt.y);
                }
                result
            })
        },
    )
    .expect("to find shortest path");
    answer.0
}

tests! {
    const EXAMPLE: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
    const INPUT: &str = include_str!("data/12.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 31);
    simple_tests!(parse, part1, part1_input_test, INPUT => 449);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 29);
    simple_tests!(parse, part2, part2_input_test, INPUT => 443);
}

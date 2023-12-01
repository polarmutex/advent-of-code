use framework::algorithms::dijkstra;
use framework::boilerplate;
use framework::grid::Grid;
use framework::vec::Coord2d;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;

boilerplate!(
    Day,
    12,
    "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
",
    "data/12.txt"
);

#[derive(Clone, Debug)]
struct Input {
    grid: Grid<char>,
    start: Coord2d,
    end: Coord2d,
}

impl Solution for Day {
    type Parsed = Input;
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 31;
    const ANSWER_1: Self::Answer = 449;
    const EXAMPLE_ANSWER_2: Self::Answer = 29;
    const ANSWER_2: Self::Answer = 443;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let vec = input
            .lines()
            .flat_map(|row| row.chars().collect_vec())
            .collect_vec();
        let width = input.lines().next().expect("at least one row").len() as u32;

        let mut grid = Grid { vec, width };
        let start = (0..grid.width())
            .flat_map(|x| (0..grid.height()).map(move |y| Coord2d::from_coords(x as i32, y as i32)))
            .filter(|&pos| grid[pos] == 'S')
            .collect_vec();
        assert!(start.len() == 1);
        let start = start[0];

        let end = (0..grid.width())
            .flat_map(|x| (0..grid.height()).map(move |y| Coord2d::from_coords(x as i32, y as i32)))
            .filter(|&pos| grid[pos] == 'E')
            .collect_vec();
        assert!(end.len() == 1);
        let end = end[0];

        *grid.get_mut(start).expect("start node is valid") = 'a';
        *grid.get_mut(end).expect("end node is valid") = 'z';

        let input = Input { grid, start, end };
        Ok(("", input))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let input = &input;
        for (pos, val) in input.grid.iter() {
            print!("{}", val);
            if pos.x == (input.grid.width() - 1) as i32 {
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

    fn part2(input: Self::Parsed) -> Self::Answer {
        let input = &input;
        let a_nodes = (0..input.grid.width())
            .flat_map(|x| {
                (0..input.grid.height()).map(move |y| Coord2d::from_coords(x as i32, y as i32))
            })
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
}

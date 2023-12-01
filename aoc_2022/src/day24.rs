use framework::algorithms::a_star;
use framework::boilerplate;
use framework::grid::Grid;
use framework::vec::Coord2d;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use std::slice::Iter;

boilerplate!(
    Day,
    24,
    "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
",
    "data/24.txt"
);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Blizzard {
    North,
    South,
    East,
    West,
    None,
}
impl Blizzard {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Blizzard::North => (0, -1),
            Blizzard::South => (0, 1),
            Blizzard::East => (1, 0),
            Blizzard::West => (-1, 0),
            Blizzard::None => (0, 0),
        }
    }
    pub fn iter() -> Iter<'static, Blizzard> {
        static BLIZZARDS: [Blizzard; 4] = [
            Blizzard::North,
            Blizzard::South,
            Blizzard::West,
            Blizzard::East,
        ];
        BLIZZARDS.iter()
    }
}

impl std::fmt::Display for Blizzard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Blizzard::North => write!(f, "^"),
            Blizzard::South => write!(f, "v"),
            Blizzard::East => write!(f, ">"),
            Blizzard::West => write!(f, "<"),
            Blizzard::None => write!(f, "."),
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Node {
    pos: Coord2d,
    steps: i32,
}

fn solve(input: &Grid<Blizzard>, start: &Node, end: &Node) -> usize {
    println!("steps: {}", start.steps);
    println!("start: {}", start.pos);
    println!("end: {}", end.pos);
    let answer = a_star(
        [*start],
        |node| node.pos.manhattan_distance(&end.pos) as usize,
        |&node| node,
        |node| node.pos == end.pos,
        move |node| {
            input
                .neighbors_plus(node.pos)
                .chain([node.pos])
                .filter(move |pt| {
                    //filter blizzards
                    if *pt == start.pos {
                        return true;
                    }
                    for dir in Blizzard::iter() {
                        let (bdx, bdy) = dir.delta();
                        let bliz_coord = Coord2d::from_coords(
                            ((pt.x - bdx * (node.steps + 1)) as i32)
                                .rem_euclid(input.width() as i32),
                            ((pt.y - bdy * (node.steps + 1)) as i32)
                                .rem_euclid(input.height() as i32),
                        );
                        if input[bliz_coord] == *dir {
                            return false;
                        }
                    }
                    true
                })
                .map(|pos| Node {
                    pos,
                    steps: node.steps + 1,
                })
                .collect_vec()
        },
    )
    .expect("to find shortest path");
    println!("node {}", answer.1.steps);
    answer.0 + 1
}

impl Solution for Day {
    type Parsed = Grid<Blizzard>;
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 18;
    const ANSWER_1: Self::Answer = 255;
    const EXAMPLE_ANSWER_2: Self::Answer = 54;
    const ANSWER_2: Self::Answer = 809;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let x_len = input.lines().next().unwrap().len() as u32 - 2;
        let y_len = input.lines().count() as u32 - 2;

        let mut blizzards: Grid<Blizzard> =
            Grid::with_dimensions_init(x_len, y_len, |_, _| Blizzard::None);
        for (y, line) in input.lines().dropping(1).dropping_back(1).enumerate() {
            for (x, c) in line.chars().dropping(1).dropping_back(1).enumerate() {
                let coord = Coord2d::from_coords(x as i32, y as i32);
                match c {
                    '.' => {}
                    '>' => blizzards[coord] = Blizzard::East,
                    '<' => blizzards[coord] = Blizzard::West,
                    '^' => blizzards[coord] = Blizzard::North,
                    'v' => blizzards[coord] = Blizzard::South,
                    _ => unreachable!(),
                }
            }
        }
        Ok(("", blizzards))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let start: Node = Node {
            pos: Coord2d::from_coords(0, -1),
            steps: 0,
        };
        let end: Node = Node {
            pos: Coord2d::from_coords(input.width() as i32 - 1, input.height() as i32 - 1),
            steps: 0,
        };
        solve(&input, &start, &end)
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        let start_coord = Coord2d::from_coords(0, -1);
        let start_coord_grid = Coord2d::from_coords(0, 0);
        let end_coord = Coord2d::from_coords(input.width() as i32 - 1, input.height() as i32);
        let end_coord_grid =
            Coord2d::from_coords(input.width() as i32 - 1, input.height() as i32 - 1);

        let mut start: Node = Node {
            pos: start_coord,
            steps: 0,
        };
        let mut end: Node = Node {
            pos: end_coord_grid,
            steps: 0,
        };

        let to_end = solve(&input, &start, &end);
        println!("to end {}", to_end);

        end.steps = to_end as i32;
        end.pos = end_coord;
        start.pos = start_coord_grid;

        let back_for_snacks = solve(&input, &end, &start);
        println!("back for snacks {}", back_for_snacks);

        start.steps = (to_end + back_for_snacks) as i32;
        start.pos = start_coord;
        end.pos = end_coord_grid;

        let back_to_end = solve(&input, &start, &end);
        println!("back to end {}", back_to_end);
        to_end + back_for_snacks + back_to_end
    }
}

use crate::prelude::*;
use ahash::AHashSet;

day!(9, parse => part1, part2);

enum Direction {
    Right,
    Left,
    Up,
    Down,
}
impl std::str::FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => anyhow::bail!("Could not match direction"),
        }
    }
}

struct Motion {
    direction: Direction,
    num: u32,
}

impl FromStr for Motion {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (left, right) = input.split_once(' ').expect("invalid input for motion");
        let motion = Motion {
            direction: left.parse::<Direction>()?,
            num: right.parse::<u32>()?,
        };
        Ok(motion)
    }
}

fn parse(input: &str) -> ParseResult<Vec<Motion>> {
    let motions = input
        .lines()
        .map(|line| line.parse::<Motion>().expect("valid motion line"))
        .collect();
    Ok(motions)
}

fn move_head(mut head: Coord2d<i32>, dir: &Direction) -> Coord2d<i32> {
    match dir {
        Direction::Right => head.x += 1,
        Direction::Left => head.x -= 1,
        Direction::Up => head.y += 1,
        Direction::Down => head.y -= 1,
    };
    head
}

fn move_knot(mut knot: Coord2d<i32>, prev: Coord2d<i32>) -> Coord2d<i32> {
    let valid_head_coords = vec![
        (knot.x, knot.y),
        (knot.x + 1, knot.y),
        (knot.x + 1, knot.y + 1),
        (knot.x, knot.y + 1),
        (knot.x - 1, knot.y + 1),
        (knot.x - 1, knot.y),
        (knot.x - 1, knot.y - 1),
        (knot.x, knot.y - 1),
        (knot.x + 1, knot.y - 1),
    ];
    if valid_head_coords
        .iter()
        .any(|coord| prev.x == coord.0 && prev.y == coord.1)
    {
        // Tail does not need to move
        knot
    } else {
        // Tail does needs to move
        let delta = prev - knot;
        knot.x += delta.x.signum();
        knot.y += delta.y.signum();
        knot
    }
}

fn part1(input: &[Motion]) -> u32 {
    let mut visited: AHashSet<Coord2d<i32>> = AHashSet::new();
    let mut head: Coord2d<i32> = Coord2d { x: 0, y: 0 };
    let mut tail: Coord2d<i32> = Coord2d { x: 0, y: 0 };
    for motion in input {
        for _ in 0..motion.num {
            // Move head
            head = move_head(head, &motion.direction);
            tail = move_knot(tail, head);
            visited.insert(tail);
        }
    }
    visited.iter().count() as u32
}

fn part2(input: &[Motion]) -> u32 {
    let mut visited: AHashSet<Coord2d<i32>> = AHashSet::new();
    let mut head: Coord2d<i32> = Coord2d { x: 0, y: 0 };
    let mut knots: Vec<Coord2d<i32>> = vec![Coord2d { x: 0, y: 0 }; 9];
    for motion in input {
        for _ in 0..motion.num {
            // Move head
            head = move_head(head, &motion.direction);
            let mut prev_knot = head;
            for knot in knots.iter_mut() {
                *knot = move_knot(*knot, prev_knot);
                prev_knot = *knot
            }
            visited.insert(*knots.last().unwrap());
        }
    }
    visited.iter().count() as u32
}

tests! {
    const EXAMPLE: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
    const EXAMPLE_LG: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    const INPUT: &str = include_str!("data/09.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 13);
    simple_tests!(parse, part1, part1_input_test, INPUT => 6018);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 1);
    simple_tests!(parse, part2, part2_example_lg_test, EXAMPLE_LG => 36);
    simple_tests!(parse, part2, part2_input_test, INPUT => 2619);
}

use ahash::AHashSet;
use framework::boilerplate;
use framework::vec::Coord2d;
use framework::IResult;
use framework::SolutionData;
use std::str::FromStr;

boilerplate!(
    Day,
    9,
    "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
",
    "data/09.txt"
);

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

fn move_head(mut head: Coord2d, dir: &Direction) -> Coord2d {
    match dir {
        Direction::Right => head.x += 1,
        Direction::Left => head.x -= 1,
        Direction::Up => head.y += 1,
        Direction::Down => head.y -= 1,
    };
    head
}

fn move_knot(mut knot: Coord2d, prev: Coord2d) -> Coord2d {
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

impl Solution for Day {
    type Parsed = Vec<Motion>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 13;
    const ANSWER_1: Self::Answer = 6018;
    const EXAMPLE_ANSWER_2: Self::Answer = 0;
    const ANSWER_2: Self::Answer = 2619;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let motions = input
            .lines()
            .map(|line| line.parse::<Motion>().expect("valid motion line"))
            .collect();
        Ok(("", motions))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let mut visited: AHashSet<Coord2d> = AHashSet::new();
        let mut head: Coord2d = Coord2d { x: 0, y: 0 };
        let mut tail: Coord2d = Coord2d { x: 0, y: 0 };
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

    fn part2(input: Self::Parsed) -> Self::Answer {
        let mut visited: AHashSet<Coord2d> = AHashSet::new();
        let mut head: Coord2d = Coord2d { x: 0, y: 0 };
        let mut knots: Vec<Coord2d> = vec![Coord2d { x: 0, y: 0 }; 9];
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
}

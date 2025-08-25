use ahash::AHashSet;
use common::{solution, Answer};
use glam::IVec2;
use std::str::FromStr;

solution!("Rope Bridge", 9);

#[derive(Clone, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}
impl std::str::FromStr for Direction {
    type Err = miette::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => miette::bail!("Could not match direction"),
        }
    }
}

#[derive(Clone, Debug)]
struct Motion {
    direction: Direction,
    num: u32,
}

impl FromStr for Motion {
    type Err = miette::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (left, right) = input.split_once(' ').expect("invalid input for motion");
        let motion = Motion {
            direction: left.parse::<Direction>()?,
            num: right.parse::<u32>().map_err(|e| miette::miette!("Parse error: {}", e))?,
        };
        Ok(motion)
    }
}

fn move_head(mut head: IVec2, dir: &Direction) -> IVec2 {
    match dir {
        Direction::Right => head.x += 1,
        Direction::Left => head.x -= 1,
        Direction::Up => head.y += 1,
        Direction::Down => head.y -= 1,
    };
    head
}

fn move_knot(mut knot: IVec2, prev: IVec2) -> IVec2 {
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

type Input = Vec<Motion>;

fn parse(input: &str) -> nom::IResult<&str, Input> {
    let motions = input
        .lines()
        .map(|line| line.parse::<Motion>().expect("valid motion line"))
        .collect();
    Ok(("", motions))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut visited: AHashSet<IVec2> = AHashSet::new();
    let mut head: IVec2 = IVec2 { x: 0, y: 0 };
    let mut tail: IVec2 = IVec2 { x: 0, y: 0 };
    for motion in data {
        for _ in 0..motion.num {
            // Move head
            head = move_head(head, &motion.direction);
            tail = move_knot(tail, head);
            visited.insert(tail);
        }
    }
    Ok((visited.iter().count() as u32).into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut visited: AHashSet<IVec2> = AHashSet::new();
    let mut head: IVec2 = IVec2 { x: 0, y: 0 };
    let mut knots: Vec<IVec2> = vec![IVec2 { x: 0, y: 0 }; 9];
    for motion in data {
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
    Ok((visited.iter().count() as u32).into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 13.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 1.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 9)?;
        assert_eq!(super::part_1(input.as_str())?, 6018.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 9)?;
        assert_eq!(super::part_2(input.as_str())?, 2619.into());
        Ok(())
    }
}

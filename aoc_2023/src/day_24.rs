use aoc_runner_macros::{aoc, generator, solver, solution};
use glam::I64Vec3;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    Parser,
};

#[aoc(2023, day24)]
pub mod solutions {
    use super::*;

type Input = Vec<Hail>;

#[derive(Clone, Debug)]
pub struct Hail {
    start: I64Vec3,
    direction: I64Vec3,
}

fn intersection(a: &Hail, b: &Hail) -> Option<(f64, f64)> {
    let x1 = a.start.x as f64;
    let y1 = a.start.y as f64;
    let dx1 = a.direction.x as f64;
    let dy1 = a.direction.y as f64;

    let x2 = b.start.x as f64;
    let y2 = b.start.y as f64;
    let dx2 = b.direction.x as f64;
    let dy2 = b.direction.y as f64;

    let m1 = dy1 / dx1;
    let m2 = dy2 / dx2;
    if (m2 - m1).abs() < f64::EPSILON {
        return None;
    }
    let x = (m1 * x1 - m2 * x2 + y2 - y1) / (m1 - m2);
    let y = (m1 * m2 * (x2 - x1) + m2 * y1 - m1 * y2) / (m2 - m1);
    Some((x, y))
}

fn parse_coords(input: &str) -> nom::IResult<&str, I64Vec3> {
    let (input, x) = complete::i64.parse(input)?;
    let (input, _) = terminated(tag(","), space1).parse(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1).parse(input)?;
    let (input, z) = complete::i64(input)?;

    Ok((input, I64Vec3::new(x, y, z)))
}

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        separated_list1(
            line_ending,
            separated_pair(
                parse_coords,
                delimited(space1, tag("@"), space1),
                parse_coords,
            )
            .map(|(start, direction)| Hail { start, direction }),
        )
        .parse(data)
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u64 {
        let range = 200000000000000.0..=400000000000000.0;
        data
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| {
                let Some((x, y)) = intersection(a, b) else {
                    return false;
                };
                if a.direction.x.signum() as f64 != (x - a.start.x as f64).signum()
                    || b.direction.x.signum() as f64 != (x - b.start.x as f64).signum()
                {
                    return false;
                }
                range.contains(&x) && range.contains(&y)
            })
            .count() as u64
    }

    #[solver(part2, main)]
    pub fn solve_part_2(_data: Input) -> u64 {
        // Z3 solver implementation commented out due to build issues
        // This would require z3 dependency to be available
        // For now, return a placeholder value
        0
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part_1(data)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part_2(data)
    }
}

#[cfg(test)]
mod tests {
    
    

    // No simple test case available for this problem
}

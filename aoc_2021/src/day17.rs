use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
};
use std::cmp::Ordering;

#[aoc(2021, day17)]
pub mod solutions {
    use super::*;

    type Input = (i32, i32, i32, i32);

    fn arc(
        (x, y): (i32, i32),
        (_x_lower_bound, x_upper_bound, y_lower_bound, _y_upper_bound): (i32, i32, i32, i32),
    ) -> Vec<(i32, i32)> {
        let mut steps = vec![];
        for timestep in 0.. {
            let new_y = (0..timestep).map(|step| y - step).sum();
            let new_x = (0..timestep)
                .map(|step| if (x - step) <= 0 { 0 } else { x - step })
                .sum();
            if new_x > x_upper_bound {
                break;
            };
            if new_y < y_lower_bound {
                break;
            };
            steps.push((new_x, new_y));
        }
        steps
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (input, _) = tag::<&str, &str, nom::error::Error<&str>>("target area: x=")(input).unwrap();
        let (input, x_lower_bound) = complete::i32::<&str, nom::error::Error<&str>>(input).unwrap();
        let (input, _) = tag::<&str, &str, nom::error::Error<&str>>("..")(input).unwrap();
        let (input, x_upper_bound) = complete::i32::<&str, nom::error::Error<&str>>(input).unwrap();
        let (input, _) = tag::<&str, &str, nom::error::Error<&str>>(", y=")(input).unwrap();
        let (input, y_lower_bound) = complete::i32::<&str, nom::error::Error<&str>>(input).unwrap();
        let (input, _) = tag::<&str, &str, nom::error::Error<&str>>("..")(input).unwrap();
        let (_, y_upper_bound) = complete::i32::<&str, nom::error::Error<&str>>(input).unwrap();

        (x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound)
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> i32 {
        let (x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound) = *input;
        
        let possible_xs = 0..=x_upper_bound;
        let possible_ys = 0..=(y_lower_bound).abs();
        let x_target = x_lower_bound..x_upper_bound;
        let y_target = y_lower_bound..y_upper_bound;
        let result = possible_xs
            .cartesian_product(possible_ys)
            .map(|(x, y)| {
                arc(
                    (x, y),
                    (x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound),
                )
            })
            // filter out any arcs that don't hit target
            .filter(|arc| {
                arc.iter()
                    .any(|(x, y)| x_target.contains(x) && y_target.contains(y))
            })
            .max_by_key(|arc| arc.iter().max_by_key(|(_, y)| y).unwrap().1);
        // get max y value
        let max_y = result.unwrap().iter().max_by_key(|(_, y)| y).unwrap().1;
        max_y
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> i32 {
        let (x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound) = *input;
        
        let possible_xs = 0..=x_upper_bound;
        let new_upper_bound = match y_lower_bound.abs().cmp(&y_upper_bound.abs()) {
            Ordering::Less => y_upper_bound.abs(),
            Ordering::Equal => y_upper_bound.abs(),
            Ordering::Greater => y_lower_bound.abs(),
        };
        let possible_ys = y_lower_bound..=new_upper_bound;
        let x_target = x_lower_bound..=x_upper_bound;
        let y_target = y_lower_bound..=y_upper_bound;
        let result = possible_xs
            .cartesian_product(possible_ys)
            .map(|(x, y)| {
                arc(
                    (x, y),
                    (x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound),
                )
            })
            // filter out any arcs that don't hit target
            .filter(|arc| {
                arc.iter().any(|(x, y)| {
                    let valid = x_target.contains(x) && y_target.contains(y);
                    valid
                })
            });
        let results = result
            .flat_map(|arc| {
                let mut it = arc.into_iter();
                it.next();
                it.next()
            })
            .collect::<Vec<(i32, i32)>>();
        // get max y value
        results.len() as i32
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> i32 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> i32 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod test {
    use super::solutions::*;

    const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part_1_example() -> miette::Result<()> {
        let _input = input_generator(EXAMPLE);
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        let _input = input_generator(EXAMPLE);
        Ok(())
    }
}

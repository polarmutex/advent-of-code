use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self},
};
use std::cmp::Ordering;

boilerplate!(
    Day,
    17,
    "\
target area: x=20..30, y=-10..-5
",
    "data/17.txt"
);

fn arc(
    (x, y): (i32, i32),
    (x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound): (i32, i32, i32, i32),
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

impl Solution for Day {
    type Parsed = (i32, i32, i32, i32);
    type Answer = i32;
    const EXAMPLE_ANSWER_1: Self::Answer = 45;
    const ANSWER_1: Self::Answer = 7626;
    const EXAMPLE_ANSWER_2: Self::Answer = 112;
    const ANSWER_2: Self::Answer = 2032;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let (input, _) = tag("target area: x=")(input)?;
        let (input, x_lower_bound) = complete::i32(input)?;
        let (input, _) = tag("..")(input)?;
        let (input, x_upper_bound) = complete::i32(input)?;
        let (input, _) = tag(", y=")(input)?;
        let (input, y_lower_bound) = complete::i32(input)?;
        let (input, _) = tag("..")(input)?;
        let (input, y_upper_bound) = complete::i32(input)?;

        Ok((
            input,
            (x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound),
        ))
    }

    fn part1(
        (x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound): Self::Parsed,
    ) -> Self::Answer {
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
        result.unwrap().iter().max_by_key(|(_, y)| y).unwrap().1
    }

    fn part2(
        (x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound): Self::Parsed,
    ) -> Self::Answer {
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
}

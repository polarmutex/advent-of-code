use common::{solution, Answer};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
};
use std::cmp::Ordering;

solution!("Trick Shot", 17);

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

fn parse(input: &str) -> nom::IResult<&str, Input> {
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

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, (x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound)) = 
        parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
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
    Ok(max_y.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, (x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound)) = 
        parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
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
    Ok((results.len() as i32).into())
}

#[cfg(test)]
mod test {
    use common::load_raw;

    const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 45.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 112.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2021, 17)?;
        assert_eq!(super::part_1(input.as_str())?, 7626.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2021, 17)?;
        assert_eq!(super::part_2(input.as_str())?, 2032.into());
        Ok(())
    }
}

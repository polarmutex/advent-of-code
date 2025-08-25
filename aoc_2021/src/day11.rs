use common::{solution, Answer};
use nom::IResult;
use ndarray::{s, Array2};
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
};

solution!("Dumbo Octopus", 11);

#[derive(Debug, Clone)]
enum Squid {
    NeedsToFlash,
    Flashed,
    Energy(u8),
    NoSquid,
}

// Squid can be constructed with u8 > 9, which
// breaks the assumptions of the board and impl
// functions
impl Squid {
    // fn should_flash(&self) -> bool {
    //     match self {
    //         Squid::Energy(x) if x > &9 => true,
    //         _ => false,
    //     }
    // }
    fn increment_energy(&mut self) {
        match self {
            Squid::Energy(x) => {
                if x == &9 {
                    *self = Squid::NeedsToFlash
                } else {
                    *x += 1
                }
            }
            _ => {}
        }
    }
    fn reset(&mut self) {
        match self {
            Squid::Flashed => *self = Squid::Energy(0),
            _ => {}
        }
    }
}

fn row(input: &str) -> IResult<&str, Vec<Squid>> {
    let (input, chars) = many1(one_of("0123456789"))(input)?;
    let nums = [Squid::NoSquid]
        .into_iter()
        .chain(
            chars
                .iter()
                .map(|v| Squid::Energy(v.to_digit(10).expect("to have succeeded") as u8)),
        )
        .chain([Squid::NoSquid])
        .collect::<Vec<Squid>>();

    Ok((input, nums))
}

fn parse(input: &str) -> IResult<&str, Array2<Squid>> {
        let (input, outputs) = separated_list1(newline, row)(input)?;
        let nrows = outputs.len();
        let ncols = outputs[0].len();

        let v = vec![Squid::NoSquid; ncols];
        let data = v
            .iter()
            .cloned()
            .chain(outputs.into_iter().flatten())
            .chain(v.iter().cloned())
            .collect::<Vec<Squid>>();

        let arr = Array2::from_shape_vec((nrows + 2, ncols), data).unwrap();
        Ok((input, arr))
    }

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, mut input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
        let mut flashes: u64 = 0;
        for _ in 0..100 {
            // Part A: Increment all squids
            for (_, squid) in input.indexed_iter_mut() {
                squid.increment_energy();
            }
            // Part B: Flash all Squids
            loop {
                if !input.indexed_iter().any(|(_, squid)| match squid {
                    Squid::NeedsToFlash => true,
                    _ => false,
                }) {
                    break;
                }
                let shape = input.shape();
                let row_count = shape[0] - 2;
                let col_count = shape[1] - 2;

                // flash squids
                for row_idx in 0..row_count {
                    for col_idx in 0..col_count {
                        let mut squids =
                            input.slice_mut(s![row_idx..=row_idx + 2, col_idx..=col_idx + 2]);
                        let central_squid = &mut squids[(1, 1)];
                        match central_squid {
                            Squid::NeedsToFlash => {
                                *central_squid = Squid::Flashed;
                                flashes = flashes + 1;
                                for squid in squids.iter_mut() {
                                    squid.increment_energy()
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            // Part C: Reset Flashed Squids
            for (_, squid) in input.indexed_iter_mut() {
                squid.reset();
            }
        }

        Ok(flashes.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, mut input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
        let mut step: u64 = 0;
        // problem counts from "step 1", not "step 0"
        for i in 1.. {
            // Part A: Increment all squids
            for (_, squid) in input.indexed_iter_mut() {
                squid.increment_energy();
            }

            // Part B: Flash all Squids
            loop {
                if !input.indexed_iter().any(|(_, squid)| match squid {
                    Squid::NeedsToFlash => true,
                    _ => false,
                }) {
                    break;
                }
                let shape = input.shape();
                let row_count = shape[0] - 2;
                let col_count = shape[1] - 2;

                // flash squids
                for row_idx in 0..row_count {
                    for col_idx in 0..col_count {
                        let mut squids =
                            input.slice_mut(s![row_idx..=row_idx + 2, col_idx..=col_idx + 2]);
                        let central_squid = &mut squids[(1, 1)];
                        match central_squid {
                            Squid::NeedsToFlash => {
                                *central_squid = Squid::Flashed;
                                // flashes = flashes + 1;
                                for squid in squids.iter_mut() {
                                    squid.increment_energy()
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            if input.indexed_iter().all(|(_, squid)| match squid {
                Squid::Flashed => true,
                Squid::NoSquid => true,
                _ => false,
            }) {
                step = i;
                break;
            }
            // Part C: Reset Flashed Squids
            for (_, squid) in input.indexed_iter_mut() {
                squid.reset();
            }
        }

        Ok(step.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_1(EXAMPLE).unwrap(), Answer::Number(1656));
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_2(EXAMPLE).unwrap(), Answer::Number(195));
    }
}

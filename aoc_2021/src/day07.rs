use common::{solution, Answer};

solution!("The Treachery of Whales", 7);

type Input = Vec<u32>;

fn parse(input: &str) -> nom::IResult<&str, Input> {
    let crabs: Result<Vec<u32>, _> = input
        .split(',')
        .map(|val| val.parse::<u32>())
        .collect();
    
    match crabs {
        Ok(c) => Ok(("", c)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::MapRes))),
    }
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, crabs) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let (low, high) = crabs.iter().fold((1_000, 0), |mut bounds, crab| {
        if bounds.0 > *crab {
            bounds.0 = *crab
        }
        if bounds.1 < *crab {
            bounds.1 = *crab
        }
        bounds
    });

    let result = (low..high)
        .map(|pos| crabs.iter().map(|crab| crab.abs_diff(pos)).sum::<u32>())
        .min()
        .unwrap();
    
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, crabs) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let (low, high) = crabs.iter().fold((1_000, 0), |mut bounds, crab| {
        if bounds.0 > *crab {
            bounds.0 = *crab
        }
        if bounds.1 < *crab {
            bounds.1 = *crab
        }
        bounds
    });

    let result = (low..high)
        .map(|pos| {
            crabs
                .iter()
                .map(|crab| {
                    let d = crab.abs_diff(pos);
                    d * (d + 1) / 2
                })
                .sum::<u32>()
        })
        .min()
        .unwrap();
    
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 37.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 168.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2021, 7)?;
        assert_eq!(super::part_1(input.as_str())?, 335330.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2021, 7)?;
        assert_eq!(super::part_2(input.as_str())?, 92439766.into());
        Ok(())
    }
}

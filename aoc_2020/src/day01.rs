use common::{solution, Answer};
use itertools::Itertools;

solution!("Report Repair", 1);

type Input = Vec<u32>;

fn parse(data: &str) -> nom::IResult<&str, Input> {
    let numbers: Vec<u32> = data
        .trim()
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    Ok(("", numbers))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data
        .iter()
        .combinations(2)
        .find(|pair| pair[0] + pair[1] == 2020)
        .map(|pair| pair[0] * pair[1])
        .unwrap_or(0);
        
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data
        .iter()
        .combinations(3)
        .find(|triple| triple[0] + triple[1] + triple[2] == 2020)
        .map(|triple| triple[0] * triple[1] * triple[2])
        .unwrap_or(0);
        
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        1721
        979
        366
        299
        675
        1456
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 514579.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 241861950.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2020, 1)?;
        assert_eq!(super::part_1(input.as_str())?, 55776.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2020, 1)?;
        assert_eq!(super::part_2(input.as_str())?, 223162626.into());
        Ok(())
    }
}

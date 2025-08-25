use common::{solution, Answer};
use itertools::Itertools;

solution!("Sonar Sweep", 1);

type Input = Vec<u32>;

fn parse(input: &str) -> nom::IResult<&str, Input> {
    let nums: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    Ok(("", nums))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data
        .iter()
        .tuple_windows()
        .filter(|&(&a, &b)| b > a)
        .count() as u32;
    
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data
        .iter()
        .tuple_windows()
        .map(|(&a, &b, &c)| a + b + c)
        .tuple_windows()
        .filter(|&(a, b)| b > a)
        .count() as u32;
    
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 7.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 5.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2021, 1)?;
        assert_eq!(super::part_1(input.as_str())?, 1448.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2021, 1)?;
        assert_eq!(super::part_2(input.as_str())?, 1471.into());
        Ok(())
    }
}

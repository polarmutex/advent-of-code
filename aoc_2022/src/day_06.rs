use ahash::AHashSet;
use common::{solution, Answer};

solution!("Tuning Trouble", 6);

type Input = Vec<char>;

fn solve(input: &[char], window: usize) -> usize {
    input
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(window)
        .find(|w| w.iter().map(|p| p.1).collect::<AHashSet<_>>().len() == window)
        .unwrap()[0]
        .0
        + window
}

fn parse(input: &str) -> nom::IResult<&str, Input> {
    let chars = input.trim().chars().collect();
    Ok(("", chars))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = solve(&data, 4);
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = solve(&data, 14);
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 11.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 26.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 6)?;
        assert_eq!(super::part_1(input.as_str())?, 1042.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 6)?;
        assert_eq!(super::part_2(input.as_str())?, 2980.into());
        Ok(())
    }
}

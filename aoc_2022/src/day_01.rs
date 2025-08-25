use common::{solution, Answer};
use itertools::Itertools;
use std::cmp::Reverse;

solution!("Calorie Counting", 1);

type Input = Vec<u32>;

fn parse(data: &str) -> nom::IResult<&str, Input> {
    let groups: Vec<u32> = data
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    Ok(("", groups))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data.into_iter().max().expect("At least one elf");
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data.into_iter()
        .sorted_unstable_by_key(|&cals| Reverse(cals))
        .take(3)
        .sum::<u32>();
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 24000.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 45000.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 1)?;
        assert_eq!(super::part_1(input.as_str())?, 68802.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 1)?;
        assert_eq!(super::part_2(input.as_str())?, 205370.into());
        Ok(())
    }
}

use aoc_lib::hash::*;
use common::{solution, Answer};

solution!("Historian Hysteria", 1);

type Input = (Vec<u32>, Vec<u32>);

fn parse(input: &str) -> Input {
    let (mut left, mut right) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<u32>().unwrap());
        right.push(parts.next().unwrap().parse::<u32>().unwrap());
    }
    (left, right)
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    println!("finished 1 1");
    Ok(left
        .into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u32>()
        .into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (left, right) = parse(input);

    let mut map = FastMap::with_capacity(1_000);
    right.iter().for_each(|v| *map.entry(v).or_insert(0) += 1);

    Ok(left
        .iter()
        .map(|left| map.get(left).unwrap_or(&0) * left)
        .sum::<u32>()
        .into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const CASE: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn part_1_case() -> miette::Result<()> {
        assert_eq!(super::part_1(CASE)?, 11.into());
        Ok(())
    }

    #[test]
    fn part_2_case() -> miette::Result<()> {
        assert_eq!(super::part_2(CASE)?, 31.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2024, 1)?;
        assert_eq!(super::part_1(input.as_str())?, 1320851.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2024, 1)?;
        assert_eq!(super::part_2(input.as_str())?, 26859182.into());
        Ok(())
    }
}

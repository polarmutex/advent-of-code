use ahash::AHashSet;
use common::{solution, Answer};
use itertools::Itertools;

solution!("Rucksack Reorganization", 3);

type Input = Vec<String>;

fn parse(data: &str) -> nom::IResult<&str, Input> {
    Ok(("", data.split('\n').map(String::from).collect()))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut priority: u32 = 0;
    for rucksack in data {
        let halfs = rucksack.split_at(rucksack.len() / 2);
        let mut common: AHashSet<char> = AHashSet::new();
        for c in halfs.0.chars() {
            if halfs.1.contains(c) {
                common.insert(c);
            }
        }
        for c in common.iter() {
            priority += score(*c);
        }
    }
    Ok(priority.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data.iter()
        .tuples()
        .map(|(a, b, c)| {
            let mut common = '0';
            for letter in a.chars() {
                if b.contains(letter) && c.contains(letter) {
                    common = letter;
                    break;
                }
            }
            score(common)
        })
        .sum::<u32>();
    
    Ok(result.into())
}

fn score(c: char) -> u32 {
    if c.is_lowercase() {
        1 + (c as u32) - 97
    } else if c.is_uppercase() {
        27 + (c as u32) - 65
    } else {
        unreachable!();
    }
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 157.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 70.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 3)?;
        assert_eq!(super::part_1(input.as_str())?, 7845.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 3)?;
        assert_eq!(super::part_2(input.as_str())?, 2790.into());
        Ok(())
    }
}

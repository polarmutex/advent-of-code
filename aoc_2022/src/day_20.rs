use common::{solution, Answer};


use itertools::Itertools;

solution!("Grove Positioning System", 20);

type Input = Vec<i64>;

fn mix<const ITERATIONS: u32, const DECRYPTION_KEY: i64>(input: &[i64]) -> i64 {
    let input = input.iter().map(|x| x * DECRYPTION_KEY).collect_vec();
    let mut new = (0..input.len()).collect::<Vec<_>>();
    for _ in 0..ITERATIONS {
        for (i, &x) in input.iter().enumerate() {
            let pos = new.iter().position(|&y| y == i).unwrap();
            new.remove(pos);
            let new_idx = (pos as i64 + x).rem_euclid(new.len() as i64) as usize;
            new.insert(new_idx, i);
        }
    }
    let orig_zero_idx = input.iter().position(|&i| i == 0).unwrap();
    let zero_idx = new.iter().position(|&i| i == orig_zero_idx).unwrap();
    [1_000, 2_000, 3_000]
        .iter()
        .map(|i| input[new[(zero_idx + i) % new.len()]])
        .sum()
}

fn parse(data: &str) -> nom::IResult<&str, Input> {
    let vec = data
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect_vec();
    Ok(("", vec))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let result = mix::<1, 1>(&data);
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let result = mix::<10, 811_589_153>(&data);
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;

    const EXAMPLE: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 3.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 1623178306_i64.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 20)?;
        assert_eq!(super::part_1(input.as_str())?, 5962.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 20)?;
        assert_eq!(super::part_2(input.as_str())?, 9862431387256_i64.into());
        Ok(())
    }
}

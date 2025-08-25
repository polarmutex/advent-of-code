use common::{solution, Answer};


use itertools::Itertools;
use std::str::FromStr;

solution!("Full of Hot Air", 25);

#[derive(Clone, Debug)]
struct Snafu {
    val: String,
}
impl FromStr for Snafu {
    type Err = miette::Error;
    fn from_str(input: &str) -> Result<Snafu, Self::Err> {
        let val = Snafu {
            val: input.to_string(),
        };
        Ok(val)
    }
}
impl Snafu {
    pub fn to_decimal(&self) -> i64 {
        self.val.chars().rev().enumerate().fold(0, |acc, (i, c)| {
            let digit = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            };
            acc + 5_i64.pow(i as u32) * digit
        })
    }
    pub fn to_snafu(val: i64) -> Self {
        let mut s = String::from("");
        let mut val = val;
        while val > 0 {
            let (digit, c) = match val % 5 {
                0 => (0, '0'),
                1 => (1, '1'),
                2 => (2, '2'),
                3 => (-2, '='),
                4 => (-1, '-'),
                _ => unreachable!(),
            };
            s.insert(0, c);
            val -= digit;
            val /= 5;
        }
        Self { val: s }
    }
}

type Input = Vec<Snafu>;

fn parse(data: &str) -> nom::IResult<&str, Input> {
    let snafus: Vec<Snafu> = data
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();
    Ok(("", snafus))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let sum: i64 = data.iter().map(|snafu| snafu.to_decimal()).sum();
    let result = Snafu::to_snafu(sum).val;
    Ok(result.into())
}

fn part_2(_input: &str) -> miette::Result<Answer> {
    Ok("".into())
}

#[cfg(test)]
mod test {
    use common::load_raw;

    const EXAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, "2=-1=0".into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, "".into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 25)?;
        assert_eq!(super::part_1(input.as_str())?, "121=2=1==0=10=2-20=2".into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 25)?;
        assert_eq!(super::part_2(input.as_str())?, "".into());
        Ok(())
    }
}

use common::{solution, Answer};
use nom::IResult;

solution!("Beacon Scanner", 19);

fn parse(_input: &str) -> IResult<&str, Vec<u32>> {
    let i = vec![];
    Ok(("", i))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, _input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    todo!()
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, _input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    #[ignore]
    fn test_part_1() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_1(EXAMPLE).unwrap(), Answer::Number(79));
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_2(EXAMPLE).unwrap(), Answer::Number(3621));
    }
}

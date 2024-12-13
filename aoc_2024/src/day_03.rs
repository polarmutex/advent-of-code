use common::{solution, Answer};
use miette::miette;
use miette::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::anychar;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;

solution!("Mull It Over", 3);

#[derive(Debug, Clone)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}
fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discarded, ins)| ins))(input)
}

#[tracing::instrument]
fn part_1(input: &str) -> Result<Answer> {
    let (_, instructions) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    Ok(instructions
        .iter()
        .fold(0, |ans, ins| match ins {
            Instruction::Mul(l, r) => ans + (l * r),
            _ => ans,
        })
        .into())
}

#[derive(PartialEq, Eq)]
enum State {
    Process,
    DontProcess,
}

fn part_2(input: &str) -> Result<Answer> {
    let (_, instructions) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    Ok(instructions
        .iter()
        .fold((0, State::Process), |(ans, state), ins| match ins {
            Instruction::Mul(l, r) => match state {
                State::Process => (ans + (l * r), state),
                State::DontProcess => (ans, state),
            },
            Instruction::Do => (ans, State::Process),
            Instruction::Dont => (ans, State::DontProcess),
        })
        .0
        .into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const CASE: &str = indoc! {"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "};
    const CASE2: &str = indoc! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "};

    #[test]
    // #[test_log::test]
    fn part_1_case() -> miette::Result<()> {
        assert_eq!(super::part_1(CASE)?, 161.into());
        Ok(())
    }

    #[test]
    // #[test_log::test]
    fn part_2_case() -> miette::Result<()> {
        assert_eq!(super::part_2(CASE2)?, 48.into());
        Ok(())
    }

    #[test]
    // #[test_log::test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2024, 3)?;
        assert_eq!(super::part_1(input.as_str())?, 192767529.into());
        Ok(())
    }

    #[test]
    // #[test_log::test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2024, 3)?;
        assert_eq!(super::part_2(input.as_str())?, 104083373.into());
        Ok(())
    }
}

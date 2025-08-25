use common::{solution, Answer, pixel_vector_to_char_strings, ocr};
use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    combinator::map,
    multi::separated_list1,
    branch::alt,
    IResult,
};

solution!("Cathode-Ray Tube", 10);

#[derive(Clone, Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

type Input = Vec<Instruction>;

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(tag("noop"), |_| Instruction::Noop),
        map(nom::sequence::preceded(tag("addx "), i32), Instruction::Addx),
    ))(input)
}

fn parse(data: &str) -> nom::IResult<&str, Input> {
    separated_list1(line_ending, parse_instruction)(data)
}

fn for_each_cycle(instr: &[Instruction], mut func: impl FnMut(i32, i32)) {
    let mut cycle = 1;
    let mut x_reg = 1;
    let mut iter = instr.iter();
    let mut to_add: Option<i32> = None;
    loop {
        func(cycle, x_reg);
        if let Some(val) = to_add {
            x_reg += val;
            to_add = None;
        } else {
            match iter.next() {
                Some(Instruction::Noop) => {}
                Some(Instruction::Addx(val)) => to_add = Some(*val),
                None => break,
            }
        }
        cycle += 1;
    }
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut answer = 0;
    for_each_cycle(&data, |cycle, x_reg| {
        if (cycle + 20) % 40 == 0 {
            answer += cycle * x_reg;
        }
    });
    Ok(answer.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut pixels: Vec<char> = Vec::new();
    for_each_cycle(&data, |cycle, x_reg| {
        if cycle > 240 {
            return;
        }
        let pixel_idx = (cycle - 1) % 40;
        let sprite_range = (x_reg - 1)..(x_reg + 2);
        if sprite_range.contains(&pixel_idx) {
            pixels.push('#');
        } else {
            pixels.push('.');
        }
    });

    let answer: String = pixel_vector_to_char_strings(&pixels, 8)
        .iter()
        .map(|s| ocr(s.as_str()))
        .collect::<String>();

    Ok(answer.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 13140.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 10)?;
        assert_eq!(super::part_1(input.as_str())?, 15120.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 10)?;
        assert_eq!(super::part_2(input.as_str())?, "RKPJBPLA".into());
        Ok(())
    }
}

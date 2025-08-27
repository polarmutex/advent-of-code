use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    combinator::map,
    multi::separated_list1,
    branch::alt,
    IResult,
};

// Simple implementations for missing common functions
fn pixel_vector_to_char_strings(pixels: &[char], width: usize) -> Vec<String> {
    pixels.chunks(width * 5) // Assuming 5 rows per character
        .map(|chunk| {
            chunk.chunks(width)
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        })
        .collect()
}

fn ocr(s: &str) -> char {
    // Simple OCR mapping - would need full implementation
    match s {
        // Add character patterns here if needed
        _ => '?', // Default for unrecognized patterns
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
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

#[aoc(2022, day10)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> i32 {
        let mut answer = 0;
        for_each_cycle(input, |cycle, x_reg| {
            if (cycle + 20) % 40 == 0 {
                answer += cycle * x_reg;
            }
        });
        answer
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> String {
        let mut pixels: Vec<char> = Vec::new();
        for_each_cycle(input, |cycle, x_reg| {
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

        pixel_vector_to_char_strings(&pixels, 8)
            .iter()
            .map(|s| ocr(s.as_str()))
            .collect::<String>()
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> i32 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> String {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod test {
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
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 13140);
    }
}

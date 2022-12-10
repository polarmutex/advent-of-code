use crate::prelude::*;
use ahash::AHashMap;

day!(10, parse => part1, part2);

enum Instruction {
    Noop,
    Addx(i32),
}
impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.split(' ');
        match iter.next().unwrap() {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx(
                iter.next()
                    .unwrap()
                    .parse::<i32>()
                    .expect("i32 number after addx"),
            )),
            _ => anyhow::bail!("Could not match instruction"),
        }
    }
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Noop => write!(f, "noop"),
            Instruction::Addx(num) => write!(f, "addx {}", num),
        }
    }
}

type Program = Vec<Instruction>;

fn parse(input: &str) -> ParseResult<Program> {
    let program: Program = input
        .lines()
        .map(|line| line.parse::<Instruction>().expect("valid instruction"))
        .collect();
    Ok(program)
}

fn part1(input: &Program) -> i32 {
    let mut interesting_cycles: AHashMap<u32, i32> = AHashMap::new();
    interesting_cycles.insert(20, 0);
    interesting_cycles.insert(60, 0);
    interesting_cycles.insert(100, 0);
    interesting_cycles.insert(140, 0);
    interesting_cycles.insert(180, 0);
    interesting_cycles.insert(220, 0);

    let mut x_reg: i32 = 1;
    let mut cycle: u32 = 0;

    for instr in input {
        cycle += 1;
        if interesting_cycles.contains_key(&cycle) {
            *interesting_cycles.get_mut(&cycle).expect("") = x_reg * (cycle as i32);
        }

        match instr {
            Instruction::Noop => {} // no op
            Instruction::Addx(num) => {
                cycle += 1;
                if interesting_cycles.contains_key(&cycle) {
                    *interesting_cycles.get_mut(&cycle).expect("") = x_reg * (cycle as i32);
                }
                x_reg += num;
            }
        };
    }
    let answer = interesting_cycles.values().sum();
    println!("answer: {}", answer);
    answer
}

fn draw(cycle: u32, x_reg: i32) {
    let cur_pos = (cycle % 40) as i32;
    if x_reg == cur_pos - 1 || x_reg == cur_pos || x_reg == cur_pos + 1 {
        print!("#");
    } else {
        print!(".");
    }
    if cur_pos == 39 {
        println!();
    }
}

fn part2(input: &Program) -> u32 {
    let mut x_reg: i32 = 1;
    let mut cycle: u32 = 0;

    for instr in input {
        cycle += 1;
        draw(cycle, x_reg);

        match instr {
            Instruction::Noop => {} // no op
            Instruction::Addx(num) => {
                cycle += 1;
                x_reg += num;
                draw(cycle, x_reg);
            }
        };
    }
    0
}

tests! {
    const EXAMPLE: &str = "\
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
";
    const INPUT: &str = include_str!("../../data/2022/10.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 13140);
    simple_tests!(parse, part1, part1_input_test, INPUT => 15120);
    // MANUAL
    //simple_tests!(parse, part2, part2_example_test, EXAMPLE => 0);
    //simple_tests!(parse, part2, part2_input_test, INPUT => 0);
}

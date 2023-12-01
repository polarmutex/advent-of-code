use framework::boilerplate;
use framework::ocr::ocr;
use framework::ocr::pixel_vector_to_char_strings;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    10,
    "\
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
",
    "data/10.txt"
);

#[derive(Clone, Debug)]
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

impl Solution for Day {
    type Parsed = Program;
    type Answer = String;
    type AnswerExample = &'static str;
    const EXAMPLE_ANSWER_1: Self::AnswerExample = "13140";
    const ANSWER_1: Self::AnswerExample = "15120";
    const EXAMPLE_ANSWER_2: Self::AnswerExample = "0";
    const ANSWER_2: Self::AnswerExample = "RKPJBPLA";

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let program: Program = input
            .lines()
            .map(|line| line.parse::<Instruction>().expect("valid instruction"))
            .collect();
        Ok(("", program))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let mut answer = 0;
        for_each_cycle(&input, |cycle, x_reg| {
            if (cycle + 20) % 40 == 0 {
                answer += cycle * x_reg;
            }
        });
        println!("answer: {}", answer);
        answer.to_string()
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        let mut pixels: Vec<char> = Vec::new();
        for_each_cycle(&input, |cycle, x_reg| {
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

        // print pixels
        for (i, val) in pixels.iter().enumerate() {
            print!("{}", val);
            if (i % 40) == 39 {
                println!();
            }
        }
        let answer: String = pixel_vector_to_char_strings(&pixels, 8)
            .iter()
            .map(ocr)
            .collect::<String>();

        answer
    }
}

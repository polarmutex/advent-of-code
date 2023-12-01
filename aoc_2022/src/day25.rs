use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use std::str::FromStr;

boilerplate!(
    Day,
    25,
    "\
1=-0-2
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
122
",
    "data/25.txt"
);

#[derive(Clone, Debug)]
struct Snafu {
    val: String,
}
impl FromStr for Snafu {
    type Err = anyhow::Error;
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

#[allow(dead_code)]
fn part1a(input: Vec<Snafu>) -> u64 {
    assert!(input.len() == 1);
    input[0].to_decimal() as u64
}

impl Solution for Day {
    type Parsed = Vec<Snafu>;
    type Answer = String;
    type AnswerExample = &'static str;
    const EXAMPLE_ANSWER_1: Self::AnswerExample = "2=-1=0";
    const ANSWER_1: Self::AnswerExample = "121=2=1==0=10=2-20=2";
    const EXAMPLE_ANSWER_2: Self::AnswerExample = "";
    const ANSWER_2: Self::AnswerExample = "";

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let snafus: Vec<Snafu> = input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect_vec();
        Ok(("", snafus))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let sum: i64 = input.iter().map(|snafu| snafu.to_decimal()).sum();
        println!("sum: {}", sum);
        Snafu::to_snafu(sum).val.clone()
    }

    fn part2(_input: Self::Parsed) -> Self::Answer {
        "".into()
    }
}

// add_test_external!(part1a, part1_snafu_test_1, "1" => 1_u64);
// add_test_external!(part1a, part1_snafu_test_2, "2" => 2_u64);
// add_test_external!(part1a, part1_snafu_test_3, "1=" => 3_u64);
// add_test_external!(part1a, part1_snafu_test_4, "1-" => 4_u64);
// add_test_external!(part1a, part1_snafu_test_5, "10" => 5_u64);
// add_test_external!(part1a, part1_snafu_test_2022, "1=11-2" => 2022_u64);
// add_test_external!(part1a, part1_snafu_test_12345, "1-0---0" => 12345_u64);
// add_test_external!(part1a, part1_snafu_test_pi, "1121-1110-1=0" => 314159265_u64);

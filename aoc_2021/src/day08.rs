use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;

boilerplate!(
    Day,
    8,
    "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
",
    "data/08.txt"
);

type Segments = u8;

#[derive(Clone, Debug)]
struct Record {
    signal_patterns: Vec<Segments>,
    output: Vec<Segments>,
}

fn convert_to_signals(s: &str) -> u8 {
    let mut val = 0_u8;
    for c in s.chars() {
        match c {
            'a' => val |= 1_u8 << 0,
            'b' => val |= 1_u8 << 1,
            'c' => val |= 1_u8 << 2,
            'd' => val |= 1_u8 << 3,
            'e' => val |= 1_u8 << 4,
            'f' => val |= 1_u8 << 5,
            'g' => val |= 1_u8 << 6,
            _ => (),
        }
    }
    val
}

impl Record {
    pub fn match_signals(&self) -> Vec<Segments> {
        /*
         aaaa
        b    c
        b    c
         dddd
        e    f
        e    f
         gggg

        0 - 6 - abc efg
        1 - 2 -   c  f
        2 - 5 - a cde g
        3 - 5 - a cd fg
        4 - 4 -  bcd f
        5 - 5 - ab d fg
        6 - 6 - ab defg
        7 - 3 - a c  f
        8 - 7 - abcdefg
        9 - 6 - abcd fg
        */

        let mut found = vec![0_u8; 10];
        for &signal in self.signal_patterns.iter() {
            match signal.count_ones() {
                // 1 only one with 2 signals
                2 => found[1] = signal,
                // 4 only one with 4 signals
                4 => found[4] = signal,
                // 7 only one with 3 signals
                3 => found[7] = signal,
                // 8 only one with 7 signals
                7 => found[8] = signal,
                _ => continue,
            }
        }
        for &signal in self.signal_patterns.iter() {
            let and_one = (signal & found[1]).count_ones();
            let xor_four = (signal ^ found[4]).count_ones();

            match (signal.count_ones(), and_one, xor_four) {
                // 3 only one with 2 segments in common with 1
                (5, 2, _) => found[3] = signal,
                //
                (5, _, 5) => found[2] = signal,
                (5, _, 3) => found[5] = signal,
                (6, 1, _) => found[6] = signal,
                (6, _, 4) => found[0] = signal,
                (6, _, 2) => found[9] = signal,
                _ => continue,
            }
        }
        for (i, f) in found.iter().enumerate() {
            println!("{} - {}", i, f);
        }
        found
    }
}

impl Solution for Day {
    type Parsed = Vec<Record>;
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 26;
    const ANSWER_1: Self::Answer = 294;
    const EXAMPLE_ANSWER_2: Self::Answer = 61229;
    const ANSWER_2: Self::Answer = 973292;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let records: Vec<Record> = input
            .lines()
            .map(|line| {
                let (measurements, digits) = line.split_once(" | ").unwrap();
                let signal_patterns = measurements
                    .split_whitespace()
                    .map(convert_to_signals)
                    .collect_vec();
                let output = digits
                    .split_whitespace()
                    .map(convert_to_signals)
                    .collect_vec();
                Record {
                    signal_patterns,
                    output,
                }
            })
            .collect_vec();
        Ok(("", records))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        input
            .iter()
            .flat_map(|record| &record.output)
            .filter(|digit| matches!(digit.count_ones(), 2 | 3 | 4 | 7))
            .count()
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        input
            .iter()
            .map(|record| {
                let found = record.match_signals();
                let nums = record
                    .output
                    .iter()
                    .map(|digit| found.iter().position(|y| digit == y).unwrap())
                    .collect_vec();
                for n in &nums {
                    print!("{}", n);
                }
                println!();
                let ans = nums.iter().fold(0, |res, nr| res * 10 + nr);
                println!("ans: {}", ans);
                ans
            })
            .sum()
    }
}

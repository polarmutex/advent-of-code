use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;


type Input = Vec<Record>;

type Segments = u8;

#[derive(Clone, Debug)]
pub struct Record {
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
        // Debug output removed for production
        found
    }
}

fn parse(input: &str) -> nom::IResult<&str, Input> {
    let records: Result<Vec<Record>, miette::Error> = input
        .lines()
        .map(|line| {
            let (measurements, digits) = line.split_once(" | ")
                .ok_or_else(|| miette::miette!("Invalid line format: {}", line))?;
            let signal_patterns = measurements
                .split_whitespace()
                .map(convert_to_signals)
                .collect_vec();
            let output = digits
                .split_whitespace()
                .map(convert_to_signals)
                .collect_vec();
            Ok(Record {
                signal_patterns,
                output,
            })
        })
        .collect();
    
    match records {
        Ok(r) => Ok(("", r)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::MapRes))),
    }
}

#[aoc(2021, day8)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> usize {
        input
            .iter()
            .flat_map(|record| &record.output)
            .filter(|digit| matches!(digit.count_ones(), 2 | 3 | 4 | 7))
            .count()
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> usize {
        input
            .iter()
            .map(|record| {
                let found = record.match_signals();
                let nums = record
                    .output
                    .iter()
                    .map(|digit| found.iter().position(|y| digit == y).unwrap())
                    .collect_vec();
                nums.iter().fold(0, |res, nr| res * 10 + nr)
            })
            .sum::<usize>()
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> usize {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> usize {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
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
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 26);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 61229);
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        Ok(())
    }
}
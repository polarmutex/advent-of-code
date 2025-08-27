use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::{IResult, character::complete::{self, one_of, char}, error::context, multi::many1, bytes::complete::tag};
use std::{collections::HashMap, fmt};
use nom_supreme::error::{ErrorTree, StackContext::Context};
use itertools::Itertools;

type Input = Vec<String>;


pub type IResultSpecial<'a, T> = nom::IResult<&'a str, T, ErrorTree<&'a str>>;

#[aoc(2021, day10)]
pub mod solutions {
    use super::*;

fn chunk(original_input: &str) -> IResultSpecial<()> {
    let (input, open_char) = one_of("({<[")(original_input)?;
    let c_res: IResult<&str, &str> = tag(match open_char {
        '{' => "}",
        '(' => ")",
        '[' => "]",
        '<' => ">",
        _ => panic!("unrecognized char"),
    })(input);
    if let Ok((input, _)) = c_res {
        Ok((input, ()))
    } else {
        let (input, _chunks) = context("chunk", many1(chunk))(input)?;
        //let mut input = input;
        loop {
            match input.chars().next() {
                Some('{') | Some('(') | Some('[') | Some('<') => chunk(input)?,
                // we aren't setting input again
                _ => {
                    break;
                }
            };
        }
        let (input, _) = context(
            "chars",
            tag(match open_char {
                '{' => "}",
                '(' => ")",
                '[' => "]",
                '<' => ">",
                _ => panic!("unrecognized char"),
            }),
        )(input)?;
        Ok((input, ()))
    }
}

#[derive(Debug, PartialEq)]
struct Ast {
    left: char,
    right: Option<char>,
    children: Vec<Ast>,
}

impl fmt::Display for Ast {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{}{}",
            // self.left,
            self.children
                .iter()
                .map(|v| v.to_string())
                .collect::<String>(),
            if self.right.is_none() {
                match self.left {
                    '{' => "}",
                    '(' => ")",
                    '[' => "]",
                    '<' => ">",
                    _ => panic!("unrecognized char"),
                }
            } else {
                // self.right.unwrap()
                ""
            }
        )
    }
}

fn chunk_2(original_input: &str) -> IResult<&str, Ast> {
    let (input, open_char) = complete::one_of("({<[")(original_input)?;
    let c_res: IResult<&str, char> = complete::char(match open_char {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        '<' => '>',
        _ => panic!("unrecognized char"),
    })(input);
    match c_res {
        Ok((input, close_char)) => Ok((
            input,
            Ast {
                left: open_char,
                right: Some(close_char),
                children: vec![],
            },
        )),
        Err(_e) => {
            if input == "" {
                Ok((
                    input,
                    Ast {
                        left: open_char,
                        right: None,
                        children: vec![],
                    },
                ))
            } else {
                let (input, output) = many1(chunk_2)(input)?;
                let c_res: IResult<&str, char> = char(match open_char {
                    '{' => '}',
                    '(' => ')',
                    '[' => ']',
                    '<' => '>',
                    _ => panic!("unrecognized char"),
                })(input);
                match c_res {
                    Ok((input, c)) => Ok((
                        input,
                        Ast {
                            left: open_char,
                            right: Some(c),
                            children: output,
                        },
                    )),
                    Err(nom::Err::Incomplete(_)) => Ok((
                        input,
                        Ast {
                            left: open_char,
                            right: None,
                            children: output,
                        },
                    )),
                    Err(e) => Err(e),
                }
            }
        }
    }
}

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        input.lines().map(|l| l.to_string()).collect_vec()
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u32 {
        let scoring: HashMap<char, u32> =
            HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
        let results: u32 = input
            .iter()
            .map(|line| chunk(line))
            // filter out lines that end early
            .filter_map(|res| match res {
                Ok(_) => None,
                Err(nom::Err::Incomplete(_e)) => None,
                Err(nom::Err::Error(ErrorTree::Stack { base: _, contexts })) => {
                    let ctx = contexts.iter().find(|v| v.1 == Context("chars")).unwrap();
                    let c = ctx.0.chars().next().unwrap();
                    let res = scoring.get(&c);
                    res
                }

                _ => panic!("uh oh"),
            })
            .sum();
        results
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u64 {
        let mut results: Vec<u64> = input
            .iter()
            .map(|line| {
                let mut res = chunk_2(line);
                loop {
                    match res {
                        Ok((input, output)) => {
                            if input.len() > 0 {
                                res = chunk_2(input);
                            } else {
                                break Ok((input, output));
                            }
                        }
                        Err(e) => break Err(e),
                    }
                }
            })
            // filter out lines that end early
            .filter_map(|res| {
                match res {
                    Ok((_input, v)) => {
                        let num = v.to_string().chars().fold(0, |acc, v| {
                            acc * 5
                                + match v {
                                    ')' => 1,
                                    ']' => 2,
                                    '}' => 3,
                                    '>' => 4,
                                    _ => panic!("askflj"),
                                }
                        });
                        Some(num)
                    }
                    Err(_e) => {
                        // dbg!(e);
                        None
                    }
                }
            })
            .collect();
        results.sort();
        results[results.len() / 2]
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod tests {


    const EXAMPLE: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part_1() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 26397);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 288957);
    }
}

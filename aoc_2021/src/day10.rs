use common::{solution, Answer};
use nom::IResult;
use itertools::Itertools;
use std::{collections::HashMap, fmt};

use nom::{
    character::{
        complete::{self},
        streaming::{char, one_of},
    },
    error::context,
    multi::many1,
};
use nom_supreme::error::ErrorTree;
use nom_supreme::error::StackContext::Context;
use nom_supreme::tag::streaming::tag;

solution!("Syntax Scoring", 10);

pub type IResultSpecial<'a, T> = nom::IResult<&'a str, T, ErrorTree<&'a str>>;

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

fn parse(input: &str) -> IResult<&str, Vec<String>> {
    Ok(("", input.lines().map(|l| l.to_string()).collect_vec()))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
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
    Ok((results as u64).into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
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
    Ok((results[results.len() / 2]).into())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_1(EXAMPLE).unwrap(), Answer::Number(26397));
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_2(EXAMPLE).unwrap(), Answer::Number(288957));
    }
}

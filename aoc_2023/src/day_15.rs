use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::alpha1;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::Parser;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Operation {
    Dash,
    Equal,
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub full: String,
    pub label: String,
    pub operation: Operation,
    pub focal_length: Option<u8>,
}

type Boxes = HashMap<u32, Vec<Lense>>;

#[derive(Clone, Debug)]
pub struct Lense {
    pub label: String,
    pub focal_lengh: u8,
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |mut acc, c| {
        let acsii_code = c as u32;
        acc += acsii_code;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    let (input, label) = alpha1.map(ToString::to_string).parse(input)?;
    let (input, operation) = is_a("-=").parse(input)?;
    let (input, focal_length) = opt(complete::u8).parse(input)?;

    let focal_length_str = if let Some(x) = focal_length {
        x.to_string()
    } else {
        String::from("")
    };

    Ok((
        input,
        Instruction {
            full: format!("{label}{operation}{focal_length_str}"),
            label,
            operation: match operation {
                "-" => Operation::Dash,
                "=" => Operation::Equal,
                _ => panic!("illegal char"),
            },
            focal_length,
        },
    ))
}

type Input = Vec<Instruction>;

#[aoc(2023, day15)]
pub mod solutions {
    use super::*;

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        separated_list1(tag(","), parse_instruction).parse(data)
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u32 {
        data.iter()
            .map(|v| {
                v.full.chars().fold(0, |mut acc, c| {
                    let acsii_code = c as u32;
                    acc += acsii_code;
                    acc *= 17;
                    acc %= 256;
                    acc
                })
            })
            .sum()
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> u32 {
        let init_boxes: Boxes = HashMap::new();
        data.iter()
            .fold(init_boxes, |mut boxes, i| {
                let box_num = hash(&i.label);
                match i.operation {
                    Operation::Dash => {
                        boxes.entry(box_num).or_default();
                        let b = boxes.get_mut(&box_num).expect("");
                        let existing_idx = b.iter().position(|v| v.label == i.label);
                        if let Some(x) = existing_idx {
                            b.remove(x);
                        }
                    }
                    Operation::Equal => {
                        // make sure entry exists
                        boxes.entry(box_num).or_default();
                        let b = boxes.get_mut(&box_num).expect("");
                        let existing_idx = b.iter().position(|v| v.label == i.label);
                        if existing_idx.is_none() {
                            b.push(Lense {
                                label: i.label.clone(),
                                focal_lengh: i.focal_length.unwrap(),
                            });
                        } else {
                            b.get_mut(existing_idx.unwrap()).expect("").focal_lengh =
                                i.focal_length.unwrap();
                        }
                    }
                };
                boxes
            })
            .into_iter()
            .map(|(box_num, b)| {
                b.iter()
                    .enumerate()
                    .map(|(i, l)| {
                        (box_num + 1) * (i as u32 + 1) * l.focal_lengh as u32
                    })
                    .sum::<u32>()
            })
            .sum()
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part_1(data)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part_2(data)
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner_macros::aoc_case;
    

    #[aoc_case(1320, 145)]
    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
}

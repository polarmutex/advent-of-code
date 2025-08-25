use common::{solution, Answer};
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::alpha1;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::Parser;
use std::collections::HashMap;
// use nom::character::complete;
// use nom_supreme::ParserExt;
// use tracing::info;
// use itertools::Itertools;

solution!("Lens Library", 15);

type Input = Vec<Instruction>;

#[derive(Clone, Debug)]
enum Operation {
    Dash,
    Equal,
}

#[derive(Clone, Debug)]
struct Instruction {
    full: String,
    label: String,
    operation: Operation,
    focal_length: Option<u8>,
}

type Boxes = HashMap<u32, Vec<Lense>>;

#[derive(Clone, Debug)]
struct Lense {
    label: String,
    focal_lengh: u8,
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

fn parse(data: &str) -> nom::IResult<&str, Input> {
    separated_list1(tag(","), parse_instruction).parse(data)
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let result: u32 = data.iter()
        .map(|v| {
            v.full.chars().fold(0, |mut acc, c| {
                let acsii_code = c as u32;
                acc += acsii_code;
                acc *= 17;
                acc %= 256;
                acc
            })
        })
        .sum();
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let init_boxes: Boxes = HashMap::new();
    let result: u32 = data.iter()
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
        .sum();
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use super::*;

    #[test]
    fn part_1_example() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(super::part_1(input)?, 1320.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(super::part_2(input)?, 145.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 15)?;
        assert_eq!(super::part_1(input.as_str())?, 508498.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 15)?;
        assert_eq!(super::part_2(input.as_str())?, 279116.into());
        Ok(())
    }
}

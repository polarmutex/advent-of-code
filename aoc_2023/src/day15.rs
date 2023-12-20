use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
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

boilerplate!(
    Day,
    15,
    "\
",
    "data/15.txt"
);

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

fn parse_instruction(input: &str) -> IResult<Instruction> {
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

impl Solution for Day {
    type Parsed = Vec<Instruction>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        separated_list1(tag(","), parse_instruction).parse(input)
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        dbg!(&data);
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

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
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
            .inspect(|v| {
                dbg!(v);
            })
            .map(|(box_num, b)| {
                b.iter()
                    .enumerate()
                    .map(|(i, l)| {
                        let val = (box_num + 1) * (i as u32 + 1) * l.focal_lengh as u32;
                        dbg!(val);
                        val
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}

tests! {
     const EXAMPLE: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 1320);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 508498);
    add_test!(part2_example, test_part2_example, EXAMPLE => 145);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 279116);
}

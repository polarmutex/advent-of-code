use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::combinator::opt;
use nom::multi::fold_many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::Parser;
use std::collections::HashMap;
use std::ops::RangeInclusive;
// use nom_supreme::ParserExt;
// use tracing::info;
// use itertools::Itertools;

boilerplate!(
    Day,
    19,
    "\
",
    "data/19.txt"
);

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operation {
    LessThan,
    GreaterThan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Target {
    Workflow(String),
    Accepted,
    Rejected,
}

#[derive(Clone, Debug)]
enum Rule {
    Target(Target),
    Condition {
        field: String,
        op: Operation,
        value: u32,
        target: Target,
    },
}

#[derive(Clone, Debug)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn run_rules(&self, p: &Part) -> Target {
        let mut last: Option<Target> = None;
        for r in &self.rules {
            match r {
                Rule::Target(t) => {
                    last = Some(t.clone());
                    break;
                }
                Rule::Condition {
                    field,
                    op,
                    value,
                    target,
                } => {
                    let val = match field.as_str() {
                        "x" => p.x,
                        "m" => p.m,
                        "a" => p.a,
                        "s" => p.s,
                        _ => unreachable!(""),
                    };
                    let test: bool = match op {
                        Operation::LessThan => val < *value,
                        Operation::GreaterThan => val > *value,
                    };
                    if test {
                        last = Some(target.clone());
                        break;
                    }
                }
            }
        }
        last.expect("to find target")
    }
}

#[derive(Clone, Debug, Default)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Clone, Debug)]
struct PartRange {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}
impl PartRange {
    fn set(&mut self, field: &str, value: RangeInclusive<u64>) {
        match field {
            "x" => self.x = value,
            "m" => self.m = value,
            "a" => self.a = value,
            "s" => self.s = value,
            _ => unreachable!(""),
        }
    }
}
impl Default for PartRange {
    fn default() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

fn sort_part(w: &HashMap<String, Workflow>, p: &Part) -> Target {
    let start = w.get("in").expect("to find in node");
    let mut cur_wf = start;
    loop {
        match cur_wf.run_rules(p) {
            Target::Rejected => break Target::Rejected,
            Target::Accepted => break Target::Accepted,
            Target::Workflow(id) => cur_wf = w.get(&id).expect("to find next workflow"),
        }
    }
}

fn find_accepted_combinations(p: PartRange, w: &HashMap<String, Workflow>, t: &Target) -> u64 {
    match t {
        Target::Rejected => 0,
        Target::Accepted => {
            (p.x.end() - p.x.start() + 1)
                * (p.m.end() - p.m.start() + 1)
                * (p.a.end() - p.a.start() + 1)
                * (p.s.end() - p.s.start() + 1)
        }
        Target::Workflow(id) => {
            dbg!(&id);
            let cur_wf = w.get(id).expect("to find in node");
            let mut sum = 0;
            let mut cur_p = p;
            for r in &cur_wf.rules {
                match r {
                    Rule::Target(t) => sum += find_accepted_combinations(cur_p.clone(), w, t),
                    Rule::Condition {
                        field,
                        op,
                        value,
                        target,
                    } => {
                        let val = match field.as_str() {
                            "x" => &cur_p.x,
                            "m" => &cur_p.m,
                            "a" => &cur_p.a,
                            "s" => &cur_p.s,
                            _ => unreachable!(""),
                        };
                        if val.contains(&(*value as u64)) {
                            match op {
                                Operation::LessThan => {
                                    let lower = *val.start()..=(*value as u64 - 1);
                                    let upper = (*value as u64)..=*val.end();
                                    let mut p_prime = cur_p.clone();
                                    p_prime.set(field.as_str(), lower);
                                    cur_p.set(field.as_str(), upper);
                                    sum += find_accepted_combinations(p_prime, w, target)
                                }
                                Operation::GreaterThan => {
                                    let lower = *val.start()..=(*value as u64);
                                    let upper = (*value as u64 + 1)..=*val.end();
                                    let mut p_prime = cur_p.clone();
                                    p_prime.set(field.as_str(), upper);
                                    cur_p.set(field.as_str(), lower);
                                    sum += find_accepted_combinations(p_prime, w, target)
                                }
                            }
                        } else if (val.end() < &(*value as u64) && op == &Operation::LessThan)
                            || (val.start() > &(*value as u64) && op == &Operation::GreaterThan)
                        {
                            sum += find_accepted_combinations(cur_p.clone(), w, target)
                        } else {
                            sum += 0
                        }
                    }
                }
            }
            sum
        }
    }
}

fn parse_target(input: &str) -> IResult<Target> {
    alt((
        tag("A").map(|_| Target::Accepted),
        tag("R").map(|_| Target::Rejected),
        alpha1.map(ToString::to_string).map(Target::Workflow),
    ))
    .parse(input)
}
fn parse_rule(input: &str) -> IResult<Rule> {
    let (input, field) = alpha1.map(ToString::to_string).parse(input)?;
    let (input, op) = alt((
        complete::char('>').map(|_| Operation::GreaterThan),
        complete::char('<').map(|_| Operation::LessThan),
    ))
    .parse(input)?;
    let (input, value) = complete::u32.parse(input)?;
    let (input, _) = complete::char(':').parse(input)?;
    let (input, target) = parse_target.parse(input)?;
    Ok((
        input,
        Rule::Condition {
            field,
            op,
            value,
            target,
        },
    ))
}

fn parse_workflow(input: &str) -> IResult<Workflow> {
    let (input, id) = alpha1.map(ToString::to_string).parse(input)?;
    let (input, rules) = delimited(
        tag("{"),
        separated_list1(tag(","), alt((parse_rule, parse_target.map(Rule::Target)))),
        tag("}"),
    )
    .parse(input)?;
    Ok((input, Workflow { id, rules }))
}

fn parse_part(input: &str) -> IResult<Part> {
    delimited(
        complete::char('{'),
        fold_many1(
            terminated(
                separated_pair(alpha1, complete::char('='), complete::u32),
                opt(tag(",")),
            ),
            Part::default,
            |mut acc, (c, v)| {
                match c {
                    "x" => acc.x = v,
                    "m" => acc.m = v,
                    "a" => acc.a = v,
                    "s" => acc.s = v,
                    _ => unreachable!("should not contain this letter"),
                }
                acc
            },
        ),
        complete::char('}'),
    )
    .parse(input)
}

impl Solution for Day {
    type Parsed = Input;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        let (input, workflows) = separated_list1(line_ending, parse_workflow).parse(input)?;
        let (input, _) = multispace1.parse(input)?;
        let (input, parts) = separated_list1(line_ending, parse_part).parse(input)?;
        Ok((
            input,
            Input {
                parts,
                workflows: workflows
                    .iter()
                    .map(|v| (v.id.clone(), v.clone()))
                    .collect::<HashMap<String, Workflow>>(),
            },
        ))
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        dbg!(&data);
        let workflows = data.workflows;
        data.parts
            .into_iter()
            .map(|p| {
                let s = (p.x + p.m + p.a + p.s) as u64;
                (s, sort_part(&workflows, &p))
            })
            .filter(|(_, t)| t == &Target::Accepted)
            .map(|(s, _)| s)
            .sum()
    }

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
        dbg!(&data);
        let p = PartRange::default();
        let workflows = data.workflows;
        find_accepted_combinations(p, &workflows, &Target::Workflow(String::from("in")))
    }
}

tests! {
     const EXAMPLE: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 19114);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 319062);
    add_test!(part2_example, test_part2_example, EXAMPLE => 167409079868000);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 118638369682135);
}

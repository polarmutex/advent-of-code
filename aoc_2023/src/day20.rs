use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
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
use std::collections::VecDeque;
use std::ops::RangeInclusive;
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    20,
    "\
",
    "data/20.txt"
);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Signal {
    High,
    Low,
}

#[derive(Clone, Debug)]
enum FlipFlopStatus {
    On,
    Off,
}

#[derive(Clone, Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop { status: FlipFlopStatus },
    Conjunction { memory: HashMap<String, Signal> },
}
#[derive(Clone, Debug)]
struct Module {
    module_type: ModuleType,
    id: String,
    destinations: Vec<String>,
}

impl Module {
    fn compute(&mut self, from_id: String, signal: &Signal) -> Vec<(String, String, Signal)> {
        match &mut self.module_type {
            ModuleType::Broadcast => self
                .destinations
                .iter()
                .map(|v| (self.id.clone(), v.clone(), signal.clone()))
                .collect_vec(),
            ModuleType::FlipFlop { ref mut status } => match (signal, &status) {
                (Signal::High, _) => vec![],
                (Signal::Low, FlipFlopStatus::On) => {
                    *status = FlipFlopStatus::Off;
                    self.destinations
                        .iter()
                        .map(|v| (self.id.clone(), v.clone(), Signal::Low))
                        .collect_vec()
                }
                (Signal::Low, FlipFlopStatus::Off) => {
                    *status = FlipFlopStatus::On;
                    self.destinations
                        .iter()
                        .map(|v| (self.id.clone(), v.clone(), Signal::High))
                        .collect_vec()
                }
            },
            ModuleType::Conjunction { memory } => {
                *memory.get_mut(&from_id).unwrap() = *signal;
                let next = memory
                    .values()
                    .all(|v| v == &Signal::High)
                    .then_some(Signal::Low)
                    .unwrap_or(Signal::High);
                self.destinations
                    .iter()
                    .map(|v| (self.id.clone(), v.clone(), next.clone()))
                    .collect_vec()
            }
        }
    }
}

fn parse_broadcast(input: &str) -> IResult<Module> {
    let (input, id) = tag("broadcaster -> ")
        .map(|_| String::from("broadcaster"))
        .parse(input)?;
    let (input, destinations) =
        separated_list1(tag(", "), alpha1.map(ToString::to_string)).parse(input)?;
    Ok((
        input,
        Module {
            id,
            module_type: ModuleType::Broadcast,
            destinations,
        },
    ))
}

fn parse_flipflop(input: &str) -> IResult<Module> {
    let (input, module_type) = tag("%")
        .map(|_| ModuleType::FlipFlop {
            status: FlipFlopStatus::Off,
        })
        .parse(input)?;
    let (input, id) = alpha1.map(ToString::to_string).parse(input)?;
    let (input, _) = tag(" -> ").parse(input)?;
    let (input, destinations) =
        separated_list1(tag(", "), alpha1.map(ToString::to_string)).parse(input)?;
    Ok((
        input,
        Module {
            id,
            module_type,
            destinations,
        },
    ))
}

fn parse_conjunction(input: &str) -> IResult<Module> {
    let (input, module_type) = tag("&")
        .map(|_| ModuleType::Conjunction {
            memory: HashMap::new(),
        })
        .parse(input)?;
    let (input, id) = alpha1.map(ToString::to_string).parse(input)?;
    let (input, _) = tag(" -> ").parse(input)?;
    let (input, destinations) =
        separated_list1(tag(", "), alpha1.map(ToString::to_string)).parse(input)?;
    Ok((
        input,
        Module {
            id,
            module_type,
            destinations,
        },
    ))
}

impl Solution for Day {
    type Parsed = HashMap<String, Module>;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        let (input, machines) = separated_list1(
            line_ending,
            alt((parse_broadcast, parse_flipflop, parse_conjunction)),
        )
        .parse(input)?;
        Ok((
            input,
            machines
                .into_iter()
                .map(|v| (v.id.clone(), v))
                .collect::<HashMap<String, Module>>(),
        ))
    }

    #[tracing::instrument(skip(data))]
    fn part1(mut data: Self::Parsed) -> Self::Answer {
        let button_pushes = 1000;
        let mut high_pulses = 0;
        let mut low_pulses = 0;

        // initialize default of conjunciton module
        let conjunctions = data
            .iter()
            .filter_map(|(id, machine)| match &machine.module_type {
                ModuleType::Conjunction { .. } => Some(id.clone()),
                _ => None,
            })
            .collect::<Vec<String>>();
        let inputs = data.iter().fold(
            HashMap::<String, Vec<String>>::new(),
            |mut acc, (id, machine)| {
                for c in conjunctions.iter() {
                    if machine.destinations.contains(c) {
                        acc.entry(c.clone())
                            .and_modify(|item| {
                                item.push(id.clone());
                            })
                            .or_insert(vec![id.clone()]);
                    }
                }
                acc
            },
        );
        inputs
            .into_iter()
            .for_each(|(conjunction, input_machines)| {
                data.entry(conjunction).and_modify(|machine| {
                    let ModuleType::Conjunction { memory, .. } = &mut machine.module_type else {
                        unreachable!("has to exist");
                    };
                    *memory = input_machines
                        .into_iter()
                        .map(|id| (id, Signal::Low))
                        .collect();
                });
            });
        dbg!(&data);

        for _ in 0..button_pushes {
            // low pulse on button press
            low_pulses += 1;

            let mut q = VecDeque::from([(
                String::from("button"),
                String::from("broadcaster"),
                Signal::Low,
            )]);
            while let Some((from, id, signal)) = q.pop_front() {
                let m = data.get_mut(&id);
                let next = if m.is_none() {
                    vec![]
                } else {
                    m.unwrap().compute(from, &signal)
                };
                next.iter().for_each(|(_, _, s)| match s {
                    Signal::Low => low_pulses += 1,
                    Signal::High => high_pulses += 1,
                });
                q.extend(next);
            }
        }

        dbg!(&high_pulses);
        dbg!(&low_pulses);
        high_pulses * low_pulses
    }

    #[tracing::instrument(skip(data))]
    fn part2(mut data: Self::Parsed) -> Self::Answer {
        // initialize default of conjunciton module
        let conjunctions = data
            .iter()
            .filter_map(|(id, machine)| match &machine.module_type {
                ModuleType::Conjunction { .. } => Some(id.clone()),
                _ => None,
            })
            .collect::<Vec<String>>();
        let inputs = data.iter().fold(
            HashMap::<String, Vec<String>>::new(),
            |mut acc, (id, machine)| {
                for c in conjunctions.iter() {
                    if machine.destinations.contains(c) {
                        acc.entry(c.clone())
                            .and_modify(|item| {
                                item.push(id.clone());
                            })
                            .or_insert(vec![id.clone()]);
                    }
                }
                acc
            },
        );
        inputs
            .into_iter()
            .for_each(|(conjunction, input_machines)| {
                data.entry(conjunction).and_modify(|machine| {
                    let ModuleType::Conjunction { memory, .. } = &mut machine.module_type else {
                        unreachable!("has to exist");
                    };
                    *memory = input_machines
                        .into_iter()
                        .map(|id| (id, Signal::Low))
                        .collect();
                });
            });
        dbg!(&data);

        let final_node = data
            .iter()
            .find_map(|(id, machine)| {
                machine
                    .destinations
                    .contains(&String::from("rx"))
                    .then_some(id.clone())
            })
            .unwrap();
        let mut nodes = data
            .iter()
            .filter_map(|(id, m)| m.destinations.contains(&final_node).then_some(id.clone()))
            .collect_vec();

        let mut lcms: Vec<u64> = vec![];
        for i in 0.. {
            if lcms.len() == 4 {
                break;
            }
            let mut q = VecDeque::from([(
                String::from("button"),
                String::from("broadcaster"),
                Signal::Low,
            )]);
            while let Some((from, id, signal)) = q.pop_front() {
                let m = data.get_mut(&id);

                if nodes.contains(&id) && signal == Signal::Low {
                    let index = nodes.iter().position(|x| x == &id).unwrap();
                    nodes.remove(index);
                    lcms.push(i + 1);
                }

                let next = if m.is_none() {
                    vec![]
                } else {
                    m.unwrap().compute(from, &signal)
                };
                q.extend(next);
            }
        }
        lcm(&lcms)
    }
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

tests! {
     const EXAMPLE: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";
     const EXAMPLE2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";


    add_test!(part1_example, test_part1_example, EXAMPLE => 32000000);
    add_test!(part1_example, test_part1_example2, EXAMPLE2 => 11687500);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 812721756);
    // NONE add_test!(part2_example, test_part2_example, EXAMPLE => 167409079868000);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 233338595643977);
}

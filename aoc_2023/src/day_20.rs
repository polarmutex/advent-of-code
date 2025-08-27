use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::Parser;
use std::collections::HashMap;
use std::collections::VecDeque;

#[aoc(2023, day20)]
pub mod solutions {
    use super::*;

type Input = HashMap<String, Module>;

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
pub struct Module {
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
                .map(|v| (self.id.clone(), v.clone(), *signal))
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
                let next = if memory.values().all(|v| v == &Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                };
                self.destinations
                    .iter()
                    .map(|v| (self.id.clone(), v.clone(), next))
                    .collect_vec()
            }
        }
    }
}

fn parse_broadcast(input: &str) -> nom::IResult<&str, Module> {
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

fn parse_flipflop(input: &str) -> nom::IResult<&str, Module> {
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

fn parse_conjunction(input: &str) -> nom::IResult<&str, Module> {
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

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        let (input, machines) = separated_list1(
            line_ending,
            alt((parse_broadcast, parse_flipflop, parse_conjunction)),
        )
        .parse(data)?;
        Ok((
            input,
            machines
                .into_iter()
                .map(|v| (v.id.clone(), v))
                .collect::<HashMap<String, Module>>(),
        ))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, main)]
    pub fn solve_part_1(mut data: Input) -> u64 {
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

        high_pulses * low_pulses
    }

    #[solver(part2, main)]
    pub fn solve_part_2(mut data: Input) -> u64 {
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

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part_1(data)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part_2(data)
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

#[cfg(test)]
mod tests {
    
    use super::solutions::*;

    #[test]
    fn part_1_example_1() {
        let input = "broadcaster -> a, b, c\n%a -> b\n%b -> c\n%c -> inv\n&inv -> a";
        assert_eq!(part_1(input), 32000000);
    }

    #[test]
    fn part_1_example_2() {
        let input = "broadcaster -> a\n%a -> inv, con\n&inv -> b\n%b -> con\n&con -> output";
        assert_eq!(part_1(input), 11687500);
    }
}

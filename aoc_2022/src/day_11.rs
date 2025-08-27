use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use std::collections::VecDeque;

#[allow(dead_code)]
const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

#[derive(Debug, Clone)]
pub struct Monkey {
    num: u32,
    items: VecDeque<u64>,
    operation: Operation,
    test_divisable: u32,
    test_true: u32,
    test_false: u32,
    inspections: u64,
}
impl std::str::FromStr for Monkey {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.lines();

        let num = iter
            .next()
            .unwrap()
            .split_once(' ')
            .expect("monkey properly parsed")
            .1
            .replace(':', "")
            .parse::<u32>()
            .expect("number for monkey");

        let mut items: VecDeque<u64> = VecDeque::new();
        iter.next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .for_each(|item| {
                items.push_back(item.trim().parse::<u64>().expect("valid item num"));
            });

        let operation = iter
            .next()
            .unwrap()
            .parse::<Operation>()
            .expect("valid operation");

        let test_divisable = iter
            .next()
            .unwrap()
            .split_once("divisible by")
            .unwrap()
            .1
            .trim()
            .parse::<u32>()
            .expect("valid test");

        let test_true = iter
            .next()
            .unwrap()
            .split_once("throw to monkey")
            .unwrap()
            .1
            .trim()
            .parse::<u32>()
            .expect("valid test true");

        let test_false = iter
            .next()
            .unwrap()
            .split_once("throw to monkey")
            .unwrap()
            .1
            .trim()
            .parse::<u32>()
            .expect("valid test true");

        let monkey = Monkey {
            num,
            items,
            operation,
            test_divisable,
            test_true,
            test_false,
            inspections: 0,
        };
        Ok(monkey)
    }
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Monkey: {}", self.num)?;
        write!(f, "\titems: ")?;
        for i in self.items.iter() {
            write!(f, "{} ", i)?;
        }
        writeln!(f)?;
        writeln!(f, "\toperation: {}", self.operation)?;
        writeln!(f, "\ttest: {}", self.test_divisable)?;
        writeln!(f, "\ttest true: {}", self.test_true)?;
        writeln!(f, "\ttest false: {}", self.test_false)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Mul(u32),
    Add(u32),
    Squared,
}
impl std::str::FromStr for Operation {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input
            .split_once(": new =")
            .unwrap()
            .1
            .split_whitespace()
            .collect_vec();
        assert!(input.len() == 3);
        match input[1] {
            "+" => Ok(Operation::Add(
                input[2].parse::<u32>().expect("u32 for add num"),
            )),
            "*" => match input[2] {
                "old" => Ok(Operation::Squared),
                _ => Ok(Operation::Mul(
                    input[2].parse::<u32>().expect("u32 for mut num"),
                )),
            },
            _ => Err("Could not match operation".to_string()),
        }
    }
}
impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Squared => write!(f, "squared"),
            Operation::Add(val) => write!(f, "+ {}", val),
            Operation::Mul(val) => write!(f, "* {}", val),
        }
    }
}

fn keep_away<const ROUNDS: u32, const RELIEF_FACTOR: u8>(monkeys: &[Monkey]) -> u64 {
    let mut monkeys = monkeys.to_vec();

    // Key observations is that all divisors are PRIME numbers
    let modulo: u32 = monkeys.iter().map(|m| m.test_divisable).product();

    for _ in 0..ROUNDS {
        for monkey_num in 0..monkeys.len() {
            //println!("{}", monkey);
            while !monkeys[monkey_num].items.is_empty() {
                let mut item = monkeys[monkey_num]
                    .items
                    .pop_front()
                    .expect("item to be there");

                monkeys[monkey_num].inspections += 1;

                match monkeys[monkey_num].operation {
                    Operation::Add(num) => item += num as u64,
                    Operation::Mul(num) => item *= num as u64,
                    Operation::Squared => item *= item,
                };
                item %= modulo as u64;
                item /= RELIEF_FACTOR as u64;
                let throw_to = if item % monkeys[monkey_num].test_divisable as u64 == 0 {
                    monkeys[monkey_num].test_true as usize
                } else {
                    monkeys[monkey_num].test_false as usize
                };
                monkeys[throw_to].items.push_back(item);
            }
        }
    }
    let monkey_inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect_vec();
    for val in &monkey_inspections {
        println!("{}", val);
    }
    let max = *monkey_inspections.iter().max().unwrap();
    let next_max = *monkey_inspections.iter().sorted().nth_back(1).unwrap();
    println!("{} {}", max, next_max);
    max * next_max
}

type Input = Vec<Monkey>;

#[aoc(2022, day11)]
pub mod solutions {
    use super::*;

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        let monkeys = data
            .split("\n\n")
            .map(|monkey| monkey.parse::<Monkey>().expect("valid monkeys"))
            .collect_vec();
        Ok(("", monkeys))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u64 {
        keep_away::<20, 3>(input)
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u64 {
        keep_away::<10_000, 1>(input)
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u64 {
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
mod test {

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(super::EXAMPLE), 10605);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(super::EXAMPLE), 2713310158u64);
    }
}

use crate::prelude::*;
use std::collections::HashMap;

day!(21, parse => part1, part2);

enum Opertation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum Node {
    Number(usize),
    Op(String, Opertation, String),
}

type Nodes = HashMap<String, Node>;

fn parse(input: &str) -> ParseResult<Nodes> {
    let nodes: Nodes = input
        .lines()
        .map(|line| {
            let (key, equation) = line.split_once(':').unwrap();
            let equation = equation.split_whitespace().collect_vec();
            let op = match equation.len() {
                1 => Node::Number(equation[0].parse().unwrap()),
                3 => match equation[1] {
                    "+" => Node::Op(
                        equation[0].to_string(),
                        Opertation::Add,
                        equation[2].to_string(),
                    ),
                    "-" => Node::Op(
                        equation[0].to_string(),
                        Opertation::Subtract,
                        equation[2].to_string(),
                    ),
                    "*" => Node::Op(
                        equation[0].to_string(),
                        Opertation::Multiply,
                        equation[2].to_string(),
                    ),
                    "/" => Node::Op(
                        equation[0].to_string(),
                        Opertation::Divide,
                        equation[2].to_string(),
                    ),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            (key.to_string(), op)
        })
        .collect();
    Ok(nodes)
}

fn eval(nodes: &Nodes, id: &String) -> usize {
    match &nodes[id] {
        Node::Number(num) => *num,
        Node::Op(lhs, op, rhs) => {
            let (left, right) = (eval(nodes, lhs), eval(nodes, rhs));
            match op {
                Opertation::Add => left + right,
                Opertation::Subtract => left - right,
                Opertation::Multiply => left * right,
                Opertation::Divide => left / right,
            }
        }
    }
}

fn part1(input: &Nodes) -> usize {
    eval(input, &String::from("root"))
}

fn path_from_root_to_humn(nodes: &Nodes, id: &String) -> Vec<String> {
    if id == "humn" {
        vec![id.clone()]
    } else {
        match &nodes[id] {
            Node::Number(_) => vec![],
            Node::Op(lhs, _, rhs) => {
                let (mut left, mut right) = (
                    path_from_root_to_humn(nodes, lhs),
                    path_from_root_to_humn(nodes, rhs),
                );
                if !left.is_empty() {
                    left.push(id.clone());
                    left
                } else if !right.is_empty() {
                    right.push(id.clone());
                    right
                } else {
                    vec![]
                }
            }
        }
    }
}

fn solve(nodes: &Nodes, result: usize, mut path: Vec<String>) -> usize {
    if path.len() == 1 {
        return result;
    }
    let next_id = path.pop().unwrap();
    match &nodes[&next_id] {
        Node::Number(_) => unreachable!(),
        Node::Op(lhs, op, rhs) => {
            let (first, other) = if *lhs == *path.last().unwrap() {
                (true, eval(nodes, rhs))
            } else {
                (false, eval(nodes, lhs))
            };
            let new_result = match op {
                Opertation::Add => result - other,

                Opertation::Subtract => {
                    if first {
                        result + other
                    } else {
                        other - result
                    }
                }
                Opertation::Multiply => result / other,

                Opertation::Divide => {
                    if first {
                        result * other
                    } else {
                        other / result
                    }
                }
            };
            solve(nodes, new_result, path)
        }
    }
}

fn part2(input: &HashMap<String, Node>) -> usize {
    let mut path = path_from_root_to_humn(input, &String::from("root"));

    // remvoe id root
    path.pop();

    // find either side of root
    let root = &input[&String::from("root")];
    if let Node::Op(lhs, _, rhs) = root {
        let next_id = path.last().unwrap();
        let result = if *lhs == *next_id {
            eval(input, rhs)
        } else {
            eval(input, lhs)
        };
        solve(input, result, path)
    } else {
        unreachable!();
    }
}

tests! {
    const EXAMPLE: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    const INPUT: &str = include_str!("data/21.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 152);
    simple_tests!(parse, part1, part1_input_test, INPUT => 158731561459602);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 301);
    simple_tests!(parse, part2, part2_input_test, INPUT => 3769668716709);
}

use common::{solution, Answer};


use itertools::Itertools;
use std::collections::HashMap;

solution!("Monkey Math", 21);

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Debug)]
enum Node {
    Number(usize),
    Op(String, Operation, String),
}

type Input = HashMap<String, Node>;

fn eval(nodes: &Input, id: &String) -> usize {
    match &nodes[id] {
        Node::Number(num) => *num,
        Node::Op(lhs, op, rhs) => {
            let (left, right) = (eval(nodes, lhs), eval(nodes, rhs));
            match op {
                Operation::Add => left + right,
                Operation::Subtract => left - right,
                Operation::Multiply => left * right,
                Operation::Divide => left / right,
            }
        }
    }
}

fn path_from_root_to_humn(nodes: &Input, id: &String) -> Vec<String> {
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

fn solve(nodes: &Input, result: usize, mut path: Vec<String>) -> usize {
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
                Operation::Add => result - other,

                Operation::Subtract => {
                    if first {
                        result + other
                    } else {
                        other - result
                    }
                }
                Operation::Multiply => result / other,

                Operation::Divide => {
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
fn parse(data: &str) -> nom::IResult<&str, Input> {
    let nodes: Input = data
        .lines()
        .map(|line| {
            let (key, equation) = line.split_once(':').unwrap();
            let equation = equation.split_whitespace().collect_vec();
            let op = match equation.len() {
                1 => Node::Number(equation[0].parse().unwrap()),
                3 => match equation[1] {
                    "+" => Node::Op(
                        equation[0].to_string(),
                        Operation::Add,
                        equation[2].to_string(),
                    ),
                    "-" => Node::Op(
                        equation[0].to_string(),
                        Operation::Subtract,
                        equation[2].to_string(),
                    ),
                    "*" => Node::Op(
                        equation[0].to_string(),
                        Operation::Multiply,
                        equation[2].to_string(),
                    ),
                    "/" => Node::Op(
                        equation[0].to_string(),
                        Operation::Divide,
                        equation[2].to_string(),
                    ),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            (key.to_string(), op)
        })
        .collect();
    Ok(("", nodes))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let result = eval(&data, &String::from("root"));
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let mut path = path_from_root_to_humn(&data, &String::from("root"));

    // remove id root
    path.pop();

    // find either side of root
    let root = &data[&String::from("root")];
    if let Node::Op(lhs, _, rhs) = root {
        let next_id = path.last().unwrap();
        let result = if *lhs == *next_id {
            eval(&data, rhs)
        } else {
            eval(&data, lhs)
        };
        let result = solve(&data, result, path);
        Ok(result.into())
    } else {
        unreachable!();
    }
}

#[cfg(test)]
mod test {
    use common::load_raw;

    const EXAMPLE: &str = "root: pppw + sjmn
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
hmdt: 32";

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 152.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 301.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 21)?;
        assert_eq!(super::part_1(input.as_str())?, 158731561459602_usize.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 21)?;
        assert_eq!(super::part_2(input.as_str())?, 3769668716709_usize.into());
        Ok(())
    }
}

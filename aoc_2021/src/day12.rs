use aoc_runner_macros::{aoc, generator, solver};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
};
use std::collections::BTreeMap;


type Input = BTreeMap<String, Vec<String>>;

fn step(node_id: String, path: Vec<String>, allowed_ways: &BTreeMap<String, Vec<String>>) -> usize {
    if node_id == "end" {
        1
    } else {
        let next_nodes = allowed_ways.get(&node_id).unwrap();
        next_nodes
            .iter()
            .map(|node| {
                if node.chars().all(|c| c.is_lowercase()) && path.contains(&node) {
                    0
                } else {
                    let mut new_path = path.clone();
                    new_path.push(node.clone());
                    step(node.clone(), new_path, &allowed_ways)
                }
            })
            .sum()
    }
}

fn step_2(
    node_id: String,
    path: Vec<String>,
    allowed_ways: &BTreeMap<String, Vec<String>>,
) -> usize {
    if node_id == "end" {
        1
    } else {
        let next_nodes = allowed_ways.get(&node_id).unwrap();
        next_nodes
            .iter()
            .map(|node| {
                let count_of_all_small_cave_visits = path
                    .iter()
                    .filter(|path_node| path_node.chars().all(|c| c.is_lowercase()))
                    .fold(BTreeMap::new(), |mut acc, item| {
                        acc.entry(item).and_modify(|v| *v += 1).or_insert(1);
                        acc
                    });
                let have_visited_small_cave_twice =
                    count_of_all_small_cave_visits.iter().any(|(_, &v)| v == 2);
                if node.chars().all(|c| c.is_lowercase())
                    && path.contains(&node)
                    && have_visited_small_cave_twice
                {
                    0
                } else {
                    let mut new_path = path.clone();
                    new_path.push(node.clone());
                    step_2(node.clone(), new_path, &allowed_ways)
                }
            })
            .sum()
    }
}

fn parse(input: &str) -> nom::IResult<&str, Input> {
    let (input, nodes) =
        separated_list1(newline, separated_pair(alpha1, tag("-"), alpha1))(input)?;
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (a, b) in nodes {
        if a == "end" {
            map.entry(a.into()).or_insert(vec![]);
        } else if b == "start" {
        } else {
            map.entry(a.into())
                .and_modify(|v| {
                    v.push(b.into());
                })
                .or_insert(vec![b.into()]);
        }
        if b == "end" {
            map.entry(b.into()).or_insert(vec![]);
        } else if a == "start" {
        } else {
            map.entry(b.into())
                .and_modify(|v| {
                    v.push(a.into());
                })
                .or_insert(vec![a.into()]);
        }
    }

    Ok((input, map))
}

#[aoc(2021, day12)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> usize {
        step("start".into(), vec!["start".into()], input)
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> usize {
        step_2("start".into(), vec!["start".into()], input)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;



    #[test]
    fn part_1_example() -> miette::Result<()> {
        
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        Ok(())
    }
}

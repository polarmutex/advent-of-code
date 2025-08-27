use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Move {
    num: u32,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = miette::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let move_idx = s.find("move ").unwrap();
        assert!(move_idx == 0);
        let from_idx = s.find("from ").unwrap();
        let to_idx = s.find("to ").unwrap();

        let move_rng = 4..from_idx;
        let from_rng = (from_idx + 4)..to_idx;
        let to_rng = (to_idx + 2)..;

        let m: Move = Move {
            num: s[move_rng].trim().parse().map_err(|e| miette::miette!("Parse error: {}", e))?,
            from: s[from_rng].trim().parse::<usize>().unwrap() - 1,
            to: s[to_rng].trim().parse::<usize>().unwrap() - 1,
        };
        Ok(m)
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "move {} from {} to {}", self.num, self.from, self.to)
    }
}

#[derive(Debug, Clone)]
pub struct Stacks {
    stacks: HashMap<usize, VecDeque<char>>,
    moves: Vec<Move>,
}

impl std::fmt::Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in self.stacks.iter().sorted_by_key(|(i, _)| *i) {
            write!(f, "{}: ", k)?;
            for val in v.iter() {
                write!(f, "[{}] ", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

type Input = Stacks;

fn parse(input: &str) -> nom::IResult<&str, Input> {
    let (stack_str, moves_str) = input.split_once("\n\n").unwrap();
    let stacks = stack_str
        .lines()
        .rev()
        .flat_map(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                // this maps to crane num
                .enumerate()
                // fiter numbered stacks
                .filter(|(_, c)| c.is_alphabetic())
        })
        .into_grouping_map()
        .collect::<VecDeque<char>>();

    let moves = moves_str
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .collect();
    Ok(("", Stacks { stacks, moves }))
}

#[aoc(2022, day5)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Stacks {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Stacks) -> String {
        let mut stacks = input.stacks.clone();

        for m in &input.moves {
            for _ in 0..m.num {
                let item = stacks.get_mut(&m.from).unwrap().pop_back().unwrap();
                stacks.get_mut(&m.to).unwrap().push_back(item);
            }
        }

        stacks
            .iter()
            .sorted_by_key(|(i, _)| *i)
            .map(|(_, v)| v.back().unwrap())
            .collect()
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Stacks) -> String {
        let mut stacks = input.stacks.clone();

        for m in &input.moves {
            let mut holding = VecDeque::<char>::new();
            for _ in 0..m.num {
                let item = stacks.get_mut(&m.from).unwrap().pop_back().unwrap();
                holding.push_front(item);
            }
            for val in holding.iter() {
                stacks.get_mut(&m.to).unwrap().push_back(*val);
            }
        }

        stacks
            .iter()
            .sorted_by_key(|(i, _)| *i)
            .map(|(_, v)| v.back().unwrap())
            .collect()
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> String {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> String {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3
        
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), "CMZ");
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), "MCD");
    }
}

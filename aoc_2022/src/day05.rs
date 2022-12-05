use crate::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;

day!(5, parse => part1, part2);

type Stack = Vec<char>;

#[derive(Debug, Clone, Copy)]
struct Move {
    num: u32,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let move_idx = s.find("move ").unwrap();
        assert!(move_idx == 0);
        let from_idx = s.find("from ").unwrap();
        let to_idx = s.find("to ").unwrap();

        let move_rng = 4..from_idx;
        let from_rng = (from_idx + 4)..to_idx;
        let to_rng = (to_idx + 2)..;

        let mut m: Move = Move {
            num: s[move_rng].trim().parse()?,
            from: s[from_rng].trim().parse()?,
            to: s[to_rng].trim().parse()?,
        };
        m.from = m.from - 1;
        m.to = m.to - 1;
        Ok(m)
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "move {} from {} to {}", self.num, self.from, self.to)
    }
}

#[derive(Debug, Clone)]
struct Stacks {
    stacks: HashMap<usize, VecDeque<char>>,
    moves: Vec<Move>,
}

fn parse(input: &str) -> ParseResult<Stacks> {
    let (stack_str, moves_str) = input.split_once("\n\n").unwrap();
    let stacks_num_lines = stack_str.lines().count();
    let stacks = stack_str
        .lines()
        .flat_map(
            |line| {
                line.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate() // this gives the crane number
                    .filter(|(_, c)| c.is_alphabetic())
            }, // remove crates which do not exist
        )
        .into_grouping_map() // itertools magic - this gets the crates for each crane
        .collect::<VecDeque<char>>();

    let moves = moves_str
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .collect();
    Ok(Stacks { stacks, moves })
}

fn part1(input: &Stacks) -> String {
    let mut stacks = input.stacks.clone();

    for m in &input.moves {
        println!("move {} from {} to {}", m.num, m.from, m.to);
        for i in 0..m.num {
            let item = stacks.get_mut(&m.from).unwrap().pop_front().unwrap();
            println!("\tmove {} from {} to {}", item, m.from, m.to);
            stacks.get_mut(&m.to).unwrap().push_front(item);
        }
    }

    let ans = stacks
        .iter()
        .sorted_by_key(|(i, _)| *i)
        .map(|(k, v)| v.front().unwrap())
        .collect();
    println!("answer: {}", ans);
    ans
}

fn part2(input: &Stacks) -> String {
    let mut stacks = input.stacks.clone();

    for m in &input.moves {
        println!("move {} from {} to {}", m.num, m.from, m.to);
        let mut holding = VecDeque::<char>::new();
        for i in 0..m.num {
            let item = stacks.get_mut(&m.from).unwrap().pop_front().unwrap();
            holding.push_front(item);
            //println!("\tmove {} from {} to {}", item, m.from, m.to);
        }
        for val in holding.iter() {
            stacks.get_mut(&m.to).unwrap().push_front(*val);
        }
    }

    let ans = stacks
        .iter()
        .sorted_by_key(|(i, _)| *i)
        .map(|(k, v)| v.front().unwrap())
        .collect();
    println!("answer: {}", ans);
    ans
}

tests! {
    const EXAMPLE: &str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    const INPUT: &str = include_str!("../../data/2022/05.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => "CMZ");
    simple_tests!(parse, part1, part1_input_test, INPUT => "QNNTGTPFN");
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => "MCD");
    simple_tests!(parse, part2, part2_input_test, INPUT => "GGNPJBTTR");
}

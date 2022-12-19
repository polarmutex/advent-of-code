use crate::prelude::*;
use ndarray::Array3;
use std::collections::HashMap;

day!(16, parse => part1, part2);

type ID = String;

#[derive(Clone, PartialEq, Eq)]
struct Valve {
    id: ID,
    flow_rate: u16,
    tunnels: Vec<ID>,
}
impl std::str::FromStr for Valve {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Valve, Self::Err> {
        let (left, right) = input.split_once(';').unwrap();

        let id = left[6..8].to_string();
        let flow_rate = left[left.find('=').unwrap() + 1..].parse::<u16>().unwrap();
        let tunnels = right
            .split_ascii_whitespace()
            .skip(4)
            .map(|tunnel| tunnel.trim().replace(',', ""))
            .collect_vec();
        let output = Valve {
            id,
            flow_rate,
            tunnels,
        };
        Ok(output)
    }
}
impl std::fmt::Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Valve {} - flow {} - tunnels: {}",
            self.id,
            self.flow_rate,
            self.tunnels.join(",")
        )
    }
}
impl std::cmp::Ord for Valve {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.flow_rate.cmp(&other.flow_rate)
    }
}
impl PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> ParseResult<Vec<Valve>> {
    let valves: Vec<Valve> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();
    Ok(valves)
}

/*
   TODO https://github.com/juanplopes/advent-of-code-2022/blob/main/day16.py
*/

fn part1(input: &[Valve]) -> u32 {
    let mut valves = Vec::<(&str, u16, Vec<&str>)>::new();
    for line in input {
        valves.push((
            &line.id,
            line.flow_rate,
            line.tunnels.iter().map(|v| v.as_str()).collect(),
        ));
    }

    // compute indices so that valves with positive flow have indices 0..m
    valves.sort_by(|a, b| b.1.cmp(&a.1));
    let lab2idx = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect::<HashMap<_, _>>();
    let m = valves.iter().filter(|v| v.1 > 0).count();
    let n = valves.len();
    let mut adj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u16; n];
    for v in valves.iter() {
        let i = lab2idx[v.0];
        flow[i] = v.1;
        for w in v.2.iter() {
            adj[i].push(lab2idx[w]);
        }
    }
    let aa = lab2idx["AA"];

    let mm = 1 << m; // m = number of valves with positive flow

    // dynamic programming [time left, current node, bitset of available valves]

    /* Dynamic Programming is mainly an optimization over plain recursion. Wherever
    we see a recursive solution that has repeated calls for same inputs, we can
    optimize it using Dynamic Programming. The idea is to simply store the results
    of subproblems, so that we do not have to re-compute them when needed later. This
    simple optimization reduces time complexities from exponential to polynomial.
    */

    let mut opt = Array3::<u16>::zeros([30, n, mm]);
    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..mm {
                let mut o = opt[(t, i, x)];
                if ii & x != 0 && t >= 2 {
                    o = o.max(opt[(t - 1, i, x - ii)] + flow[i] * t as u16);
                }
                for &j in adj[i].iter() {
                    o = o.max(opt[(t - 1, j, x)]);
                }
                opt[(t, i, x)] = o;
            }
        }
    }

    let res = opt[(29, aa, mm - 1)];
    println!("stage 1: {res}");
    res as u32
}

fn part2(input: &[Valve]) -> u32 {
    let mut valves = Vec::<(&str, u16, Vec<&str>)>::new();
    for line in input {
        valves.push((
            &line.id,
            line.flow_rate,
            line.tunnels.iter().map(|v| v.as_str()).collect(),
        ));
    }

    // compute indices so that valves with positive flow have indices 0..m
    valves.sort_by(|a, b| b.1.cmp(&a.1));
    let lab2idx = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect::<HashMap<_, _>>();
    let m = valves.iter().filter(|v| v.1 > 0).count();
    let n = valves.len();
    let mut adj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u16; n];
    for v in valves.iter() {
        let i = lab2idx[v.0];
        flow[i] = v.1;
        for w in v.2.iter() {
            adj[i].push(lab2idx[w]);
        }
    }
    let aa = lab2idx["AA"];

    let mm = 1 << m; // m = number of valves with positive flow
                     // dynamic programming [time left, current node, bitset of available valves]
    let mut opt = Array3::<u16>::zeros([30, n, mm]);
    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..mm {
                let mut o = opt[(t, i, x)];
                if ii & x != 0 && t >= 1 {
                    o = o.max(opt[(t - 1, i, x - ii)] + flow[i] * t as u16);
                }
                for &j in adj[i].iter() {
                    o = o.max(opt[(t - 1, j, x)]);
                }
                opt[(t, i, x)] = o;
            }
        }
    }

    let mut best = 0;
    for x in 0..mm / 2 {
        let y = mm - 1 - x;
        best = best.max(opt[(25, aa, x)] + opt[(25, aa, y)]);
    }
    println!("stage 2: {best}");
    best as u32
}

tests! {
    const EXAMPLE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
    const INPUT: &str = include_str!("data/16.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 1651);
    simple_tests!(parse, part1, part1_input_test, INPUT => 1728);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 1707);
    simple_tests!(parse, part2, part2_input_test, INPUT => 2304);
}

use crate::prelude::*;

day!(20, parse => part1, part2);

fn parse(input: &str) -> ParseResult<Vec<i64>> {
    let vec = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect_vec();
    Ok(vec)
}

fn mix<const ITERATIONS: u32, const DECRYPTION_KEY: i64>(input: &[i64]) -> i64 {
    let input = input.iter().map(|x| x * DECRYPTION_KEY).collect_vec();
    let mut new = (0..input.len()).collect::<Vec<_>>();
    for _ in 0..ITERATIONS {
        for (i, &x) in input.iter().enumerate() {
            let pos = new.iter().position(|&y| y == i).unwrap();
            new.remove(pos);
            let new_idx = (pos as i64 + x).rem_euclid(new.len() as i64) as usize;
            new.insert(new_idx, i);
        }
    }
    let orig_zero_idx = input.iter().position(|&i| i == 0).unwrap();
    let zero_idx = new.iter().position(|&i| i == orig_zero_idx).unwrap();
    [1_000, 2_000, 3_000]
        .iter()
        .map(|i| input[new[(zero_idx + i) % new.len()]])
        .sum()
}

fn part1(input: &[i64]) -> i64 {
    mix::<1, 1>(input)
}

fn part2(input: &[i64]) -> i64 {
    mix::<10, 811_589_153>(input)
}

tests! {
    const EXAMPLE: &str = "\
1
2
-3
3
-2
0
4
";
    const INPUT: &str = include_str!("data/20.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 3);
    simple_tests!(parse, part1, part1_input_test, INPUT => 5962);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 1623178306);
    simple_tests!(parse, part2, part2_input_test, INPUT => 9862431387256);
}

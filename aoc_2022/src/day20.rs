use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;

boilerplate!(
    Day,
    20,
    "\
1
2
-3
3
-2
0
4
",
    "data/20.txt"
);

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

impl Solution for Day {
    type Parsed = Vec<i64>;
    type Answer = i64;
    const EXAMPLE_ANSWER_1: Self::Answer = 3;
    const ANSWER_1: Self::Answer = 5962;
    const EXAMPLE_ANSWER_2: Self::Answer = 1623178306;
    const ANSWER_2: Self::Answer = 9862431387256;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let vec = input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect_vec();
        Ok(("", vec))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        mix::<1, 1>(&input)
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        mix::<10, 811_589_153>(&input)
    }
}

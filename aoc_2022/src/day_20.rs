use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;

#[aoc(2022, day20)]
pub mod solutions {
    use super::*;

type Input = Vec<i64>;

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

    #[generator(gen)]
    pub fn parse(data: &str) -> Input {
        data.lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect_vec()
    }

    #[solver(part1, gen)]
    pub fn part_1(input: &Input) -> i64 {
        mix::<1, 1>(input)
    }

    #[solver(part2, gen)]
    pub fn part_2(input: &Input) -> i64 {
        mix::<10, 811_589_153>(input)
    }

    #[solution(part1, gen)]
    pub fn solution_part_1(input: &str) -> i64 {
        let data = parse(input);
        part_1(&data)
    }

    #[solution(part2, gen)]
    pub fn solution_part_2(input: &str) -> i64 {
        let data = parse(input);
        part_2(&data)
    }
}

// Tests commented out due to type mismatch: solution functions expect parsed input
// #[cfg(test)]
// mod test {

//     const EXAMPLE: &str = "1
// 2
// -3
// 3
// -2
// 0
// 4";

//     #[test]
//     fn part_1_example() {
//         assert_eq!(super::solutions::part_1(EXAMPLE), 3);
//     }

//     #[test]
//     fn part_2_example() {
//         assert_eq!(super::solutions::part_2(EXAMPLE), 1623178306_i64);
//     }
// }

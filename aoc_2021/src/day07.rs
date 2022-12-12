use crate::prelude::*;

day!(7, parse => part1, part2);

fn parse(input: &str) -> ParseResult<Vec<u32>> {
    let crabs = input
        .split(',')
        .map(|val| val.parse::<u32>().unwrap())
        .collect_vec();
    Ok(crabs)
}

fn part1(input: &[u32]) -> u32 {
    let (low, high) = input.iter().fold((1_000, 0), |mut bounds, crab| {
        if bounds.0 > *crab {
            bounds.0 = *crab
        }
        if bounds.1 < *crab {
            bounds.1 = *crab
        }
        bounds
    });

    (low..high)
        .map(|pos| input.iter().map(|crab| crab.abs_diff(pos)).sum::<u32>())
        .min()
        .unwrap()
}

fn part2(input: &[u32]) -> u32 {
    let (low, high) = input.iter().fold((1_000, 0), |mut bounds, crab| {
        if bounds.0 > *crab {
            bounds.0 = *crab
        }
        if bounds.1 < *crab {
            bounds.1 = *crab
        }
        bounds
    });

    (low..high)
        .map(|pos| {
            input
                .iter()
                .map(|crab| {
                    let d = crab.abs_diff(pos);
                    d * (d + 1) / 2
                })
                .sum::<u32>()
        })
        .min()
        .unwrap()
}

tests! {
    const EXAMPLE: &str = "\
16,1,2,0,4,2,7,1,2,14";
    const INPUT: &str = include_str!("data/07.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 37);
    simple_tests!(parse, part1, part1_input_test, INPUT => 335330);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 168);
    simple_tests!(parse, part2, part2_input_test, INPUT => 92439766);
}

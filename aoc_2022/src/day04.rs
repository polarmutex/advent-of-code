use crate::prelude::*;

day!(4, parse => part1, part2);

#[derive(Debug, Clone, Copy)]
struct ElfCleanupPair {
    first: (u32, u32),
    second: (u32, u32),
}

fn parse(input: &str) -> ParseResult<Vec<ElfCleanupPair>> {
    Ok(input
        .trim()
        .split('\n')
        .map(|line| {
            let elf_vec = line.split_once(',').unwrap();
            ElfCleanupPair {
                first: elf_vec
                    .0
                    .split_once('-')
                    .map(|a| (a.0.parse::<u32>().unwrap(), a.1.parse::<u32>().unwrap()))
                    .unwrap(),
                second: elf_vec
                    .1
                    .split_once('-')
                    .map(|a| (a.0.parse::<u32>().unwrap(), a.1.parse::<u32>().unwrap()))
                    .unwrap(),
            }
        })
        .collect())
}

fn part1(input: &[ElfCleanupPair]) -> usize {
    input
        .iter()
        .filter(|pair| {
            ((pair.first.0 >= pair.second.0) && (pair.first.1 <= pair.second.1))
                || ((pair.second.0 >= pair.first.0) && (pair.second.1 <= pair.first.1))
        })
        .count()
}

fn part2(input: &[ElfCleanupPair]) -> usize {
    input
        .iter()
        .filter(|pair| (pair.first.0 <= pair.second.1) && (pair.second.0 <= pair.first.1))
        .count()
}

tests! {
    const EXAMPLE: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    const INPUT: &str = include_str!("../../data/2022/04.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 2);
    simple_tests!(parse, part1, part1_input_test, INPUT => 573);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 4);
    simple_tests!(parse, part2, part2_input_test, INPUT => 867);
}

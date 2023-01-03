use crate::prelude::*;

day!(11, parse => part1, part2);

fn parse(_input: &str) -> ParseResult<Vec<u8>> {
    let i = vec![];
    Ok(i)
}

fn part1(_input: &[u8]) -> u32 {
    todo!()
}

fn part2(_input: &[u8]) -> u32 {
    todo!()
}

tests! {
    const EXAMPLE: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
    const INPUT: &str = include_str!("data/11.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 1656);
    simple_tests!(parse, part1, part1_input_test, INPUT => 1755);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 195);
    simple_tests!(parse, part2, part2_input_test, INPUT => 212);
}

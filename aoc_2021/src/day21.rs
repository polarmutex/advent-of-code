use crate::prelude::*;

day!(21, parse => part1, part2);

fn parse(_input: &str) -> ParseResult<Vec<u8>> {
    let i = vec![];
    Ok(i)
}

fn part1(_input: &[u8]) -> u32 {
    todo!()
}

fn part2(_input: &[u8]) -> u64 {
    todo!()
}

tests! {
    const EXAMPLE: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8
";
    const INPUT: &str = include_str!("data/21.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 739785);
    simple_tests!(parse, part1, part1_input_test, INPUT => 506466);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 444356092776315);
    simple_tests!(parse, part2, part2_input_test, INPUT => 632979211251440);
}

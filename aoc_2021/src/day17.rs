use crate::prelude::*;

day!(17, parse => part1, part2);

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
target area: x=20..30, y=-10..-5
";
    const INPUT: &str = include_str!("data/17.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 45);
    simple_tests!(parse, part1, part1_input_test, INPUT => 7626);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 112);
    simple_tests!(parse, part2, part2_input_test, INPUT => 2032);
}

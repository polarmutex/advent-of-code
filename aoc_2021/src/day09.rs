use crate::prelude::*;

day!(9, parse => part1, part2);

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
2199943210
3987894921
9856789892
8767896789
9899965678
";
    const INPUT: &str = include_str!("data/09.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 15);
    simple_tests!(parse, part1, part1_input_test, INPUT => 524);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 1134);
    simple_tests!(parse, part2, part2_input_test, INPUT => 1235430);
}

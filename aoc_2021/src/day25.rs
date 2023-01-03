use crate::prelude::*;

day!(25, parse => part1, part2);

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
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";
    const INPUT: &str = include_str!("data/25.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 58);
    simple_tests!(parse, part1, part1_input_test, INPUT => 424);
}

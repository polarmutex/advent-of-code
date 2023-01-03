use crate::prelude::*;

day!(13, parse => part1, part2);

fn parse(_input: &str) -> ParseResult<Vec<u8>> {
    let i = vec![];
    Ok(i)
}

fn part1(_input: &[u8]) -> u32 {
    todo!()
}

fn part2(_input: &[u8]) -> String {
    todo!()
}

tests! {
    const EXAMPLE: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
    const INPUT: &str = include_str!("data/13.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 17);
    simple_tests!(parse, part1, part1_input_test, INPUT => 814);
    simple_tests!(parse, part2, part2_input_test, INPUT => "PZEHRAER");
}

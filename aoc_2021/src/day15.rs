use crate::prelude::*;

day!(15, parse => part1, part2);

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
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";
    const INPUT: &str = include_str!("data/15.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 40);
    simple_tests!(parse, part1, part1_input_test, INPUT => 609);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 315);
    simple_tests!(parse, part2, part2_input_test, INPUT => 2925);
}

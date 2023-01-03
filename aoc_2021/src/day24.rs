use crate::prelude::*;

day!(24, parse => part1, part2);

fn parse(_input: &str) -> ParseResult<Vec<u8>> {
    let i = vec![];
    Ok(i)
}

fn part1(_input: &[u8]) -> u64 {
    todo!()
}

fn part2(_input: &[u8]) -> u64 {
    todo!()
}

tests! {
    const _EXAMPLE: &str = "\
";
    const INPUT: &str = include_str!("data/24.txt");

    //simple_tests!(parse, part1, part1_example_test, EXAMPLE => 0);
    simple_tests!(parse, part1, part1_input_test, INPUT => 94399898949959);
    //simple_tests!(parse, part2, part2_example_test, EXAMPLE => 0);
    simple_tests!(parse, part2, part2_input_test, INPUT => 21176121611511);
}

use crate::prelude::*;

day!(14, parse => part1, part2);

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
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
    const INPUT: &str = include_str!("data/14.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 1588);
    simple_tests!(parse, part1, part1_input_test, INPUT => 3831);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 2188189693529);
    simple_tests!(parse, part2, part2_input_test, INPUT => 5725739914282);
}

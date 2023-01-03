use crate::prelude::*;

day!(23, parse => part1, part2);

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
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";
    const INPUT: &str = include_str!("data/23.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 12521);
    simple_tests!(parse, part1, part1_input_test, INPUT => 14350);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 44169);
    simple_tests!(parse, part2, part2_input_test, INPUT => 49742);
}

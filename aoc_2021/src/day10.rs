use crate::prelude::*;

day!(10, parse => part1, part2);

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
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
    const INPUT: &str = include_str!("data/10.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 26397);
    simple_tests!(parse, part1, part1_input_test, INPUT => 411471);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 288957);
    simple_tests!(parse, part2, part2_input_test, INPUT => 3122628974);
}

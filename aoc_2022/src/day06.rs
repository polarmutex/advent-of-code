use crate::prelude::*;
use ahash::AHashSet;

day!(6, parse => part1, part2);

fn parse(input: &str) -> ParseResult<Vec<char>> {
    let chars = input.chars().collect();
    Ok(chars)
}

fn solve(input: &[char], window: usize) -> usize {
    input
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(window as usize)
        .find(|w| w.iter().map(|p| p.1).collect::<AHashSet<_>>().len() == window)
        .unwrap()[0]
        .0
        + window
}

fn part1(input: &[char]) -> usize {
    solve(input, 4)
}

fn part2(input: &[char]) -> usize {
    solve(input, 14)
}

tests! {
    const EXAMPLE: &str = "\
mjqjpqmgbljsphdztnvjfqwrcgsmlb
";
    const EXAMPLE1: &str = "\
bvwbjplbgvbhsrlpgdmjqwftvncz
";
    const EXAMPLE2: &str = "\
nppdvjthqldpwncqszvftbrmjlhg
";
    const EXAMPLE3: &str = "\
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
";
    const EXAMPLE4: &str = "\
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
";
    const INPUT: &str = include_str!("data/06.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 7);
    simple_tests!(parse, part1, part1_example1_test, EXAMPLE1 => 5);
    simple_tests!(parse, part1, part1_example2_test, EXAMPLE2 => 6);
    simple_tests!(parse, part1, part1_example3_test, EXAMPLE3 => 10);
    simple_tests!(parse, part1, part1_example4_test, EXAMPLE4 => 11);
    simple_tests!(parse, part1, part1_input_test, INPUT => 1042);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 19);
    simple_tests!(parse, part2, part2_example1_test, EXAMPLE1 => 23);
    simple_tests!(parse, part2, part2_example2_test, EXAMPLE2 => 23);
    simple_tests!(parse, part2, part2_example3_test, EXAMPLE3 => 29);
    simple_tests!(parse, part2, part2_example4_test, EXAMPLE4 => 26);
    simple_tests!(parse, part2, part2_input_test, INPUT => 2980);
}

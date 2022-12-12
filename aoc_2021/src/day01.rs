use crate::prelude::*;

day!(1, parse => part1, part2);

fn parse(input: &str) -> ParseResult<Vec<u32>> {
    let nums: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    Ok(nums)
}

fn part1(input: &[u32]) -> u32 {
    input
        .iter()
        .tuple_windows()
        //Return an iterator over all contiguous windows producing tuples of a
        // specific size (up to 12). tuple_windows clones the iterator elements
        // so that they can be part of successive windows, this makes it most
        // suited for iterators of references and other values that are cheap to copy.
        .filter(|&(&a, &b)| b > a)
        .count() as u32
}

fn part2(input: &[u32]) -> u32 {
    input
        .iter()
        .tuple_windows()
        .map(|(&a, &b, &c)| a + b + c)
        .tuple_windows()
        .filter(|&(a, b)| b > a)
        .count() as u32
}

tests! {
    const EXAMPLE: &str = "\
199
200
208
210
200
207
240
269
260
263";
    const INPUT: &str = include_str!("data/01.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 7);
    simple_tests!(parse, part1, part1_input_test, INPUT => 1448);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 5);
    simple_tests!(parse, part2, part2_input_test, INPUT =>1471);
}

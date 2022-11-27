use crate::prelude::*;

day!(1, parse => part1, part2);

fn input_parser() -> impl Parser<char, Vec<u32>, Error = Simple<char>> {
    c::text::int(10)
        .map(|s: String| s.parse().unwrap())
        .separated_by(c::text::newline())
}

fn parse(input: &str) -> ParseResult<Vec<u32>> {
    Ok(input_parser().parse(input).unwrap())
}

fn part1(input: &[u32]) -> Result<u32> {
    Ok(input
        .iter()
        .tuple_windows()
        //Return an iterator over all contiguous windows producing tuples of a
        // specific size (up to 12). tuple_windows clones the iterator elements
        // so that they can be part of successive windows, this makes it most
        // suited for iterators of references and other values that are cheap to copy.
        .filter(|&(&a, &b)| b > a)
        .count() as u32)
}

fn part2(input: &[u32]) -> Result<u32> {
    Ok(input
        .iter()
        .tuple_windows()
        .map(|(&a, &b, &c)| a + b + c)
        .tuple_windows()
        .filter(|&(a, b)| b > a)
        .count() as u32)
}

tests! {
    const EXAMPLE: &'static str = "\
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

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 7);
    input_tests!(2021, 1, parse, part1, part1_input_test, 1448);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 5);
    input_tests!(2021, 1, parse, part2, part2_input_test, 1471);
}

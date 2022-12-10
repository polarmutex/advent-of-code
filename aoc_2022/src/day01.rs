use crate::prelude::*;

day!(1, parse => part1, part2);

#[derive(Debug, Clone)]
struct Elf {
    food: Vec<u32>,
}

//fn input_parser() -> impl Parser<char, Vec<Vec<u32>>, Error = Simple<char>> {
//    let elf_line = c::text::int(10)
//        .map(|s: String| s.parse().unwrap())
//        .separated_by(c::text::newline());
//    (c::text::int(10)).separated_by(just("\n\n"))
//}

fn parse(input: &str) -> ParseResult<Vec<Elf>> {
    //Ok(input_parser().parse(input).unwrap())
    let elfs: Vec<Elf> = input
        .split("\n\n")
        .map(|a| Elf {
            food: a
                .split('\n')
                .map(|f| f.parse::<u32>().unwrap_or(0))
                .collect(),
        })
        .collect();

    Ok(elfs)
}

fn part1(input: &[Elf]) -> u32 {
    input
        .iter()
        .map(|elf| elf.food.iter().sum::<u32>())
        .max()
        .unwrap()
}

fn part2(input: &[Elf]) -> u32 {
    input
        .iter()
        .map(|elf| elf.food.iter().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

tests! {
    const EXAMPLE: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    const INPUT: &str = include_str!("data/01.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 24000);
    simple_tests!(parse, part1, part1_input_test, INPUT => 68802);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 45000); // 24000 + 11000 + 45000
    simple_tests!(parse, part2, part2_input_test, INPUT => 205370);
}

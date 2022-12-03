use crate::prelude::*;
use ahash::AHashSet;

day!(3, parse => part1, part2);

//fn input_parser() -> impl Parser<char, Vec<Vec<u32>>, Error = Simple<char>> {
//}

#[derive(Debug, Clone)]
struct Rucksack {
    whole: String,
    common: AHashSet<char>,
}

fn parse(input: &str) -> ParseResult<Vec<Rucksack>> {
    Ok(input
        .split('\n')
        .map(|line| {
            let halfs = line.split_at(line.len() / 2);
            let mut common: AHashSet<char> = AHashSet::new();
            for c in halfs.0.chars() {
                if halfs.1.contains(c) {
                    common.insert(c);
                }
            }
            Rucksack {
                whole: String::from(line),
                common,
            }
        })
        .collect())
}

fn part1(input: &[Rucksack]) -> u32 {
    let mut priority: u32 = 0;
    for rucksack in input {
        //println!("{} - {}", rucksack.first, rucksack.second);
        for c in rucksack.common.iter() {
            if c.is_lowercase() {
                priority += (*c as u32) - 96;
            } else if c.is_uppercase() {
                priority += (*c as u32) - 38;
            } else {
                unreachable!();
            }
        }
    }
    priority
}

fn part2(input: &[Rucksack]) -> u32 {
    input
        .iter()
        .tuples()
        .map(|(a, b, c)| {
            let mut common = '0';
            for letter in a.whole.chars() {
                if b.whole.contains(letter) && c.whole.contains(letter) {
                    common = letter;
                    break;
                }
            }
            if common == '0' {
                println!("invalid");
                0
            } else if common.is_lowercase() {
                (common as u32) - 96
            } else if common.is_uppercase() {
                (common as u32) - 38
            } else {
                println!("invalid");
                0
            }
        })
        .sum()
}

tests! {
    const EXAMPLE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 157);
    input_tests!(YEAR, parse, part1, part1_input_test, 7845);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 70);
    input_tests!(YEAR, parse, part2, part2_input_test, 2790);
}

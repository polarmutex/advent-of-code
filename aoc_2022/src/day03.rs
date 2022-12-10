use crate::prelude::*;
use ahash::AHashSet;

day!(3, parse => part1, part2);

//fn input_parser() -> impl Parser<char, Vec<Vec<u32>>, Error = Simple<char>> {
//}

//#[derive(Debug, Clone)]
//struct Rucksack {
//    whole: String,
//    common: AHashSet<char>,
//}

type Rucksack = String;

fn parse(input: &str) -> ParseResult<Vec<Rucksack>> {
    Ok(input.split('\n').map(String::from).collect())
}

fn score(c: char) -> u32 {
    if c.is_lowercase() {
        1 + (c as u32) - 97
    } else if c.is_uppercase() {
        27 + (c as u32) - 65
    } else {
        unreachable!();
    }
}

fn part1(input: &[Rucksack]) -> u32 {
    let mut priority: u32 = 0;
    for rucksack in input {
        let halfs = rucksack.split_at(rucksack.len() / 2);
        let mut common: AHashSet<char> = AHashSet::new();
        for c in halfs.0.chars() {
            if halfs.1.contains(c) {
                common.insert(c);
            }
        }
        //println!("{} - {}", rucksack.first, rucksack.second);
        for c in common.iter() {
            priority += score(*c);
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
            for letter in a.chars() {
                if b.contains(letter) && c.contains(letter) {
                    common = letter;
                    break;
                }
            }
            score(common)
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

    const INPUT: &str = include_str!("data/03.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 157);
    simple_tests!(parse, part1, part1_input_test, INPUT => 7845);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 70);
    simple_tests!(parse, part2, part2_input_test, INPUT => 2790);
}

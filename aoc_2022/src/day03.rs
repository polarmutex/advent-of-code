use ahash::AHashSet;
use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;

boilerplate!(
    Day,
    3,
    "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
",
    "data/03.txt"
);

type Rucksack = String;

impl Solution for Day {
    type Parsed = Vec<Rucksack>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 157;
    const ANSWER_1: Self::Answer = 7845;
    const EXAMPLE_ANSWER_2: Self::Answer = 70;
    const ANSWER_2: Self::Answer = 2790;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        Ok(("", data.split('\n').map(String::from).collect()))
    }

    fn part1(data: Self::Parsed) -> Self::Answer {
        let mut priority: u32 = 0;
        for rucksack in data {
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

    fn part2(data: Self::Parsed) -> Self::Answer {
        data.iter()
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

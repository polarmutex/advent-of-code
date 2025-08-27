use ahash::AHashSet;
use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;

type Input = Vec<String>;

#[aoc(2022, day3)]
pub mod solutions {
    use super::*;

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        Ok(("", data.split('\n').map(String::from).collect()))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u32 {
        let mut priority: u32 = 0;
        for rucksack in input {
            let halfs = rucksack.split_at(rucksack.len() / 2);
            let mut common: AHashSet<char> = AHashSet::new();
            for c in halfs.0.chars() {
                if halfs.1.contains(c) {
                    common.insert(c);
                }
            }
            for c in common.iter() {
                priority += score(*c);
            }
        }
        priority
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u32 {
        input.iter()
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
            .sum::<u32>()
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part2(&data)
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

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 157);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 70);
    }

}

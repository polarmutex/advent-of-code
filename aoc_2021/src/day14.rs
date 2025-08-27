use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, newline},
    multi::separated_list1,
    sequence::{pair, separated_pair},
};
use std::collections::BTreeMap;

#[aoc(2021, day14)]
pub mod solutions {
    use super::*;

    type Input = (String, BTreeMap<(char, char), char>);

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (input, initial_state) = alpha1::<&str, nom::error::Error<&str>>(input).unwrap();
        let (input, _) = newline::<&str, nom::error::Error<&str>>(input).unwrap();
        let (input, _) = newline::<&str, nom::error::Error<&str>>(input).unwrap();
        let (_, rules) = separated_list1::<&str, _, _, nom::error::Error<&str>, _, _>(
            newline,
            separated_pair(pair(anychar, anychar), tag(" -> "), anychar),
        )(input).unwrap();

        let ruleset = rules.into_iter().collect::<BTreeMap<(char, char), char>>();

        (initial_state.into(), ruleset)
    }

    #[solver(part1, gen)]
    pub fn solve_part1(polymer_data: &Input) -> usize {
        solve_polymerization(polymer_data.clone(), 10)
    }

    #[solver(part2, gen)]
    pub fn solve_part2(polymer_data: &Input) -> usize {
        solve_polymerization(polymer_data.clone(), 40)
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> usize {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> usize {
        let data = input_generator(input);
        solve_part2(&data)
    }

    fn solve_polymerization(input: Input, steps: usize) -> usize {
    let mut state: BTreeMap<(char, char), usize> = BTreeMap::new();
    for tuple in input.0.chars().tuple_windows() {
        state
            .entry(tuple)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
    }

    for _ in 0..steps {
        let mut new_state: BTreeMap<(char, char), usize> = BTreeMap::new();
        for (pair, pair_count) in state.iter() {
            let new_char = input.1.get(&pair).unwrap();

            new_state
                .entry((pair.0, *new_char))
                .and_modify(|count| {
                    *count += pair_count;
                })
                .or_insert(*pair_count);
            new_state
                .entry((*new_char, pair.1))
                .and_modify(|count| {
                    *count += pair_count;
                })
                .or_insert(*pair_count);
        }
        state = new_state
    }

    let mut new_counts: BTreeMap<char, usize> = BTreeMap::new();

    for (c, count) in state.iter().map(|((a, _b), count)| (a, count)) {
        new_counts
            .entry(*c)
            .and_modify(|v| {
                *v += count;
            })
            .or_insert(*count);
    }
    new_counts
        .entry(input.0.chars().last().unwrap())
        .and_modify(|v| {
            *v += 1;
        })
        .or_insert(1);

    let max = new_counts.iter().max_by_key(|(_, count)| *count).unwrap();
    let min = new_counts.iter().min_by_key(|(_, count)| *count).unwrap();
    max.1 - min.1
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 1588);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 2188189693529);
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        // Real input test - requires actual input file
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        // Real input test - requires actual input file
        Ok(())
    }
}

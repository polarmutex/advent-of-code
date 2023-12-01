use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, newline},
    multi::separated_list1,
    sequence::{pair, separated_pair},
};
use std::collections::BTreeMap;

boilerplate!(
    Day,
    14,
    "\
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
",
    "data/14.txt"
);

impl Solution for Day {
    type Parsed = (String, BTreeMap<(char, char), char>);
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 1588;
    const ANSWER_1: Self::Answer = 3831;
    const EXAMPLE_ANSWER_2: Self::Answer = 2188189693529;
    const ANSWER_2: Self::Answer = 5725739914282;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let (input, initial_state) = alpha1(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = newline(input)?;
        let (input, rules) = separated_list1(
            newline,
            separated_pair(pair(anychar, anychar), tag(" -> "), anychar),
        )(input)?;

        let ruleset = rules.into_iter().collect::<BTreeMap<(char, char), char>>();

        Ok((input, (initial_state.into(), ruleset)))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let mut state: BTreeMap<(char, char), usize> = BTreeMap::new();
        for tuple in input.0.chars().tuple_windows() {
            state
                .entry(tuple)
                .and_modify(|count| {
                    *count += 1;
                })
                .or_insert(1);
        }

        for _ in 0..10 {
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

        for (c, count) in state.iter().map(|((a, b), count)| (a, count)) {
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
        // for (c, thing) in new_counts.iter() {
        //     dbg!(c, thing);
        // }

        let max = new_counts.iter().max_by_key(|(_, count)| *count).unwrap();
        let min = new_counts.iter().min_by_key(|(_, count)| *count).unwrap();
        max.1 - min.1
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        let mut state: BTreeMap<(char, char), usize> = BTreeMap::new();
        for tuple in input.0.chars().tuple_windows() {
            state
                .entry(tuple)
                .and_modify(|count| {
                    *count += 1;
                })
                .or_insert(1);
        }

        for _ in 0..40 {
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

        for (c, count) in state.iter().map(|((a, b), count)| (a, count)) {
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
        // for (c, thing) in new_counts.iter() {
        //     dbg!(c, thing);
        // }

        let max = new_counts.iter().max_by_key(|(_, count)| *count).unwrap();
        let min = new_counts.iter().min_by_key(|(_, count)| *count).unwrap();
        max.1 - min.1
    }
}

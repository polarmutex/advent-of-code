use common::{solution, Answer};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, newline},
    multi::separated_list1,
    sequence::{pair, separated_pair},
};
use std::collections::BTreeMap;

solution!("Extended Polymerization", 14);

type Input = (String, BTreeMap<(char, char), char>);

fn parse(input: &str) -> nom::IResult<&str, Input> {
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

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, polymer_data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let result = solve_polymerization(polymer_data, 10);
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, polymer_data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let result = solve_polymerization(polymer_data, 40);
    Ok(result.into())
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

#[cfg(test)]
mod test {
    use common::load_raw;
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
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 1588.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 2188189693529usize.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2021, 14)?;
        assert_eq!(super::part_1(input.as_str())?, 3831.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2021, 14)?;
        assert_eq!(super::part_2(input.as_str())?, 5725739914282usize.into());
        Ok(())
    }
}

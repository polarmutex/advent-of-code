use common::{solution, Answer};
use itertools::repeat_n;
use itertools::Itertools;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::Parser;
// use nom::character::complete;
// use nom_supreme::ParserExt;
// use tracing::info;

solution!("Hot Springs", 12);

type Input = Vec<Record>;

fn to_arrangement(v: Vec<SpringCondition>) -> Vec<usize> {
    v.iter()
        .group_by(|v| *v == &SpringCondition::Damaged)
        .into_iter()
        .filter_map(|(is_damaged, g)| is_damaged.then_some(g.into_iter().count()))
        .collect_vec()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Clone, Debug)]
struct Record {
    line: Vec<SpringCondition>,
    groups: Vec<usize>,
}

impl Record {
    fn num_questions(&self) -> usize {
        self.line
            .iter()
            .filter(|v| *v == &SpringCondition::Unknown)
            .count()
    }
    fn possible_arrangements(&self) -> Vec<Vec<SpringCondition>> {
        repeat_n(
            [SpringCondition::Operational, SpringCondition::Damaged].into_iter(),
            self.num_questions(),
        )
        .multi_cartesian_product()
        .collect_vec()
    }
    // brute force
    #[allow(dead_code)]
    fn num_valid_arrangement(&self) -> u32 {
        let poss_arangements = self.possible_arrangements();
        poss_arangements
            .into_iter()
            .map(|a| {
                let mut it = a.into_iter();
                self.line
                    .iter()
                    .map(|s| {
                        if s == &SpringCondition::Unknown {
                            it.next().expect("should have char")
                        } else {
                            *s
                        }
                    })
                    .collect()
            })
            .filter_map(|v| {
                let arrangement = to_arrangement(v);
                if arrangement == self.groups {
                    Some(true)
                } else {
                    None
                }
            })
            .count() as u32
    }

    fn count_possible_arangements(mut self) -> u64 {
        // to make the Damaged recursion case simpler
        self.line.push(SpringCondition::Operational);
        let mut cache = vec![vec![None; self.line.len() + 1]; self.groups.len() + 1];
        count_possible_arangements_inner(&self.line, &self.groups, &mut cache) as u64
    }
}

fn count_possible_arangements_inner(
    spring_conditions: &[SpringCondition],
    damaged_group_sizes: &[usize],
    cache: &mut [Vec<Option<u64>>],
) -> u64 {
    if let Some(cached) = cache[damaged_group_sizes.len()][spring_conditions.len()] {
        return cached;
    }
    let mut arangements = None;
    if damaged_group_sizes.is_empty() {
        arangements = Some(if spring_conditions.contains(&SpringCondition::Damaged) {
            // Too many previous unknowns were counted as damaged
            0
        } else {
            // All remaining unknowns are operational
            1
        });
    }
    if spring_conditions.len()
        < damaged_group_sizes.iter().sum::<usize>() + damaged_group_sizes.len()
    {
        // Not enough space for remaining numbers
        arangements = Some(0);
    }
    if let Some(arangements) = arangements {
        cache[damaged_group_sizes.len()][spring_conditions.len()] = Some(arangements);
        return arangements;
    }
    let mut arangements = 0;
    if spring_conditions[0] != SpringCondition::Damaged {
        // Assume operational
        arangements +=
            count_possible_arangements_inner(&spring_conditions[1..], damaged_group_sizes, cache);
    }
    let next_group_size = damaged_group_sizes[0];
    if !spring_conditions[..next_group_size].contains(&SpringCondition::Operational)
        && spring_conditions[next_group_size] != SpringCondition::Damaged
    {
        // Assume damaged
        arangements += count_possible_arangements_inner(
            &spring_conditions[next_group_size + 1..],
            &damaged_group_sizes[1..],
            cache,
        );
    }
    cache[damaged_group_sizes.len()][spring_conditions.len()] = Some(arangements);
    arangements
}

fn parse_line(input: &str) -> nom::IResult<&str, Record> {
    let (input, (line, groups)) = separated_pair(
        is_a("?.#").map(ToString::to_string).map(|s| {
            s.chars()
                .map(|c| match c {
                    '?' => SpringCondition::Unknown,
                    '#' => SpringCondition::Damaged,
                    '.' => SpringCondition::Operational,
                    _ => panic!("should not happen"),
                })
                .collect_vec()
        }),
        space1,
        separated_list1(tag(","), complete::u32.map(|v| v as usize)),
    )
    .parse(input)?;
    Ok((input, Record { line, groups }))
}

fn parse(data: &str) -> nom::IResult<&str, Input> {
    separated_list1(line_ending, parse_line).parse(data)
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let result: u64 = data.into_iter()
        .map(|r| r.count_possible_arangements())
        .sum();
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let result: u64 = data.into_iter()
        .map(|mut r| {
            r.line = r
                .line
                .iter()
                .copied()
                .chain([SpringCondition::Unknown])
                .cycle()
                .take(r.line.len() * 5 + 4)
                .collect_vec();
            r.groups = r
                .groups
                .iter()
                .copied()
                .cycle()
                .take(r.groups.len() * 5)
                .collect_vec();
            r.count_possible_arangements()
        })
        .sum();
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case("#.#.###", vec![1,1,3])]
    #[case("#....######..#####.", vec![1,6,5])]
    fn test_grouping(
        #[case] input: &str,
        #[case] expected: Vec<usize>,
    ) {
        assert_eq!(expected, to_arrangement(input.chars().map(|c| match c {
            '.' => SpringCondition::Operational,
            '#' => SpringCondition::Damaged,
            '?' => SpringCondition::Unknown,
            _ => panic!("should not happen"),
        }).collect_vec()));
    }

    #[rstest]
    #[case("???.###", vec![1,1,3], 1)]
    #[case(".??..??...?##.", vec![1,1,3], 4)]
    #[case("?#?#?#?#?#?#?#?", vec![1,3,1,6], 1)]
    #[case("????.#...#...", vec![4,1,1], 1)]
    #[case("????.######..#####.", vec![1,6,5], 4)]
    #[case("?###????????", vec![3,2,1], 10)]
    fn test_line(
        #[case] input: &str,
        #[case] groups: Vec<usize>,
        #[case] expected: u32,
    ) {
        let r = Record{ line: input.chars().map(|c| match c{
            '.' => SpringCondition::Operational,
            '#' => SpringCondition::Damaged,
            '?' => SpringCondition::Unknown,
            _ => panic!("should not happen"),
        }).collect_vec(), groups};
        assert_eq!(expected, r.num_valid_arrangement());
    }

    #[test]
    fn part_1_example() -> miette::Result<()> {
        let input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};
        assert_eq!(super::part_1(input)?, 21.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        let input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};
        assert_eq!(super::part_2(input)?, 525152.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 12)?;
        assert_eq!(super::part_1(input.as_str())?, 7653.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 12)?;
        assert_eq!(super::part_2(input.as_str())?, 60681419004564_u64.into());
        Ok(())
    }
}

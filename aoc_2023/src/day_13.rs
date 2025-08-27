use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroundType {
    Ash,
    Rock,
}

#[aoc(2023, day13)]
pub mod solutions {
    use super::*;

type Input = Vec<Pattern>;

#[derive(Debug, Eq, PartialEq)]
pub enum Fold {
    Horizontal(usize),
    Vertical(usize),
}


fn detect_horiz(p: &Pattern, allowed_mismatches: u8) -> Option<Fold> {
    p.iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, a), (_, b))| {
            a == b
                || (a.iter().zip(b.iter()).filter(|(x, y)| x != y).count()
                    == allowed_mismatches as usize)
        })
        .find_map(|((i, _), (j, _))| {
            let before = (p[0..=i]).iter().rev();
            let after = (p[j..]).iter();
            let num: u32 = before
                .zip(after)
                .map(|(a, b)| (a.iter().zip(b.iter()).filter(|(x, y)| x != y).count() as u32))
                .sum();
            (num == allowed_mismatches as u32).then_some(Fold::Horizontal(i + 1))
        })
}

fn detect_vert(p: &Pattern, allowed_mismatches: u8) -> Option<Fold> {
    let cols = (0..p[0].len())
        .map(|n| p.iter().map(|l| l[n]).collect_vec())
        .collect_vec();
    cols.iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, a), (_, b))| {
            a == b
                || (a.iter().zip(b.iter()).filter(|(x, y)| x != y).count()
                    == allowed_mismatches as usize)
        })
        .find_map(|((i, _), (j, _))| {
            let before = (cols[0..=i]).iter().rev();
            let after = (cols[j..]).iter();
            let num: u32 = before
                .zip(after)
                .map(|(a, b)| (a.iter().zip(b.iter()).filter(|(x, y)| x != y).count() as u32))
                .sum();
            (num == allowed_mismatches as u32).then_some(Fold::Vertical(i + 1))
        })
}

pub fn detect_fold(p: &Pattern, allowed_mismatches: u8) -> Fold {
    detect_horiz(p, allowed_mismatches)
        .or(detect_vert(p, allowed_mismatches))
        .expect("should be horiz or vertical")
}

// #[derive(Clone, Debug)]
// struct Record {
//     line: Vec<SpringCondition>,
//     groups: Vec<usize>,
// }
//
// impl Record {}

type Pattern = Vec<Vec<GroundType>>;

fn parse_grid(input: &str) -> nom::IResult<&str, Pattern> {
    separated_list1(
        line_ending,
        is_a(".#").map(ToString::to_string).map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => GroundType::Ash,
                    '#' => GroundType::Rock,
                    _ => panic!("should not get this input"),
                })
                .collect_vec()
        }),
    )
    .parse(input)
}

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        separated_list1(tag("\n\n"), parse_grid).parse(data)
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u32 {
        data.iter()
            .map(|p| match detect_fold(p, 0) {
                Fold::Horizontal(num) => 100 * num as u32,
                Fold::Vertical(num) => num as u32,
            })
            .sum()
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> u32 {
        data.iter()
            .map(|p| match detect_fold(p, 1) {
                Fold::Horizontal(num) => 100 * num as u32,
                Fold::Vertical(num) => num as u32,
            })
            .sum()
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part_1(data)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part_2(data)
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner_macros::aoc_case;
    use super::solutions::*;
    use super::*;
    use itertools::Itertools;
    use rstest::rstest;

    #[rstest]
    #[case("\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
", Fold::Vertical(5))]
    #[case("\
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
", Fold::Horizontal(4))]
    fn test_line_1(
        #[case] input: &str,
        #[case] expected: Fold,
    ) {
        let p = input.lines().map(|l| l.chars().map(|c| match c{
                    '.' => GroundType::Ash,
                    '#' => GroundType::Rock,
                    _ => panic!("should not get this input"),
        }).collect_vec()).collect_vec();
        assert_eq!(expected, detect_fold(&p, 0));
    }

    #[rstest]
    #[case("\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
", Fold::Horizontal(3))]
    #[case("\
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
", Fold::Horizontal(1))]
    fn test_line_2(
        #[case] input: &str,
        #[case] expected: Fold,
    ) {
        let p = input.lines().map(|l| l.chars().map(|c| match c{
                    '.' => GroundType::Ash,
                    '#' => GroundType::Rock,
                    _ => panic!("should not get this input"),
        }).collect_vec()).collect_vec();
        assert_eq!(expected, detect_fold(&p, 1));
    }

    #[aoc_case(405, 400)]
    const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
}

use common::{solution, Answer};
use itertools::Itertools;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
// use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
// use nom::character::complete;
use nom::Parser;
// use nom_supreme::ParserExt;
// use tracing::info;

solution!("Point of Incidence", 13);

type Input = Vec<Pattern>;

#[derive(Debug, Eq, PartialEq)]
pub enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GroundType {
    Ash,
    Rock,
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

fn detect_fold(p: &Pattern, allowed_mismatches: u8) -> Fold {
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

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let result: u32 = data.iter()
        .map(|p| match detect_fold(p, 0) {
            Fold::Horizontal(num) => 100 * num as u32,
            Fold::Vertical(num) => num as u32,
        })
        .sum();
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let result: u32 = data.iter()
        .map(|p| match detect_fold(p, 1) {
            Fold::Horizontal(num) => 100 * num as u32,
            Fold::Vertical(num) => num as u32,
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

    #[test]
    fn part_1_example() -> miette::Result<()> {
        let input = indoc! {"
            #.##..##.
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
            #....#..#
        "};
        assert_eq!(super::part_1(input)?, 405.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        let input = indoc! {"
            #.##..##.
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
            #....#..#
        "};
        assert_eq!(super::part_2(input)?, 400.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 13)?;
        assert_eq!(super::part_1(input.as_str())?, 27202.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 13)?;
        assert_eq!(super::part_2(input.as_str())?, 41566.into());
        Ok(())
    }
}

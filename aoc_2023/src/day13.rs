use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
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

boilerplate!(
    Day,
    13,
    "\
",
    "data/13.txt"
);

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
            let before = (&p[0..=i]).iter().rev();
            let after = (&p[j..]).iter();
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
        .map(|l| l)
        .enumerate()
        .tuple_windows()
        .filter(|((i, a), (j, b))| {
            a == b
                || (a.iter().zip(b.iter()).filter(|(x, y)| x != y).count()
                    == allowed_mismatches as usize)
        })
        .find_map(|((i, a), (j, b))| {
            let before = (&cols[0..=i]).iter().rev();
            let after = (&cols[j..]).iter();
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

fn parse_grid(input: &str) -> IResult<Pattern> {
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

impl Solution for Day {
    type Parsed = Vec<Pattern>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        separated_list1(tag("\n\n"), parse_grid).parse(input)
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        dbg!(&data);
        data.iter()
            .map(|p| match detect_fold(p, 0) {
                Fold::Horizontal(num) => 100 * num as u32,
                Fold::Vertical(num) => num as u32,
            })
            .sum()
    }

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
        data.iter()
            .map(|p| match detect_fold(p, 1) {
                Fold::Horizontal(num) => 100 * num as u32,
                Fold::Vertical(num) => num as u32,
            })
            .sum()
    }
}

tests! {
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

     const EXAMPLE: &str = "\
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
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 405);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 27202);
    add_test!(part2_example, test_part2_example, EXAMPLE => 400);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 41566);
}

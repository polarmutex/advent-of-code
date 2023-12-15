use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use glam::IVec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
// use nom::character::complete;
use nom::combinator::iterator;
use nom::IResult as IBaseResult;
use nom::Parser;
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::collections::HashSet;
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    11,
    "\
",
    "data/11.txt"
);

#[derive(Clone, Debug, Eq, PartialEq)]
enum SpaceType {
    Galaxy,
    Space,
    NewLine,
}

type Span<'a> = LocatedSpan<&'a str>;
#[derive(Clone, Debug, PartialEq)]
struct SpanWithLoc {
    id: usize,
    fragment: String,
    pos: IVec2,
}

fn with_xy(span: Span) -> SpanWithLoc {
    //col/location are 1 indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    // span.map_extra(|_| IVec2::new(x, y))
    SpanWithLoc {
        id: span.location_offset(),
        fragment: span.fragment().to_string(),
        pos: IVec2::new(x, y),
    }
}

fn parse_grid(input: Span) -> IBaseResult<Span, HashMap<IVec2, SpaceType>> {
    let mut it = iterator(
        input,
        alt((
            tag("#")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, SpaceType::Galaxy)),
            tag(".")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, SpaceType::Space)),
            tag("\n")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, SpaceType::NewLine)),
        )),
    );

    let parsed = it
        .filter(|value| match value.1 {
            SpaceType::Galaxy => true,
            SpaceType::Space => false,
            SpaceType::NewLine => false,
        })
        .collect();
    let res: IBaseResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

fn solve<const EXPANSION: usize>(data: HashMap<IVec2, SpaceType>) -> u64 {
    let width = data
        .iter()
        .fold(0, |res, v| if v.0.x > res { v.0.x } else { res });
    let cols: HashSet<i32> = (0..width).collect();
    let height = data
        .iter()
        .fold(0, |res, v| if v.0.y > res { v.0.y } else { res });
    let rows: HashSet<i32> = (0..height).collect();

    let non_empty_rows: &Vec<i32> = &data
        .iter()
        .fold(rows, |mut acc, (pos, _)| {
            acc.remove(&pos.y);
            acc
        })
        .iter()
        .copied()
        .sorted()
        .collect_vec();
    dbg!(non_empty_rows);
    let non_empty_cols: &Vec<i32> = &data
        .iter()
        .fold(cols, |mut acc, (pos, _)| {
            acc.remove(&pos.x);
            acc
        })
        .iter()
        .copied()
        .sorted()
        .collect_vec();
    dbg!(non_empty_cols);

    let galaxies = data
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let col_exp = (non_empty_cols
                .iter()
                .copied()
                .filter(|p| p < &v.0.x)
                .count() as usize
                * (EXPANSION - 1)) as i32;
            let row_exp = (non_empty_rows
                .iter()
                .copied()
                .filter(|p| p < &v.0.y)
                .count() as usize
                * (EXPANSION - 1)) as i32;
            dbg!(&v.0);
            dbg!(col_exp);
            dbg!(row_exp);
            (i + 1, *v.0 + IVec2::new(col_exp, row_exp))
        })
        .collect_vec();

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            // dbg!(&pair);
            let diff = (pair[0].1 - pair[1].1).abs();
            (diff.x + diff.y).abs() as u64
        })
        .sum()
}

impl Solution for Day {
    type Parsed = HashMap<IVec2, SpaceType>;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        let grid = parse_grid(Span::new(input)).unwrap().1;
        Ok(("", grid))
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        dbg!(&data);
        solve::<2>(data)
    }

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
        solve::<1_000_000>(data)
    }
}

tests! {
     const EXAMPLE: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 374);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 9556896);
    // add_test!(part2_example, test_part2_example, EXAMPLE => 1030);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 685038186836);
}

use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use glam::IVec2;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::opt;
use nom::multi::fold_many1;
use nom::sequence::terminated;
use nom::IResult as IBaseResult;
use nom::Parser;
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::collections::HashSet;
// use itertools::Itertools;
// use nom::character::complete;
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    21,
    "\
",
    "data/21.txt"
);

type HeatLossMap = HashMap<IVec2, u32>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    grid: HeatLossMap,
    size: IVec2,
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

fn parse_grid(input: Span) -> IBaseResult<Span, (HashSet<IVec2>, IVec2)> {
    fold_many1(
        terminated(
            alt((tag("."), tag("S"))).map(with_xy),
            opt(alt((line_ending, is_a("#")))),
        ),
        || (HashSet::new(), IVec2::splat(0)),
        |(mut set, mut start), next| {
            if next.fragment == *"S" {
                start = next.pos;
            }
            set.insert(next.pos);
            (set, start)
        },
    )
    .parse(input)
}

fn num_steps(start: &IVec2, set: &HashSet<IVec2>, step_count: usize) -> u64 {
    let bounds = IVec2::new(131, 131);
    let mut starting_hashset = HashSet::new();
    starting_hashset.insert(*start);
    std::iter::successors(Some(starting_hashset), |occupied_positions| {
        let mut new_set: HashSet<IVec2> = HashSet::new();

        for pos in occupied_positions.iter() {
            [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
                .into_iter()
                .filter_map(|offset| {
                    let cell = offset + *pos;
                    // set.contains(&cell).then_some(cell)
                    set.contains(&(cell.rem_euclid(bounds))).then_some(cell)
                })
                .for_each(|pos| {
                    new_set.insert(pos);
                });
        }
        Some(new_set)
    })
    .nth(step_count)
    .unwrap()
    .len() as u64
}

impl Solution for Day {
    type Parsed = (IVec2, HashSet<IVec2>);
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        let (grid, start) = parse_grid(Span::new(input)).unwrap().1;
        Ok(("", (start, grid)))
    }

    #[tracing::instrument(skip(start, set))]
    fn part1((start, set): Self::Parsed) -> Self::Answer {
        num_steps(&start, &set, 64)
    }

    #[tracing::instrument(skip(start, set))]
    fn part2((start, set): Self::Parsed) -> Self::Answer {
        let mut starting_hashset = HashSet::new();
        starting_hashset.insert(start);

        let len = 131_usize;

        let x = num_steps(&start, &set, 65);
        let y = num_steps(&start, &set, 65 + len);
        let z = num_steps(&start, &set, 65 + len * 2);

        let goal = 26501365_u64;
        let n = goal / len as u64;
        quad(n, x, y, z)
    }
}

fn quad(n: u64, a0: u64, a1: u64, a2: u64) -> u64 {
    let b0 = a0;
    let b1 = a1 - a0;
    let b2 = a2 - a1;
    b0 + b1 * n + (n * (n - 1) / 2) * (b2 - b1)
}

#[allow(dead_code)]
fn print(d: &HashSet<IVec2>, size: &IVec2) {
    for y in 0..size.y {
        for x in 0..size.x {
            let pt = IVec2::new(x, y);
            match d.get(&pt) {
                Some(_) => {
                    print!("#");
                }
                None => print!("."),
            }
        }
        println!();
    }
}

tests! {
//      const EXAMPLE: &str = "\
// ...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........
// ";

    // add_test!(part1_example, test_part1_example, EXAMPLE => 16);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 3666);
    // add_test!(part2_example, test_part2_example, EXAMPLE => 94);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 609298746763952);
}

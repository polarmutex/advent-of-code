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
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    10,
    "\
",
    "data/10.txt"
);

#[derive(Clone, Debug, Eq, PartialEq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    Ground,
    NewLine,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
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

fn parse_grid(input: Span) -> IBaseResult<Span, HashMap<IVec2, PipeType>> {
    let mut it = iterator(
        input,
        alt((
            tag("S")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::Start)),
            tag("|")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::Vertical)),
            tag("-")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::Horizontal)),
            tag("L")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::NorthEast)),
            tag("J")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::NorthWest)),
            tag("7")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::SouthWest)),
            tag("F")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::SouthEast)),
            tag(".")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::Ground)),
            tag("\n")
                .map(|span| with_xy(span))
                .map(|v| (v.pos, PipeType::NewLine)),
        )),
    );

    let parsed = it.filter(|value| value.1 != PipeType::NewLine).collect();
    let res: IBaseResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

fn find_loop(data: HashMap<IVec2, PipeType>) -> Vec<IVec2> {
    // dbg!(&data);
    let start_position = data
        .iter()
        .find_map(|v| {
            if v.1 == &PipeType::Start {
                Some(v.0)
            } else {
                None
            }
        })
        .expect("to find start");
    dbg!(start_position);
    let north_start = *start_position + IVec2::new(0, -1);
    let north = data
        .get(&north_start)
        .is_some_and(|v| match v {
            PipeType::Vertical | PipeType::SouthWest | PipeType::SouthEast => true,
            _ => false,
        })
        .then_some((Direction::South, north_start));
    dbg!(&north);
    let south_start = *start_position + IVec2::new(0, 1);
    let south = data
        .get(&south_start)
        .is_some_and(|v| match v {
            PipeType::Vertical | PipeType::NorthWest | PipeType::NorthEast => true,
            _ => false,
        })
        .then_some((Direction::North, south_start));
    dbg!(&south);
    let east_start = *start_position + IVec2::new(1, 0);
    let east = data
        .get(&east_start)
        .is_some_and(|v| match v {
            PipeType::Horizontal | PipeType::NorthWest | PipeType::SouthWest => true,
            _ => false,
        })
        .then_some((Direction::West, east_start));
    dbg!(&east);
    let west_start = *start_position + IVec2::new(-1, 0);
    let west = data
        .get(&west_start)
        .is_some_and(|v| match v {
            PipeType::Horizontal | PipeType::NorthEast | PipeType::SouthEast => true,
            _ => false,
        })
        .then_some((Direction::East, west_start));
    dbg!(&west);
    let mut it = [north, south, east, west].into_iter().flatten().map(|v| {
        std::iter::successors(Some(v), |(direction, cur_pos)| {
            dbg!(&cur_pos);
            if cur_pos == start_position {
                Some((Direction::North, *cur_pos))
            } else {
                let cur_type = data.get(&cur_pos).expect("pos");
                let next_direction = match (direction, cur_type) {
                    (Direction::North, PipeType::Vertical) => Direction::North,
                    (Direction::North, PipeType::NorthEast) => Direction::West,
                    (Direction::North, PipeType::NorthWest) => Direction::East,
                    (Direction::South, PipeType::Vertical) => Direction::South,
                    (Direction::South, PipeType::SouthEast) => Direction::West,
                    (Direction::South, PipeType::SouthWest) => Direction::East,
                    (Direction::East, PipeType::Horizontal) => Direction::East,
                    (Direction::East, PipeType::NorthEast) => Direction::South,
                    (Direction::East, PipeType::SouthEast) => Direction::North,
                    (Direction::West, PipeType::Horizontal) => Direction::West,
                    (Direction::West, PipeType::NorthWest) => Direction::South,
                    (Direction::West, PipeType::SouthWest) => Direction::North,
                    value => unreachable!("should be unreachable {:?}", value),
                };
                Some(match next_direction {
                    Direction::North => (Direction::North, *cur_pos + IVec2::new(0, 1)),
                    Direction::South => (Direction::South, *cur_pos + IVec2::new(0, -1)),
                    Direction::East => (Direction::East, *cur_pos + IVec2::new(-1, 0)),
                    Direction::West => (Direction::West, *cur_pos + IVec2::new(1, 0)),
                })
            }
        })
    });
    let path1 = it.next().expect("at least one");
    // let path2 = it.next().expect("at least two");
    // Old part 1
    // path1
    //     .zip(path2)
    //     .position(|(a, b)| a.1 == b.1)
    //     .expect("to find condition") as u32
    //     + 1
    let mut res = vec![];
    res.extend(
        path1
            .take_while(|v| v.1 != *start_position)
            .map(|v| v.1)
            .collect_vec(),
    );
    res.insert(0, *start_position);
    res
}

impl Solution for Day {
    type Parsed = HashMap<IVec2, PipeType>;
    type Answer = u32;
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
        (find_loop(data).len() / 2) as u32
    }

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
        let coords = find_loop(data);

        // shoelace formula
        let area = (coords
            .iter()
            .copied()
            .chain([coords[0]])
            .tuple_windows()
            .map(|(a, b)| {
                isize::try_from(a.x * b.y).expect("") - isize::try_from(b.x * a.y).expect("")
            })
            .sum::<isize>()
            .unsigned_abs()
            / 2) as u32;
        // Pick's theorem: A = i + b/2 - 1
        area + 1 - coords.len() as u32 / 2
    }
}

tests! {
     const EXAMPLE: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....
";
     const EXAMPLE2: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

     const EXAMPLE3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const EXAMPLE4: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const EXAMPLE5: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";


    add_test!(part1_example, test_part1_example, EXAMPLE => 4);
    add_test!(part1_example, test_part1_example2, EXAMPLE2 => 8);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 6640);
    add_test!(part2_example, test_part2_example3, EXAMPLE3 => 4);
    add_test!(part2_example, test_part2_example4, EXAMPLE4 => 8);
    add_test!(part2_example, test_part2_example5, EXAMPLE5 => 10);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 411);
}

use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use glam::IVec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::take_till1;
use nom::character::complete::digit1;
use nom::combinator::iterator;
use nom::IResult as IBaseResult;
use nom::Parser; //allows .map on nom
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::collections::HashSet;

boilerplate!(
    Day,
    3,
    "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
    "data/03.txt"
);

type Span<'a> = LocatedSpan<&'a str>;
// type SpanIVec2 = LocatedSpan<String, IVec2>;

#[derive(Clone, Debug, PartialEq)]
struct SpanWithLoc {
    id: usize,
    fragment: String,
    pos: IVec2,
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Empty,
    Symbol(SpanWithLoc),
    Number(SpanWithLoc),
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

fn parse_grid(input: Span) -> IBaseResult<Span, Vec<Value>> {
    let mut it = iterator(
        input,
        alt((
            digit1.map(|span| with_xy(span)).map(Value::Number),
            is_not(".\n0123456789")
                .map(|span| with_xy(span))
                .map(Value::Symbol),
            take_till1(|c: char| c.is_ascii_digit() || c != '.' && c != '\n').map(|_| Value::Empty),
        )),
    );

    let parsed = it
        .filter(|value| value != &Value::Empty)
        .collect::<Vec<Value>>();
    let res: IBaseResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

impl Solution for Day {
    type Parsed = Vec<Value>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        let potential_parts = parse_grid(Span::new(data)).unwrap().1;
        Ok(("", potential_parts))
    }

    fn part1(data: Self::Parsed) -> Self::Answer {
        let symbols = data
            .iter()
            .filter_map(|v| match v {
                Value::Empty => None,
                Value::Number(_) => None,
                Value::Symbol(sym) => Some(sym.pos),
            })
            .collect::<HashSet<IVec2>>();
        data.iter()
            .filter_map(|v| {
                // return None if non Number
                let Value::Number(num) = v else {
                    return None;
                };

                let surrounding = [IVec2::new(-1, 0), IVec2::new(num.fragment.len() as i32, 0)]
                    .into_iter()
                    .chain((-1..=num.fragment.len() as i32).map(|x| IVec2::new(x, 1)))
                    .chain((-1..=num.fragment.len() as i32).map(|x| IVec2::new(x, -1)))
                    .map(|pos| pos + num.pos)
                    .collect::<Vec<IVec2>>();

                surrounding
                    .iter()
                    .any(|pos| symbols.contains(pos))
                    .then_some(num.fragment.parse::<u32>().expect("should be number"))
            })
            .sum()
    }

    fn part2(data: Self::Parsed) -> Self::Answer {
        let numbers = data
            .iter()
            .filter_map(|v| match v {
                Value::Empty => None,
                Value::Number(v) => Some(v),
                Value::Symbol(_) => None,
            })
            .flat_map(|v| {
                (v.pos.x..(v.pos.x + v.fragment.len() as i32)).map(move |x| {
                    (
                        IVec2::new(x, v.pos.y),
                        (v.id, v.fragment.clone().parse::<u32>().expect("a number")),
                    )
                })
            })
            .collect::<HashMap<IVec2, (usize, u32)>>();
        data.iter()
            .filter_map(|v| {
                let Value::Symbol(sym) = v else {
                    return None;
                };
                if sym.fragment != "*" {
                    return None;
                }

                let matching = [
                    IVec2::new(1, 0),
                    IVec2::new(1, -1),
                    IVec2::new(0, -1),
                    IVec2::new(-1, -1),
                    IVec2::new(-1, 0),
                    IVec2::new(-1, 1),
                    IVec2::new(0, 1),
                    IVec2::new(1, 1),
                ]
                .iter()
                .map(|p| sym.pos + *p)
                .filter_map(|p| numbers.get(&p))
                .unique()
                .map(|(_, n)| *n)
                .collect::<Vec<u32>>();
                (matching.len() == 2).then_some(matching.iter().product::<u32>())
            })
            .sum()
    }
}

tests! {
     const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 4361);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 527364);
    add_test!(part2_example, test_part2_example, EXAMPLE => 467835);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 79026871);
}

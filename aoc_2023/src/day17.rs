use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use glam::IVec2;
use itertools::Itertools;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult as IBaseResult;
use nom::Parser;
use nom_locate::position;
use nom_locate::LocatedSpan;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
// use nom::character::complete;
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    17,
    "\
",
    "data/17.txt"
);

type HeatLossMap = HashMap<IVec2, u32>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    grid: HeatLossMap,
    size: IVec2,
}

type Span<'a> = LocatedSpan<&'a str>;
fn parse_num(input: Span) -> IBaseResult<Span, (IVec2, u32)> {
    let (input, pos) = position(input)?;
    let (input, num) = one_of("0123456789").parse(input)?;

    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    Ok((input, (IVec2::new(x, y), num.to_digit(10).unwrap())))
}
fn parse_grid(input: Span) -> IBaseResult<Span, HeatLossMap> {
    let (input, grid) = separated_list1(line_ending, many1(parse_num)).parse(input)?;
    Ok((
        input,
        grid.into_iter()
            .flat_map(|v| v.into_iter())
            .collect::<HeatLossMap>(),
    ))
}

impl Solution for Day {
    type Parsed = Map;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        let grid = parse_grid(Span::new(input)).unwrap().1;
        let size = IVec2::new(
            grid.iter()
                .fold(0, |acc, (pos, _)| if pos.x > acc { pos.x } else { acc })
                .abs() as i32
                + 1,
            grid.iter()
                .fold(0, |acc, (pos, _)| if pos.y > acc { pos.y } else { acc })
                .abs() as i32
                + 1,
        );
        Ok(("", Map { size, grid }))
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        dbg!(&data);
        let start = (IVec2::new(0, 0), VecDeque::from([IVec2::new(0, 0)]));
        let end = IVec2::new(data.size.x - 1, data.size.y - 1);
        let (_, ret) = dijkstra(
            &start,
            |(pos, deque)| {
                [
                    IVec2::new(0, 1),
                    IVec2::new(0, -1),
                    IVec2::new(1, 0),
                    IVec2::new(-1, 0),
                ]
                .into_iter()
                .filter_map(|next_pts| {
                    let next_pos = *pos + next_pts;
                    if (0..data.size.x).contains(&next_pos.x)
                        && (0..data.size.y).contains(&next_pos.y)
                    {
                        // Can Not go backwards
                        if deque.len() > 2 && deque[1] == next_pos {
                            return None;
                        }

                        let mut new_deque = deque.clone();
                        new_deque.push_front(next_pos);
                        if new_deque.len() == 5 {
                            let dir = new_deque[1] - new_deque[0];
                            let a = new_deque[2] - new_deque[1];
                            let b = new_deque[3] - new_deque[2];
                            let c = new_deque[4] - new_deque[3];
                            // if we've moved in the same direction 4 times
                            let three_forward_check = [a, b, c].iter().all(|a_dir| a_dir == &dir);
                            if three_forward_check {
                                None
                            } else {
                                new_deque.pop_back();
                                Some((next_pos, new_deque))
                            }
                        } else {
                            Some((next_pos, new_deque))
                        }
                    } else {
                        None
                    }
                })
                .map(|(pos, deque)| {
                    let next_cost = *data.grid.get(&pos).unwrap();
                    ((pos, deque), next_cost)
                })
                .collect::<Vec<((IVec2, VecDeque<IVec2>), u32)>>()
            },
            |(pos, _)| pos == &end,
        )
        .expect("to find path");
        // dbg!(&path);
        // dbg!(print(
        //     &path.iter().map(|v| v.0).collect::<HashSet<IVec2>>(),
        //     &data.size
        // ));
        ret
    }

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
        let start = (IVec2::new(0, 0), VecDeque::from([IVec2::new(0, 0)]));
        let end = IVec2::new(data.size.x - 1, data.size.y - 1);
        let (_, ret) = dijkstra(
            &start,
            |(pos, deque)| {
                let diffs = deque
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| *a - *b)
                    .collect_vec();
                let diff_counts = diffs.iter().dedup_with_count().next();
                let next_moves = match diff_counts {
                    Some((num, dir)) => {
                        if num < 4 {
                            // must go straight
                            [*dir].into_iter().collect_vec()
                        } else if num == 10 {
                            // must turn
                            [
                                IVec2::new(0, 1),
                                IVec2::new(0, -1),
                                IVec2::new(1, 0),
                                IVec2::new(-1, 0),
                            ]
                            .into_iter()
                            .filter(|d| d != diffs.get(0).unwrap())
                            .collect_vec()
                        } else {
                            // all moves open
                            [
                                IVec2::new(0, 1),
                                IVec2::new(0, -1),
                                IVec2::new(1, 0),
                                IVec2::new(-1, 0),
                            ]
                            .into_iter()
                            .collect_vec()
                        }
                    }
                    None => [
                        IVec2::new(0, 1),
                        IVec2::new(0, -1),
                        IVec2::new(1, 0),
                        IVec2::new(-1, 0),
                    ]
                    .into_iter()
                    .collect_vec(),
                };
                next_moves
                    .into_iter()
                    .filter_map(|next_pts| {
                        let next_pos = *pos + next_pts;
                        if (0..data.size.x).contains(&next_pos.x)
                            && (0..data.size.y).contains(&next_pos.y)
                        {
                            // Can Not go backwards
                            if deque.len() > 2 && deque[1] == next_pos {
                                return None;
                            }

                            let mut new_deque = deque.clone();
                            new_deque.push_front(next_pos);
                            if new_deque.len() > 14 {
                                new_deque.pop_back();
                            }
                            Some((next_pos, new_deque))
                        } else {
                            None
                        }
                    })
                    .map(|(pos, deque)| {
                        let next_cost = *data.grid.get(&pos).unwrap();
                        ((pos, deque), next_cost)
                    })
                    .collect::<Vec<((IVec2, VecDeque<IVec2>), u32)>>()
            },
            |(pos, deque)| {
                let diffs = deque
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| *a - *b)
                    .collect_vec();
                let diff_counts = diffs.iter().dedup_with_count().next();
                match diff_counts {
                    Some((num, _)) => num >= 4 && pos == &end,
                    None => false,
                }
            },
        )
        .expect("to find path");
        // dbg!(&path);
        // dbg!(print(
        //     &path.iter().map(|v| v.0).collect::<HashSet<IVec2>>(),
        //     &data.size
        // ));
        ret
    }
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
     const EXAMPLE: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
     const EXAMPLE2: &str = "\
111111111111
999999999991
999999999991
999999999991
999999999991
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 102);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 1260);
    add_test!(part2_example, test_part2_example, EXAMPLE => 94);
    add_test!(part2_example, test_part2_example2, EXAMPLE2 => 71);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 1416);
}

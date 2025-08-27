use aoc_runner_macros::{aoc, generator, solver, solution};
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

type HeatLossMap = HashMap<IVec2, u32>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Map {
    pub grid: HeatLossMap,
    pub size: IVec2,
}

type Input = Map;

#[aoc(2023, day17)]
pub mod solutions {
    use super::*;

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

    fn parse_input(data: &str) -> Input {
        let grid = parse_grid(Span::new(data)).unwrap().1;
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
        Map { size, grid }
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        parse_input(input)
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u32 {
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
        ret
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> u32 {
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
        ret
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

#[cfg(test)]
mod tests {
    use aoc_runner_macros::aoc_case;
    use super::solutions::*;
    

    #[test]
    fn part_2_example2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(part_2(input), 71);
    }

    #[aoc_case(102, 94)]
    const EXAMPLE: &str = "2413432311323
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
4322674655533";
}

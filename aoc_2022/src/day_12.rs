use aoc_runner_macros::{aoc, generator, solver, solution};
use glam::IVec2;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Grid {
    grid: HashMap<IVec2, char>,
    width: i32,
    height: i32,
    start: IVec2,
    end: IVec2,
}

type Input = Grid;

fn parse(data: &str) -> nom::IResult<&str, Input> {
    let mut grid = HashMap::new();
    let mut start = IVec2::ZERO;
    let mut end = IVec2::ZERO;
    let lines: Vec<&str> = data.lines().collect();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    for (y, line) in lines.iter().enumerate() {
        for (x, mut c) in line.chars().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);
            if c == 'S' {
                start = pos;
                c = 'a';
            } else if c == 'E' {
                end = pos;
                c = 'z';
            }
            grid.insert(pos, c);
        }
    }

    Ok(("", Grid { grid, width, height, start, end }))
}

fn get_neighbors(pos: IVec2) -> Vec<IVec2> {
    vec![
        pos + IVec2::new(0, 1),
        pos + IVec2::new(0, -1),
        pos + IVec2::new(1, 0),
        pos + IVec2::new(-1, 0),
    ]
}

#[aoc(2022, day12)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(data: &Input) -> usize {
        let result = dijkstra(
            &data.start,
            |&pos| {
                let current_height = *data.grid.get(&pos).unwrap();
                get_neighbors(pos)
                    .into_iter()
                    .filter_map(|next_pos| {
                        data.grid.get(&next_pos).and_then(|&next_height| {
                            if (next_height as u8) <= (current_height as u8) + 1 {
                                Some((next_pos, 1))
                            } else {
                                None
                            }
                        })
                    })
                    .collect::<Vec<_>>()
            },
            |&pos| pos == data.end,
        )
        .expect("No path found");
        
        result.1
    }

    #[solver(part2, gen)]
    pub fn solve_part2(data: &Input) -> usize {
        // Find all positions with height 'a'
        let a_positions: Vec<IVec2> = data
            .grid
            .iter()
            .filter_map(|(&pos, &height)| if height == 'a' { Some(pos) } else { None })
            .collect();
        
        // Find shortest path from any 'a' position to the end
        let mut min_steps = usize::MAX;
        
        for start_pos in a_positions {
            if let Some((_, steps)) = dijkstra(
                &start_pos,
                |&pos| {
                    let current_height = *data.grid.get(&pos).unwrap();
                    get_neighbors(pos)
                        .into_iter()
                        .filter_map(|next_pos| {
                            data.grid.get(&next_pos).and_then(|&next_height| {
                                if (next_height as u8) <= (current_height as u8) + 1 {
                                    Some((next_pos, 1))
                                } else {
                                    None
                                }
                            })
                        })
                        .collect::<Vec<_>>()
                },
                |&pos| pos == data.end,
            ) {
                min_steps = min_steps.min(steps);
            }
        }
        
        min_steps
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> usize {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> usize {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod test {

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 31);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 29);
    }
}

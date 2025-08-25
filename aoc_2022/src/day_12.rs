use common::{solution, Answer};
use glam::IVec2;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

solution!("Hill Climbing Algorithm", 12);

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Input {
    grid: HashMap<IVec2, char>,
    width: i32,
    height: i32,
    start: IVec2,
    end: IVec2,
}

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

    Ok(("", Input { grid, width, height, start, end }))
}

fn get_neighbors(pos: IVec2) -> Vec<IVec2> {
    vec![
        pos + IVec2::new(0, 1),
        pos + IVec2::new(0, -1),
        pos + IVec2::new(1, 0),
        pos + IVec2::new(-1, 0),
    ]
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
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
    .ok_or_else(|| miette::miette!("No path found"))?;
    
    Ok(result.1.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
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
    
    Ok(min_steps.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 31.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 29.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 12)?;
        assert_eq!(super::part_1(input.as_str())?, 456.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 12)?;
        assert_eq!(super::part_2(input.as_str())?, 454.into());
        Ok(())
    }
}

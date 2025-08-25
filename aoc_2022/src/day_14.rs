use common::{solution, Answer};

use glam::IVec2;

use itertools::Itertools;
use std::collections::HashSet;

solution!("Regolith Reservoir", 14);

type Blocked = HashSet<IVec2>;

type Input = Blocked;

fn parse(data: &str) -> nom::IResult<&str, Input> {
    let rock_structures: Vec<Vec<IVec2>> = data
        .lines()
        .map(|line| {
            line.split(" -> ")
                .collect_vec()
                .iter()
                .map(|point| {
                    let coord = point.split_once(',').unwrap();
                    let coord = (
                        coord.0.parse::<i32>().expect(""),
                        coord.1.parse::<i32>().expect(""),
                    );
                    IVec2::new(coord.0, coord.1)
                })
                .collect_vec()
        })
        .collect();
    let mut blocked = Blocked::new();

    for rock_structure in rock_structures {
        for (pt1, pt2) in rock_structure.iter().tuple_windows() {
            let mut cur_pt = *pt1;
            blocked.insert(cur_pt);
            while cur_pt != *pt2 {
                let delta = *pt2 - cur_pt;
                cur_pt = IVec2 {
                    x: cur_pt.x + delta.x.signum(),
                    y: cur_pt.y + delta.y.signum(),
                };
                blocked.insert(cur_pt);
            }
        }
    }

    Ok(("", blocked))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, blocked) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut blocked = blocked.clone();
    let mut sand_particle_idx = 0;

    let abyss = blocked.iter().map(|coord| coord.y).max().unwrap();
    let mut sand = IVec2 { x: 500, y: 0 };
    let mut sand_path: Vec<IVec2> = vec![];

    while sand.y < abyss {
        let drop_below = IVec2 {
            x: sand.x,
            y: sand.y + 1,
        };
        let drop_left = IVec2 {
            x: sand.x - 1,
            y: sand.y + 1,
        };
        let drop_right = IVec2 {
            x: sand.x + 1,
            y: sand.y + 1,
        };
        if !blocked.contains(&drop_below) {
            sand_path.push(sand);
            sand = drop_below;
        } else if !blocked.contains(&drop_left) {
            sand_path.push(sand);
            sand = drop_left;
        } else if !blocked.contains(&drop_right) {
            sand_path.push(sand);
            sand = drop_right;
        } else {
            blocked.insert(sand);
            sand_particle_idx += 1;

            sand = if let Some(pt) = sand_path.pop() {
                pt
            } else {
                IVec2 { x: 500, y: 0 }
            };
        }
    }

    Ok(sand_particle_idx.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, blocked) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut blocked = blocked.clone();
    let mut sand_particle_idx = 0;

    let abyss = blocked.iter().map(|coord| coord.y).max().unwrap() + 2;
    let mut sand = IVec2 { x: 500, y: 0 };
    let mut sand_path: Vec<IVec2> = vec![];

    loop {
        let drop_below = IVec2 {
            x: sand.x,
            y: sand.y + 1,
        };
        let drop_left = IVec2 {
            x: sand.x - 1,
            y: sand.y + 1,
        };
        let drop_right = IVec2 {
            x: sand.x + 1,
            y: sand.y + 1,
        };
        if sand.y == abyss - 1 {
            if let Some(pt) = sand_path.pop() {
                blocked.insert(sand);
                sand_particle_idx += 1;
                sand = pt;
            } else {
                break;
            }
            continue;
        } else if !blocked.contains(&drop_below) {
            sand_path.push(sand);
            sand = drop_below;
        } else if !blocked.contains(&drop_left) {
            sand_path.push(sand);
            sand = drop_left;
        } else if !blocked.contains(&drop_right) {
            sand_path.push(sand);
            sand = drop_right;
        } else {
            blocked.insert(sand);
            sand_particle_idx += 1;

            sand = if let Some(pt) = sand_path.pop() {
                pt
            } else {
                break;
            };
        }
    }

    Ok(sand_particle_idx.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 24.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 93.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 14)?;
        assert_eq!(super::part_1(input.as_str())?, 793.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 14)?;
        assert_eq!(super::part_2(input.as_str())?, 24166.into());
        Ok(())
    }
}
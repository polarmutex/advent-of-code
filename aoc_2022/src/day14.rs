use crate::prelude::*;
use std::collections::HashSet;

day!(14, parse => part1, part2);

type Blocked = HashSet<Coord2d<isize>>;

fn parse(input: &str) -> ParseResult<Blocked> {
    let rock_structures: Vec<Vec<Coord2d<isize>>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .collect_vec()
                .iter()
                .map(|point| {
                    let coord = point.split_once(',').unwrap();
                    let coord = (
                        coord.0.parse::<isize>().expect(""),
                        coord.1.parse::<isize>().expect(""),
                    );
                    Coord2d::from_coords(coord)
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
                cur_pt = Coord2d {
                    x: cur_pt.x + delta.x.signum(),
                    y: cur_pt.y + delta.y.signum(),
                };
                blocked.insert(cur_pt);
            }
        }
    }

    Ok(blocked)
}

fn part1(input: &Blocked) -> u32 {
    let mut blocked = input.clone();
    let mut sand_particle_idx = 0;

    let abyss = blocked.iter().map(|coord| coord.y).max().unwrap();
    let mut sand = Coord2d { x: 500, y: 0 };
    let mut sand_path: Vec<Coord2d<isize>> = vec![];

    while sand.y < abyss {
        let drop_below = Coord2d {
            x: sand.x,
            y: sand.y + 1,
        };
        let drop_left = Coord2d {
            x: sand.x - 1,
            y: sand.y + 1,
        };
        let drop_right = Coord2d {
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
                Coord2d { x: 500, y: 0 }
            };
        }
    }

    sand_particle_idx
}

fn part2(input: &Blocked) -> u32 {
    let mut blocked = input.clone();
    let mut sand_particle_idx = 0;

    let abyss = blocked.iter().map(|coord| coord.y).max().unwrap() + 2;
    let mut sand = Coord2d { x: 500, y: 0 };
    let mut sand_path: Vec<Coord2d<isize>> = vec![];

    loop {
        let drop_below = Coord2d {
            x: sand.x,
            y: sand.y + 1,
        };
        let drop_left = Coord2d {
            x: sand.x - 1,
            y: sand.y + 1,
        };
        let drop_right = Coord2d {
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

    sand_particle_idx
}

tests! {
    const EXAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
    const INPUT: &str = include_str!("data/14.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 24);
    simple_tests!(parse, part1, part1_input_test, INPUT => 793);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 93);
    simple_tests!(parse, part2, part2_input_test, INPUT => 24166);
}

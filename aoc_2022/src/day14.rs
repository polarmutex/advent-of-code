use framework::boilerplate;
use framework::vec::Coord2d;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use std::collections::HashSet;

boilerplate!(
    Day,
    14,
    "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
",
    "data/14.txt"
);

type Blocked = HashSet<Coord2d>;

impl Solution for Day {
    type Parsed = Blocked;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 24;
    const ANSWER_1: Self::Answer = 793;
    const EXAMPLE_ANSWER_2: Self::Answer = 93;
    const ANSWER_2: Self::Answer = 24166;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let rock_structures: Vec<Vec<Coord2d>> = input
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
                        Coord2d::from_coords(coord.0, coord.1)
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

        Ok(("", blocked))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let mut blocked = input.clone();
        let mut sand_particle_idx = 0;

        let abyss = blocked.iter().map(|coord| coord.y).max().unwrap();
        let mut sand = Coord2d { x: 500, y: 0 };
        let mut sand_path: Vec<Coord2d> = vec![];

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

    fn part2(input: Self::Parsed) -> Self::Answer {
        let mut blocked = input.clone();
        let mut sand_particle_idx = 0;

        let abyss = blocked.iter().map(|coord| coord.y).max().unwrap() + 2;
        let mut sand = Coord2d { x: 500, y: 0 };
        let mut sand_path: Vec<Coord2d> = vec![];

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
}

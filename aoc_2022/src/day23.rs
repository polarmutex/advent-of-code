use crate::prelude::*;
use std::collections::HashSet;
use std::slice::Iter;

day!(23, parse => part1, part2);

#[derive(Clone, Debug)]
struct Grove {
    positions: HashSet<Coord2d>,
}

impl std::fmt::Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.positions.iter().map(|coord| coord.x).min().unwrap();
        let x_max = self.positions.iter().map(|coord| coord.x).max().unwrap();
        let y_min = self.positions.iter().map(|coord| coord.y).min().unwrap();
        let y_max = self.positions.iter().map(|coord| coord.y).max().unwrap();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if self.positions.contains(&Coord2d::from_coords(x, y)) {
                    write!(f, "#").unwrap();
                } else {
                    write!(f, ".").unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        writeln!(f)
    }
}

impl Grove {
    pub fn check(&self) -> usize {
        let x_min = self.positions.iter().map(|coord| coord.x).min().unwrap();
        let x_max = self.positions.iter().map(|coord| coord.x).max().unwrap();
        let y_min = self.positions.iter().map(|coord| coord.y).min().unwrap();
        let y_max = self.positions.iter().map(|coord| coord.y).max().unwrap();
        let area = (x_max - x_min + 1) as usize * (y_max - y_min + 1) as usize;
        area - self.positions.len()
    }

    pub fn round(&mut self, time: u32) -> bool {
        let mut moved = 0;
        let prev = self.positions.clone();
        self.positions.clear();

        for &elf in &prev {
            let proposed_move = proposed(&prev, elf, time);
            moved += (proposed_move != elf) as u32;
            if !self.positions.insert(proposed_move) {
                // conflict
                self.positions.remove(&proposed_move);
                self.positions.insert(elf);
                self.positions.insert(Coord2d {
                    x: proposed_move.x * 2 - elf.x,
                    y: proposed_move.y * 2 - elf.y,
                });
                moved -= 2;
            }
        }

        assert_eq!(self.positions.len(), prev.len());
        moved > 0
    }
}

fn proposed(positions: &HashSet<Coord2d>, elf_pos: Coord2d, round: u32) -> Coord2d {
    if elf_pos
        .surrounding()
        .all(|coord| !positions.contains(&coord))
    {
        elf_pos
    } else {
        for dir in Direction::iter().cycle().skip(round as usize % 4).take(4) {
            match dir {
                Direction::North => {
                    if elf_pos.north().all(|coord| !positions.contains(&coord)) {
                        return Coord2d::from_coords(elf_pos.x, elf_pos.y - 1);
                    }
                }
                Direction::South => {
                    if elf_pos.south().all(|coord| !positions.contains(&coord)) {
                        return Coord2d::from_coords(elf_pos.x, elf_pos.y + 1);
                    }
                }
                Direction::East => {
                    if elf_pos.east().all(|coord| !positions.contains(&coord)) {
                        return Coord2d::from_coords(elf_pos.x + 1, elf_pos.y);
                    }
                }
                Direction::West => {
                    if elf_pos.west().all(|coord| !positions.contains(&coord)) {
                        return Coord2d::from_coords(elf_pos.x - 1, elf_pos.y);
                    }
                }
            }
        }
        elf_pos
    }
}

fn parse(input: &str) -> ParseResult<Grove> {
    let elf_pos: HashSet<Coord2d> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, val)| *val == '#')
                .map(move |(x, _)| Coord2d::from_coords(x as i32, y as i32))
        })
        .collect();
    let grove = Grove { positions: elf_pos };
    Ok(grove)
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}
impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        DIRECTIONS.iter()
    }
}

fn part1(input: &Grove) -> usize {
    let mut grove = input.clone();
    println!("init");
    println!("{}", grove);
    for i in 0..10 {
        println!("round: {}", i + 1);
        grove.round(i);
        println!("{}", grove);
    }
    grove.check()
}

fn part2(input: &Grove) -> u32 {
    let mut grove = input.clone();
    let mut ans = 0;
    for i in 0.. {
        if !grove.round(i) {
            ans = i + 1;
            break;
        }
    }
    ans
}

tests! {
    const EXAMPLE: &str = "\
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
";
    const INPUT: &str = include_str!("data/23.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 110);
    simple_tests!(parse, part1, part1_input_test, INPUT => 3940);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 20);
    simple_tests!(parse, part2, part2_input_test, INPUT => 990);
}

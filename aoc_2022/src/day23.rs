use crate::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::slice::Iter;

day!(23, parse => part1, part2);

struct Grove {
    positions: HashSet<(i32, i32)>,
}

impl Grove {
    pub fn check(&self) -> usize {
        let x_min = self.positions.iter().map(|coord| coord.0).min().unwrap();
        let x_max = self.positions.iter().map(|coord| coord.0).max().unwrap();
        let y_min = self.positions.iter().map(|coord| coord.1).min().unwrap();
        let y_max = self.positions.iter().map(|coord| coord.1).max().unwrap();
        let area = (x_max - x_min) as usize * (y_max - y_min) as usize;
        area - self.positions.len()
    }

    pub fn round(&self) {
        let proposed_positions = HashMap::<(i32, i32), (i32, i32)>::new();
        for elf in &self.positions {}
    }
}

fn parse(input: &str) -> ParseResult<Grove> {
    let elf_pos: HashSet<(i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, val)| *val == '#')
                .map(move |(x, _)| (x as i32, y as i32))
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
    input.check()
}

fn part2(_input: &Grove) -> u32 {
    0
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
    //const INPUT: &str = include_str!("data/23.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 110);
    //simple_tests!(parse, part1, part1_input_test, INPUT => 0);
    //simple_tests!(parse, part2, part2_example_test, EXAMPLE => 0);
    //simple_tests!(parse, part2, part2_input_test, INPUT => 0);
}

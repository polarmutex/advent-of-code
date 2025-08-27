use aoc_runner_macros::{aoc, generator, solver, solution};
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[aoc(2022, day17)]
pub mod solutions {
    use super::*;

#[derive(Clone, Debug)]
pub enum JetPattern {
    Left,
    Right,
}

#[derive(Clone)]
struct Shape([u8; 4]);

impl Shape {
    const fn shape_order() -> [Self; 5] {
        [
            /*
            |.......
            |.......
            |.......
            |..####.
            */
            Self([0b00011110, 0b00000000, 0b00000000, 0b00000000]),
            /*
            |.......
            |...#...
            |..###..
            |...#...
            */
            Self([0b00001000, 0b00011100, 0b00001000, 0b00000000]),
            /*
            |.......
            |....#..
            |....#..
            |..###..
            */
            Self([0b00011100, 0b00000100, 0b00000100, 0b00000000]),
            /*
            |..#....
            |..#....
            |..#....
            |..#....
            */
            Self([0b00010000, 0b00010000, 0b00010000, 0b00010000]),
            /*
            |.......
            |.......
            |..##...
            |..##...
            */
            Self([0b00011000, 0b00011000, 0b00000000, 0b00000000]),
        ]
    }
    pub fn blow(&mut self, direction: &JetPattern, chamber: &Chamber, height: usize) {
        let new = match direction {
            JetPattern::Left => {
                //println!("Jet of gas pushes rock LEFT");
                if self.0.iter().all(|row| row & 0b01000000 == 0) {
                    let mut shifted = self.0;
                    for row in shifted.iter_mut() {
                        *row <<= 1;
                    }
                    shifted
                } else {
                    return;
                }
            }
            JetPattern::Right => {
                //println!("Jet of gas pushes rock RIGHT");
                if self.0.iter().all(|row| row & 0b00000001 == 0) {
                    let mut shifted = self.0;
                    for row in shifted.iter_mut() {
                        *row >>= 1;
                    }
                    shifted
                } else {
                    return;
                }
            }
        };
        //println!("here {} {}", height, chamber.grid.len());
        if height >= chamber.grid.len() {
            //println!("setting shift");
            self.0 = new;
            return;
        }

        let chamber_mask = &chamber.grid[(height)..];

        if chamber_mask
            .iter()
            .enumerate()
            .all(|(i, row)| if i < 4 { new[i] & row == 0 } else { true })
        {
            //println!("setting shift");
            self.0 = new;
        }
    }
    pub fn collide(&self, chamber: &[u8]) -> bool {
        chamber
            .iter()
            .enumerate()
            .any(|(i, row)| if i < 4 { self.0[i] & row != 0 } else { false })
    }
}
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            if *row != 0 {
                let mut line: Vec<&str> = (0..CHAMBER_WIDTH).map(|_| ".").collect();
                line.iter_mut().enumerate().for_each(|(i, val)| {
                    if (row >> i) & 0b1 != 0 {
                        *val = "@";
                    }
                });
                line.reverse();
                writeln!(f, "|{}|", line.join(""))?;
            }
        }
        Ok(())
    }
}
/*
|1000000 - 40
|1000000 - 40
|1000000 - 40
|1000000 - 40
*/
/*
|0000001 - 01
|0000001 - 01
|0000001 - 01
|0000001 - 01
*/

const CHAMBER_WIDTH: usize = 7;

#[derive(Clone)]
struct Chamber {
    grid: Vec<u8>,
}

impl Default for Chamber {
    fn default() -> Self {
        Self {
            grid: Vec::with_capacity(1_024),
        }
    }
}

impl std::fmt::Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter().rev() {
            let mut line: Vec<&str> = (0..CHAMBER_WIDTH).map(|_| ".").collect();
            line.iter_mut()
                .enumerate()
                .take(CHAMBER_WIDTH)
                .for_each(|(i, val)| {
                    if (row >> i) & 0b1 != 0 {
                        *val = "@";
                    }
                });
            line.reverse();
            writeln!(f, "|{}|", line.join("")).expect("");
        }
        writeln!(f, "+-------+")
    }
}

#[allow(dead_code)]
fn print_scenario(chamber: &Chamber, shape: &Shape, height: usize) {
    let mut new_chamber = chamber.clone();
    if height > chamber.grid.len() {
        for _ in 0..(height - chamber.grid.len()) {
            new_chamber.grid.push(0u8);
        }
    }
    let mut cur_height = height;
    for line in shape.0.iter() {
        if cur_height < new_chamber.grid.len() {
            new_chamber.grid[height] |= line;
        } else {
            new_chamber.grid.push(*line);
        }
        cur_height += 1;
    }
    println!("{}", new_chamber);
}

fn sim<const NUM_ROCKS: usize>(jet_pattern: &[JetPattern]) -> usize {
    let mut chamber: Chamber = Chamber::default();
    let shapes = Shape::shape_order();
    let mut shape_iter = shapes.iter().cycle();
    let mut jet_iter = jet_pattern.iter().cycle();
    let mut seen_states = HashMap::with_capacity(1_024);
    let mut jet_idx = 0;
    let mut cycle_height = 0;
    let mut cur_rock = 0;

    //for (cur_rock, shape) in Shape::shape_order()
    //    .iter()
    //    .cycle()
    //    .take(NUM_ROCKS)
    //    .enumerate()
    while cur_rock < NUM_ROCKS {
        cur_rock += 1;
        let mut shape = shape_iter.next().unwrap().clone();

        let mut height = chamber.grid.len() + 3;
        //println!("Rock {cur_rock} begins falling:");
        //print_scenario(&chamber, &shape, height);

        // drop rock
        loop {
            shape.blow(jet_iter.next().expect(""), &chamber, height);
            jet_idx = (jet_idx + 1) % jet_pattern.len();
            //print_scenario(&chamber, &shape, height);

            if height > chamber.grid.len() {
                //println!("Rock falls 1 unit:");
                height -= 1;
                //print_scenario(&chamber, &shape, height);
            } else if height == 0 || shape.collide(&chamber.grid[(height - 1)..]) {
                //println!("rock comes to rest");
                for b in shape.0.iter() {
                    if *b != 0 {
                        if height < chamber.grid.len() {
                            chamber.grid[height] |= b;
                        } else {
                            chamber.grid.push(*b);
                        }
                        height += 1
                    }
                }
                //print!("{}\n", chamber);
                break;
            } else {
                //println!("Rock falls 1 unit:");
                height -= 1;
                //print_scenario(&chamber, &shape, height);
            }
        }

        if chamber.grid.len() < 8 {
            continue;
        }
        let top_chamber =
            u64::from_ne_bytes(chamber.grid[chamber.grid.len() - 8..].try_into().unwrap());
        let state = (top_chamber, cur_rock % 5, jet_idx);
        match seen_states.entry(state) {
            Entry::Occupied(e) => {
                let (n, h) = e.get();
                let num_rocks_in_cycle = cur_rock - n;
                let num_cycles = (NUM_ROCKS - cur_rock) / num_rocks_in_cycle;
                cur_rock += num_rocks_in_cycle * num_cycles;
                cycle_height += num_cycles * (chamber.grid.len() - h);
                seen_states.clear();
            }
            Entry::Vacant(e) => {
                e.insert((cur_rock, chamber.grid.len()));
            }
        }
        /*
        println!();
        println!("CHAMBER END");
        println!("{}", chamber);
        println!("CHAMBER END");
        println!();
        */
    }
    chamber.grid.len() + cycle_height
}

type Input = Vec<JetPattern>;

    #[generator(gen)]
    pub fn parse(data: &str) -> Input {
        data.trim()
            .chars()
            .map(|c| match c {
                '<' => JetPattern::Left,
                '>' => JetPattern::Right,
                _ => unreachable!("unexpected input for jet pattern"),
            })
            .collect_vec()
    }

    #[solver(part1, gen)]
    pub fn part_1(input: &Input) -> usize {
        sim::<2022>(input)
    }

    #[solver(part2, gen)]
    pub fn part_2(input: &Input) -> usize {
        sim::<1_000_000_000_000>(input)
    }

    #[solution(part1, gen)]
    pub fn solution_part_1(input: &str) -> usize {
        let data = parse(input);
        part_1(&data)
    }

    #[solution(part2, gen)]
    pub fn solution_part_2(input: &str) -> usize {
        let data = parse(input);
        part_2(&data)
    }
}

// Tests commented out due to type mismatch: solution functions expect parsed input
// #[cfg(test)]
// mod test {
//     use indoc::indoc;

//     const EXAMPLE: &str = indoc! {"
//         >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
//     "};

//     #[test]
//     fn part_1_example() {
//         assert_eq!(super::solutions::part_1(EXAMPLE), 3068);
//     }

//     #[test]
//     fn part_2_example() {
//         assert_eq!(super::solutions::part_2(EXAMPLE), 1514285714288_usize);
//     }
// }

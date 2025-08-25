use common::{solution, Answer};

use glam::IVec2;

use std::collections::HashSet;
use std::slice::Iter;

solution!("Unstable Diffusion", 23);

#[derive(Clone, Debug)]
struct Grove {
    positions: HashSet<IVec2>,
}

impl std::fmt::Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.positions.iter().map(|coord| coord.x).min().unwrap();
        let x_max = self.positions.iter().map(|coord| coord.x).max().unwrap();
        let y_min = self.positions.iter().map(|coord| coord.y).min().unwrap();
        let y_max = self.positions.iter().map(|coord| coord.y).max().unwrap();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if self.positions.contains(&IVec2::new(x, y)) {
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

fn get_surrounding(pos: IVec2) -> Vec<IVec2> {
    vec![
        pos + IVec2::new(-1, -1), pos + IVec2::new(0, -1), pos + IVec2::new(1, -1),
        pos + IVec2::new(-1,  0),                           pos + IVec2::new(1,  0),
        pos + IVec2::new(-1,  1), pos + IVec2::new(0,  1), pos + IVec2::new(1,  1),
    ]
}

fn get_north(pos: IVec2) -> Vec<IVec2> {
    vec![
        pos + IVec2::new(-1, -1), pos + IVec2::new(0, -1), pos + IVec2::new(1, -1),
    ]
}

fn get_south(pos: IVec2) -> Vec<IVec2> {
    vec![
        pos + IVec2::new(-1, 1), pos + IVec2::new(0, 1), pos + IVec2::new(1, 1),
    ]
}

fn get_west(pos: IVec2) -> Vec<IVec2> {
    vec![
        pos + IVec2::new(-1, -1), pos + IVec2::new(-1, 0), pos + IVec2::new(-1, 1),
    ]
}

fn get_east(pos: IVec2) -> Vec<IVec2> {
    vec![
        pos + IVec2::new(1, -1), pos + IVec2::new(1, 0), pos + IVec2::new(1, 1),
    ]
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
                self.positions.insert(IVec2::new(
                    proposed_move.x * 2 - elf.x,
                    proposed_move.y * 2 - elf.y,
                ));
                moved -= 2;
            }
        }

        assert_eq!(self.positions.len(), prev.len());
        moved > 0
    }
}

fn proposed(positions: &HashSet<IVec2>, elf_pos: IVec2, round: u32) -> IVec2 {
    if get_surrounding(elf_pos)
        .iter()
        .all(|coord| !positions.contains(coord))
    {
        elf_pos
    } else {
        for dir in Direction::iter().cycle().skip(round as usize % 4).take(4) {
            match dir {
                Direction::North => {
                    if get_north(elf_pos).iter().all(|coord| !positions.contains(coord)) {
                        return IVec2::new(elf_pos.x, elf_pos.y - 1);
                    }
                }
                Direction::South => {
                    if get_south(elf_pos).iter().all(|coord| !positions.contains(coord)) {
                        return IVec2::new(elf_pos.x, elf_pos.y + 1);
                    }
                }
                Direction::East => {
                    if get_east(elf_pos).iter().all(|coord| !positions.contains(coord)) {
                        return IVec2::new(elf_pos.x + 1, elf_pos.y);
                    }
                }
                Direction::West => {
                    if get_west(elf_pos).iter().all(|coord| !positions.contains(coord)) {
                        return IVec2::new(elf_pos.x - 1, elf_pos.y);
                    }
                }
            }
        }
        elf_pos
    }
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

type Input = Grove;

fn parse(data: &str) -> nom::IResult<&str, Input> {
    let elf_pos: HashSet<IVec2> = data
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, val)| *val == '#')
                .map(move |(x, _)| IVec2::new(x as i32, y as i32))
        })
        .collect();
    let grove = Grove { positions: elf_pos };
    Ok(("", grove))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, mut data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    for i in 0..10 {
        data.round(i);
    }
    let result = data.check();
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, mut data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let mut ans = 0;
    for i in 0.. {
        if !data.round(i) {
            ans = i + 1;
            break;
        }
    }
    Ok((ans as usize).into())
}

#[cfg(test)]
mod test {
    use common::load_raw;

    const EXAMPLE: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 110.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 20.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 23)?;
        assert_eq!(super::part_1(input.as_str())?, 3940.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 23)?;
        assert_eq!(super::part_2(input.as_str())?, 990.into());
        Ok(())
    }
}

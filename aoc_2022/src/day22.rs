use crate::prelude::*;
use colored::Colorize;
use framework::grid::Grid;

day!(22, parse => part1, part2::<50>);

#[derive(Debug)]
enum Instruction {
    RotateLeft,
    RotateRight,
    Move(u32),
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Instruction {
    pub fn execute(&self, cur: &Cursor, grid: &Grid<char>) -> Cursor {
        match self {
            Instruction::RotateLeft => Cursor {
                pos: cur.pos,
                direction: match cur.direction {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                },
                warp: cur.warp,
            },
            Instruction::RotateRight => Cursor {
                pos: cur.pos,
                direction: match cur.direction {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                },
                warp: cur.warp,
            },
            Instruction::Move(num) => {
                let mut new_cur = *cur;
                for _ in 0..*num {
                    new_cur = (cur.warp)(new_cur, grid);
                }
                new_cur
            }
        }
    }
}

struct Input {
    instructions: Vec<Instruction>,
    grid: Grid<char>,
}

fn parse(input: &str) -> ParseResult<Input> {
    let (grid, path) = input.split_once("\n\n").unwrap();

    let width = grid
        .lines()
        .fold(0, |size, line| line.bytes().len().max(size));
    let vec = grid
        .lines()
        .flat_map(|line| {
            let mut v = line.bytes().map(char::from).collect_vec();
            v.resize(width, ' ');
            v
        })
        .collect_vec();

    let mut instructions = vec![];
    let mut chars = path.trim().chars().peekable();
    while let Some(c) = chars.next() {
        let instr = match c {
            'L' => Instruction::RotateLeft,
            'R' => Instruction::RotateRight,
            c => {
                let mut s = c.to_string();
                while let Some(ch) = chars.peek() {
                    if !ch.is_ascii_digit() {
                        break;
                    };
                    s.push(chars.next().unwrap());
                }
                Instruction::Move(s.parse().unwrap())
            }
        };
        instructions.push(instr);
    }

    let input: Input = Input {
        instructions,
        grid: Grid { vec, width },
    };
    Ok(input)
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Direction {
    pub fn delta(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

pub fn move_cursor(cur: Cursor, grid: &Grid<char>) -> Cursor {
    let mut new_cur = cur;
    let (dx, dy) = cur.direction.delta();
    new_cur.pos.x = (new_cur.pos.x as i64 + dx) as usize;
    new_cur.pos.y = (new_cur.pos.y as i64 + dy) as usize;

    match grid.get(new_cur.pos).unwrap_or(&' ') {
        '#' => new_cur = cur,

        '.' => (),
        ' ' => {
            match cur.direction {
                Direction::Right => {
                    new_cur.pos.x = grid
                        .points()
                        .iter()
                        .filter(|coord| coord.y == new_cur.pos.y)
                        .filter(|coord| coord.x != cur.pos.x)
                        .filter(|coord| grid.get(**coord).unwrap() != &' ')
                        .map(|coord| coord.x)
                        .min()
                        .unwrap();
                }
                Direction::Left => {
                    new_cur.pos.x = grid
                        .points()
                        .iter()
                        .filter(|coord| coord.y == new_cur.pos.y)
                        .filter(|coord| coord.x != cur.pos.x)
                        .filter(|coord| grid.get(**coord).unwrap() != &' ')
                        .map(|coord| coord.x)
                        .max()
                        .unwrap();
                }
                Direction::Up => {
                    new_cur.pos.y = grid
                        .points()
                        .iter()
                        .filter(|coord| coord.x == new_cur.pos.x)
                        .filter(|coord| coord.y != cur.pos.y)
                        .filter(|coord| grid.get(**coord).unwrap() != &' ')
                        .map(|coord| coord.y)
                        .max()
                        .unwrap();
                }
                Direction::Down => {
                    new_cur.pos.y = grid
                        .points()
                        .iter()
                        .filter(|coord| coord.x == new_cur.pos.x)
                        .filter(|coord| coord.y != cur.pos.y)
                        .filter(|coord| grid.get(**coord).unwrap() != &' ')
                        .map(|coord| coord.y)
                        .min()
                        .unwrap();
                }
            }
            if grid.get(new_cur.pos).unwrap() == &'#' {
                new_cur = cur;
            }
        }
        _ => unreachable!(),
    }
    new_cur
}

pub fn move_cursor_cube<const FACE_SIZE: usize>(cur: Cursor, grid: &Grid<char>) -> Cursor {
    let mut new_cur = cur;
    let (dx, dy) = cur.direction.delta();
    new_cur.pos.x = (new_cur.pos.x as i64 + dx) as usize;
    new_cur.pos.y = (new_cur.pos.y as i64 + dy) as usize;

    match grid.get(new_cur.pos).unwrap_or(&' ') {
        '#' => new_cur = cur,

        '.' => (),
        ' ' => {
            // This is specific to my input, which looks like this:
            //  12
            //  3
            // 45
            // 6
            let current_face = match (cur.pos.x / FACE_SIZE, cur.pos.y / FACE_SIZE) {
                (1, 0) => 1,
                (2, 0) => 2,
                (1, 1) => 3,
                (0, 2) => 4,
                (1, 2) => 5,
                (0, 3) => 6,
                _ => panic!("Unexpected position: {:?}", cur.pos),
            };
            match current_face {
                1 => match cur.direction {
                    Direction::Left => {
                        new_cur.direction = Direction::Right;
                        new_cur.pos.y = FACE_SIZE * 3 - cur.pos.y - 1;
                        new_cur.pos.x = 0;
                    }
                    Direction::Up => {
                        new_cur.direction = Direction::Right;
                        new_cur.pos.y = cur.pos.x + FACE_SIZE * 2;
                        new_cur.pos.x = 0;
                    }
                    _ => unreachable!(),
                },
                2 => match cur.direction {
                    Direction::Right => {
                        new_cur.direction = Direction::Left;
                        new_cur.pos.y = FACE_SIZE * 3 - cur.pos.y - 1;
                        new_cur.pos.x = cur.pos.x - FACE_SIZE;
                    }
                    Direction::Down => {
                        new_cur.direction = Direction::Left;
                        new_cur.pos.y = cur.pos.x - FACE_SIZE;
                        new_cur.pos.x = FACE_SIZE * 2 - 1;
                    }
                    Direction::Up => {
                        new_cur.direction = Direction::Up;
                        new_cur.pos.y = FACE_SIZE * 4 - 1;
                        new_cur.pos.x = cur.pos.x - FACE_SIZE * 2;
                    }
                    _ => unreachable!(),
                },
                3 => match cur.direction {
                    Direction::Right => {
                        new_cur.direction = Direction::Up;
                        new_cur.pos.y = FACE_SIZE - 1;
                        new_cur.pos.x = cur.pos.y + FACE_SIZE;
                    }
                    Direction::Left => {
                        new_cur.direction = Direction::Down;
                        new_cur.pos.y = FACE_SIZE * 2;
                        new_cur.pos.x = cur.pos.y - FACE_SIZE;
                    }
                    _ => unreachable!(),
                },
                4 => match cur.direction {
                    Direction::Left => {
                        new_cur.direction = Direction::Right;
                        new_cur.pos.y = 3 * FACE_SIZE - cur.pos.y - 1;
                        new_cur.pos.x = FACE_SIZE;
                    }
                    Direction::Up => {
                        new_cur.direction = Direction::Right;
                        new_cur.pos.y = cur.pos.x + FACE_SIZE;
                        new_cur.pos.x = FACE_SIZE;
                    }
                    _ => unreachable!(),
                },
                5 => match cur.direction {
                    Direction::Right => {
                        new_cur.direction = Direction::Left;
                        new_cur.pos.y = 3 * FACE_SIZE - cur.pos.y - 1;
                        new_cur.pos.x = FACE_SIZE * 3 - 1;
                    }
                    Direction::Down => {
                        new_cur.direction = Direction::Left;
                        new_cur.pos.y = cur.pos.x + FACE_SIZE * 2;
                        new_cur.pos.x = FACE_SIZE - 1;
                    }
                    _ => unreachable!(),
                },
                6 => match cur.direction {
                    Direction::Right => {
                        new_cur.direction = Direction::Up;
                        new_cur.pos.y = FACE_SIZE * 3 - 1;
                        new_cur.pos.x = cur.pos.y - 2 * FACE_SIZE;
                    }
                    Direction::Down => {
                        new_cur.direction = Direction::Down;
                        new_cur.pos.y = 0;
                        new_cur.pos.x = cur.pos.x + FACE_SIZE * 2;
                    }
                    Direction::Left => {
                        new_cur.direction = Direction::Down;
                        new_cur.pos.y = 0;
                        new_cur.pos.x = cur.pos.y - FACE_SIZE * 2;
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            if grid.get(new_cur.pos).unwrap() == &'#' {
                new_cur = cur;
            }
        }
        _ => unreachable!(),
    }
    new_cur
}

#[derive(Clone, Copy)]
pub struct Cursor {
    pos: Coord2d<usize>,
    direction: Direction,
    warp: fn(Cursor, &Grid<char>) -> Cursor,
}
impl std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) - {}", self.pos, self.direction)
    }
}
impl Cursor {
    pub fn password(&self) -> u64 {
        (self.pos.y as u64 + 1) * 1_000
            + (self.pos.x as u64 + 1) * 4
            + match self.direction {
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Up => 3,
            }
    }
}

#[allow(dead_code)]
fn print_map(grid: &Grid<char>, cur: &Cursor) {
    let cur_idx = cur.pos.y as usize * grid.width + cur.pos.x as usize;
    for (i, c) in grid.vec.iter().enumerate() {
        let mut s = c.to_string().white();
        if i == cur_idx {
            s = match cur.direction {
                Direction::Up => "^".bold().bright_yellow(),
                Direction::Down => "v".bold().bright_yellow(),
                Direction::Right => ">".bold().bright_yellow(),
                Direction::Left => "<".bold().bright_yellow(),
            }
        }
        if (i + 1) % grid.width == 0 {
            println!("{}", s);
        } else {
            print!("{}", s);
        }
    }
    println!();
    println!();
}

fn part1(input: &Input) -> u64 {
    let mut cur = Cursor {
        pos: Coord2d::from((0_usize, 0_usize)),
        direction: Direction::Right,
        warp: move_cursor,
    };
    cur.pos.x = input
        .grid
        .row(0)
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == '.')
        .map(|(i, _)| i)
        .min()
        .unwrap();
    println!("starting cursor: {}", cur);
    //print_map(&input.grid, &cur);

    for instr in &input.instructions {
        cur = instr.execute(&cur, &input.grid);
        println!("{}", instr);
        println!("cursor: {}", cur);
        //print_map(&input.grid, &cur);
    }

    let password = cur.password();
    println!("answer: {}", password);
    password
}

fn part2<const FACE_SIZE: usize>(input: &Input) -> u64 {
    let mut cur = Cursor {
        pos: Coord2d::from((0_usize, 0_usize)),
        direction: Direction::Right,
        warp: move_cursor_cube::<FACE_SIZE>,
    };
    cur.pos.x = input
        .grid
        .row(0)
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == '.')
        .map(|(i, _)| i)
        .min()
        .unwrap();
    println!("starting cursor: {}", cur);
    //print_map(&input.grid, &cur);

    for instr in &input.instructions {
        cur = instr.execute(&cur, &input.grid);
        println!("{}", instr);
        println!("cursor: {}", cur);
        //print_map(&input.grid, &cur);
    }

    let password = cur.password();
    println!("answer: {}", password);
    // highter than 108_159
    password
}

tests! {
    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";
    const INPUT: &str = include_str!("data/22.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 6032);
    simple_tests!(parse, part1, part1_input_test, INPUT => 36518);
    //HARD CODED FOR MY INPUT - simple_tests!(parse, part2::<4>, part2_example_test, EXAMPLE => 0);
    simple_tests!(parse, part2::<50>, part2_input_test, INPUT => 143208);
}

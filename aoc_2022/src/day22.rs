use crate::prelude::*;
use colored::Colorize;
use framework::grid::Grid;

day!(22, parse => part1, part2);

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
            },
            Instruction::RotateRight => Cursor {
                pos: cur.pos,
                direction: match cur.direction {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                },
            },
            Instruction::Move(num) => {
                let mut new_cur = *cur;
                for _ in 0..*num {
                    new_cur = Self::move_cursor(new_cur, grid);
                }
                new_cur
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
                        println!("wrap down {}", new_cur.pos);
                        new_cur.pos.y = grid
                            .points()
                            .iter()
                            .filter(|coord| coord.x == new_cur.pos.x)
                            .filter(|coord| coord.y != cur.pos.y)
                            .filter(|coord| grid.get(**coord).unwrap() != &' ')
                            .map(|coord| coord.y)
                            .min()
                            .unwrap();
                        println!("wrap down {}", new_cur.pos);
                    }
                }
                println!("wrapping new coord {}", new_cur);
                if grid.get(new_cur.pos).unwrap() == &'#' {
                    new_cur = cur;
                }
            }
            _ => unreachable!(),
        }
        new_cur
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

#[derive(Clone, Copy)]
struct Cursor {
    pos: Coord2d<usize>,
    direction: Direction,
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

fn part2(_input: &Input) -> u32 {
    0
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
    //simple_tests!(parse, part2, part2_example_test, EXAMPLE => 0);
    //simple_tests!(parse, part2, part2_input_test, INPUT => 0);
}

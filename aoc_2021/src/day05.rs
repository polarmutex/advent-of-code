use common::{solution, Answer};
use std::ops::{AddAssign, Sub};

solution!("Hydrothermal Venture", 5);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Coord2d {
    x: i32,
    y: i32,
}

impl Coord2d {
    fn from_coords(x: i32, y: i32) -> Self {
        Coord2d { x, y }
    }
}

impl AddAssign for Coord2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coord2d {
    type Output = Coord2d;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    from: Coord2d,
    to: Coord2d,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }
}

#[derive(Debug, Clone)]
struct LineIter {
    current: Coord2d,
    offset: Coord2d,
    remaining_points: u32,
}

impl IntoIterator for &Line {
    type Item = Coord2d;
    type IntoIter = LineIter;

    fn into_iter(self) -> Self::IntoIter {
        let delta = self.to - self.from;
        let remainder = delta.x.abs().max(delta.y.abs());
        LineIter {
            current: self.from,
            offset: Coord2d::from_coords(delta.x.signum(), delta.y.signum()),
            remaining_points: remainder as u32 + 1,
        }
    }
}

impl Iterator for LineIter {
    type Item = Coord2d;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_points > 0 {
            self.remaining_points -= 1;
            let value = self.current;
            self.current += self.offset;
            Some(value)
        } else {
            None
        }
    }
}

fn count_overlap_pts<'i, I: Iterator<Item = &'i Line> + Clone + 'i>(lines: I) -> u32 {
    let mut board_size: Coord2d = lines.clone().flat_map(|line| [line.from, line.to]).fold(
        Coord2d::from_coords(0, 0),
        |mut size, coord| {
            size.x = size.x.max(coord.x);
            size.y = size.x.max(coord.y);
            size
        },
    );
    // to account for zero based indexing
    board_size.x += 1;
    board_size.y += 1;
    // Initialize 2d vector
    let mut board = vec![vec![0; board_size.y as usize]; board_size.x as usize];
    for line in lines {
        for point in line {
            let x = point.x as usize;
            let y = point.y as usize;
            board[x][y] += 1;
        }
    }
    let mut count = 0;
    for x in 0..board_size.x {
        for y in 0..board_size.y {
            if board[x as usize][y as usize] > 1 {
                count += 1;
            }
        }
    }
    count
}
type Input = Vec<Line>;

fn parse(input: &str) -> nom::IResult<&str, Input> {
    let lines: Result<Vec<Line>, miette::Error> = input
        .lines()
        .map(|line| {
            let (from_str, to_str) = line.split_once(" -> ")
                .ok_or_else(|| miette::miette!("Invalid line format: {}", line))?;
            
            let (x_str, y_str) = from_str.split_once(',')
                .ok_or_else(|| miette::miette!("Invalid from coordinate: {}", from_str))?;
            let from = Coord2d::from_coords(
                x_str.parse::<i32>().map_err(|e| miette::miette!("Invalid x coordinate: {}", e))?,
                y_str.parse::<i32>().map_err(|e| miette::miette!("Invalid y coordinate: {}", e))?,
            );
            
            let (x_str, y_str) = to_str.split_once(',')
                .ok_or_else(|| miette::miette!("Invalid to coordinate: {}", to_str))?;
            let to = Coord2d::from_coords(
                x_str.parse::<i32>().map_err(|e| miette::miette!("Invalid x coordinate: {}", e))?,
                y_str.parse::<i32>().map_err(|e| miette::miette!("Invalid y coordinate: {}", e))?,
            );
            
            Ok(Line { from, to })
        })
        .collect();
    
    match lines {
        Ok(lines) => Ok(("", lines)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::MapRes))),
    }
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, lines) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = count_overlap_pts(
        lines
            .iter()
            .filter(|line| line.is_horizontal() || line.is_vertical()),
    );
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, lines) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = count_overlap_pts(lines.iter());
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 5.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 12.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2021, 5)?;
        assert_eq!(super::part_1(input.as_str())?, 7269.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2021, 5)?;
        assert_eq!(super::part_2(input.as_str())?, 21140.into());
        Ok(())
    }
}

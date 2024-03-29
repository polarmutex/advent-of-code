use itertools::Itertools;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Coord2d {
    pub x: i32,
    pub y: i32,
}

impl Coord2d {
    pub fn from_coords(x: i32, y: i32) -> Self {
        Coord2d { x, y }
    }

    pub fn manhattan_distance(&self, other: &Coord2d) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn surrounding(&self) -> impl Iterator<Item = Coord2d> + '_ {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ]
        .into_iter()
        .map(move |(dx, dy)| Coord2d {
            x: self.x + dx,
            y: self.y + dy,
        })
    }

    pub fn north(&self) -> impl Iterator<Item = Coord2d> + '_ {
        [(-1, -1), (0, -1), (1, -1)]
            .into_iter()
            .map(move |(dx, dy)| Coord2d {
                x: self.x + dx,
                y: self.y + dy,
            })
    }

    pub fn south(&self) -> impl Iterator<Item = Coord2d> + '_ {
        [(-1, 1), (0, 1), (1, 1)]
            .into_iter()
            .map(move |(dx, dy)| Coord2d {
                x: self.x + dx,
                y: self.y + dy,
            })
    }

    pub fn west(&self) -> impl Iterator<Item = Coord2d> + '_ {
        [(-1, -1), (-1, 0), (-1, 1)]
            .into_iter()
            .map(move |(dx, dy)| Coord2d {
                x: self.x + dx,
                y: self.y + dy,
            })
    }

    pub fn east(&self) -> impl Iterator<Item = Coord2d> + '_ {
        [(1, -1), (1, 0), (1, 1)]
            .into_iter()
            .map(move |(dx, dy)| Coord2d {
                x: self.x + dx,
                y: self.y + dy,
            })
    }
}

impl Display for Coord2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl AddAssign for Coord2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl Sub for Coord2d {
    type Output = Coord2d;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord2d {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord3d {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Coord3d {
    pub fn from_coords((x, y, z): (isize, isize, isize)) -> Self {
        Coord3d { x, y, z }
    }

    pub fn plus_adjacent(&self) -> impl Iterator<Item = Coord3d> + '_ {
        [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .into_iter()
        .map(move |(dx, dy, dz)| Coord3d {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        })
    }
}

impl FromStr for Coord3d {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Coord3d, Self::Err> {
        let point_vec = input.splitn(3, ',').collect_vec();

        let point = Coord3d {
            x: point_vec[0].parse().ok().unwrap(),
            y: point_vec[1].parse().ok().unwrap(),
            z: point_vec[2].parse().ok().unwrap(),
        };
        Ok(point)
    }
}

impl Display for Coord3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

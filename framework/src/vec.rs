use std::ops::AddAssign;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Coord2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coord2d<T> {
    pub fn from_coords((x, y): (T, T)) -> Self {
        Coord2d::<T> { x, y }
    }
}

impl<T: AddAssign> AddAssign for Coord2d<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl<F, I: From<F>> From<(F, F)> for Coord2d<I> {
    fn from((x, y): (F, F)) -> Self {
        Coord2d {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coord2d<T> {
    type Output = Coord2d<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord2d {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

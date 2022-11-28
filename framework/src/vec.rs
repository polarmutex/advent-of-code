use std::ops::AddAssign;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn from_coords((x, y): (T, T)) -> Self {
        Vec2::<T> { x, y }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl<F, I: From<F>> From<(F, F)> for Vec2<I> {
    fn from((x, y): (F, F)) -> Self {
        Vec2 {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

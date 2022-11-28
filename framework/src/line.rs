use crate::vec::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub from: Vec2<i32>,
    pub to: Vec2<i32>,
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    pub fn between_points((p1, p2): (Vec2<i32>, Vec2<i32>)) -> Self {
        Line { from: p1, to: p2 }
    }
}

#[derive(Debug, Clone)]
pub struct LineIter {
    current: Vec2<i32>,
    offset: Vec2<i32>,
    remaining_points: u32,
}

impl IntoIterator for &'_ Line {
    type Item = Vec2<i32>;
    type IntoIter = LineIter;

    fn into_iter(self) -> Self::IntoIter {
        let delta = self.to - self.from;
        let remainder = delta.x.abs().max(delta.y.abs());
        LineIter {
            current: self.from,
            offset: (delta.x.signum(), delta.y.signum()).into(),
            remaining_points: remainder as u32 + 1,
        }
    }
}

impl Iterator for LineIter {
    type Item = Vec2<i32>;

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

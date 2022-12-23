use crate::vec::Coord2d;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub from: Coord2d,
    pub to: Coord2d,
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    pub fn between_points((p1, p2): (Coord2d, Coord2d)) -> Self {
        Line { from: p1, to: p2 }
    }
}

#[derive(Debug, Clone)]
pub struct LineIter {
    current: Coord2d,
    offset: Coord2d,
    remaining_points: u32,
}

impl IntoIterator for &'_ Line {
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

use crate::vec::Coord2d;

#[derive(Clone)]
pub struct Grid<T> {
    pub vec: Vec<T>,
    pub width: u32,
}

impl<T: Clone> Grid<T> {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.vec.len() as u32 / self.width()
    }

    pub fn get(&self, pos: Coord2d) -> Option<&T> {
        if pos.x as u32 >= self.width() || pos.y as u32 >= self.height() {
            return None;
        }
        self.vec
            .get((pos.y as u32 * self.width() + pos.x as u32) as usize)
    }

    pub fn get_mut(&mut self, pos: Coord2d) -> Option<&mut T> {
        if pos.x as u32 >= self.width() || pos.y as u32 >= self.height() {
            return None;
        }
        self.vec
            .get_mut((pos.y as u32 * self.width + pos.x as u32) as usize)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Coord2d, &T)> {
        self.vec
            .chunks_exact(self.width() as usize)
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, val)| (Coord2d::from_coords(x as i32, y as i32), val))
            })
    }

    pub fn points(&self) -> Vec<Coord2d> {
        self.vec
            .iter()
            .enumerate()
            .map(move |(i, _)| {
                let pos = (i % self.width as usize, i / self.width as usize);
                Coord2d::from_coords(pos.0 as i32, pos.1 as i32)
            })
            .collect()
    }

    pub fn find(&self, mut f: impl FnMut(&T) -> bool) -> Vec<Coord2d> {
        self.vec
            .iter()
            .enumerate()
            .filter(move |(_, val)| f(val))
            .map(move |(i, _)| {
                let pos = (i % self.width as usize, i / self.width as usize);
                Coord2d::from_coords(pos.0 as i32, pos.1 as i32)
            })
            .collect()
    }

    pub fn neighbors_plus(&self, node: Coord2d) -> impl Iterator<Item = Coord2d> {
        let neighbor_index = [(0, -1), (1, 0), (-1, 0), (0, 1)]
            .into_iter()
            .map(move |(dx, dy)| Coord2d::from_coords(node.x + dx, node.y + dy));
        self.filter_in_bounds(neighbor_index)
    }

    pub fn filter_in_bounds(
        &self,
        iter: impl Iterator<Item = Coord2d>,
    ) -> impl Iterator<Item = Coord2d> {
        let (width, height) = (self.width() as i32, self.height() as i32);
        iter.filter(move |&pos| 0 <= pos.x && pos.x < width && 0 <= pos.y && pos.y < height)
            .map(|pos| Coord2d::from_coords(pos.x, pos.y))
    }

    pub fn row(&self, num: usize) -> Vec<T> {
        let start = num * self.width as usize;
        let end = (num + 1) * self.width as usize;
        // TODO
        self.vec[start..end].to_vec()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.vec.iter().enumerate() {
            if (i + 1) % self.width as usize == 0 {
                writeln!(f, "{}", c).unwrap();
            } else {
                write!(f, "{}", c).unwrap();
            }
        }
        writeln!(f)
    }
}

impl<T: Clone> std::ops::Index<Coord2d> for Grid<T> {
    type Output = T;
    fn index(&self, pos: Coord2d) -> &Self::Output {
        self.get(pos).expect("Index out of bounds")
    }
}

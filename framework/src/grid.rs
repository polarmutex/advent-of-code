use crate::vec::Coord2d;

#[derive(Clone)]
pub struct Grid<T> {
    pub vec: Vec<T>,
    pub width: usize,
}

impl<T: Clone> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.vec.len() / self.width()
    }

    pub fn get(&self, pos: Coord2d<usize>) -> Option<&T> {
        if pos.x >= self.width() || pos.y >= self.height() {
            return None;
        }
        self.vec.get(pos.y * self.width() + pos.x)
    }

    pub fn get_mut(&mut self, pos: Coord2d<usize>) -> Option<&mut T> {
        if pos.x >= self.width() || pos.y >= self.height() {
            return None;
        }
        self.vec.get_mut(pos.y * self.width + pos.x)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Coord2d<usize>, &T)> {
        self.vec
            .chunks_exact(self.width())
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, val)| (Coord2d::from((x, y)), val))
            })
    }

    pub fn points(&self) -> Vec<Coord2d<usize>> {
        self.vec
            .iter()
            .enumerate()
            .map(move |(i, _)| {
                let pos = (i % self.width, i / self.width);
                Coord2d::from(pos)
            })
            .collect()
    }

    pub fn find(&self, mut f: impl FnMut(&T) -> bool) -> Vec<Coord2d<usize>> {
        self.vec
            .iter()
            .enumerate()
            .filter(move |(_, val)| f(val))
            .map(move |(i, _)| {
                let pos = (i % self.width, i / self.width);
                Coord2d::from(pos)
            })
            .collect()
    }

    pub fn neighbors_plus(&self, node: Coord2d<usize>) -> impl Iterator<Item = Coord2d<usize>> {
        let neighbor_index = [(0, -1), (1, 0), (-1, 0), (0, 1)]
            .into_iter()
            .map(move |(dx, dy)| {
                Coord2d::<isize>::from((node.x as isize + dx, node.y as isize + dy))
            });
        self.filter_in_bounds(neighbor_index)
    }

    pub fn filter_in_bounds(
        &self,
        iter: impl Iterator<Item = Coord2d<isize>>,
    ) -> impl Iterator<Item = Coord2d<usize>> {
        let (width, height) = (self.width() as isize, self.height() as isize);
        iter.filter(move |&pos| 0 <= pos.x && pos.x < width && 0 <= pos.y && pos.y < height)
            .map(|pos| Coord2d::from((pos.x as usize, pos.y as usize)))
    }

    pub fn row(&self, num: usize) -> Vec<T> {
        let start = num * self.width;
        let end = (num + 1) * self.width;
        // TODO
        self.vec[start..end].to_vec()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.vec.iter().enumerate() {
            if (i + 1) % self.width == 0 {
                writeln!(f, "{}", c).unwrap();
            } else {
                write!(f, "{}", c).unwrap();
            }
        }
        writeln!(f)
    }
}

impl<T: Clone> std::ops::Index<Coord2d<usize>> for Grid<T> {
    type Output = T;
    fn index(&self, pos: Coord2d<usize>) -> &Self::Output {
        self.get(pos).expect("Index out of bounds")
    }
}

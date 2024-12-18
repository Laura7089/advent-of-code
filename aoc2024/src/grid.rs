#[derive(Debug, Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    pub elems: Vec<Vec<T>>,
}

pub type Point = (usize, usize);
pub type Vector = (isize, isize);

#[inline]
#[allow(clippy::cast_possible_wrap)]
pub fn get_vector(first: Point, second: Point) -> Vector {
    (
        second.0 as isize - first.0 as isize,
        second.1 as isize - first.1 as isize,
    )
}

pub const MANHATTAN_OFFSETS: [Vector; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
pub const ALL_OFFSETS: [Vector; 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

impl<T> std::ops::Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): Point) -> &Self::Output {
        &self.elems[y][x]
    }
}

impl<T> std::ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, (x, y): Point) -> &mut Self::Output {
        &mut self.elems[y][x]
    }
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, elems: Vec<Vec<T>>) -> Self {
        Self {
            width,
            height,
            elems,
        }
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            elems: Vec::new(),
        }
    }

    #[inline]
    pub fn dims(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn offset_point(&self, (x, y): Point, (dx, dy): Vector) -> Option<Point> {
        let xmod = match x.checked_add_signed(dx) {
            Some(x) if x < self.width => x,
            _ => return None,
        };
        let ymod = match y.checked_add_signed(dy) {
            Some(y) if y < self.height => y,
            _ => return None,
        };
        Some((xmod, ymod))
    }

    pub fn adj_coords_manhattan<'a>(&'a self, point: Point) -> impl Iterator<Item = Point> + 'a {
        MANHATTAN_OFFSETS
            .into_iter()
            .flat_map(move |offset| self.offset_point(point, offset))
    }

    pub fn adj_coords<'a>(&'a self, point: Point) -> impl Iterator<Item = Point> + 'a {
        ALL_OFFSETS
            .into_iter()
            .flat_map(move |offset| self.offset_point(point, offset))
    }

    pub fn neighbours_manhattan(&self, point: Point) -> impl Iterator<Item = &T> {
        self.adj_coords_manhattan(point).map(|p| &self[p])
    }

    pub fn neighbours(&self, point: Point) -> impl Iterator<Item = &T> {
        self.adj_coords(point).map(|p| &self[p])
    }

    pub fn raycast(&self, start: Point, offset: Vector) -> SteppedRaycast<T> {
        SteppedRaycast {
            grid: self,
            cursor: Some(start),
            offset,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SteppedRaycast<'a, T> {
    grid: &'a Grid<T>,
    cursor: Option<Point>,
    offset: Vector,
}

impl<T> Iterator for SteppedRaycast<'_, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor = self.grid.offset_point(self.cursor?, self.offset);
        self.cursor
    }
}

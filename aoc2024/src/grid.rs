#![warn(missing_docs)]
//! Helper types and functions for 2D gridlike problems.

/// Two-dimensional row-major ordered grid of `T`.
///
/// # Note
/// It is valid for `elems` to be **smaller** than the dimensions of the grid (see [`Self::new_unaligned`]).
/// However, if this is case, it is the responsibility of the caller to ensure that indexing operations are valid.
#[derive(Debug, Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    /// Elements stored by `self`.
    pub elems: Vec<Vec<T>>,
}

/// Two-dimensional coordinate pair.
pub type Point = (usize, usize);
/// Two-dimensional directional pair.
pub type Vector = (isize, isize);

/// Calculate the [`Vector`] offset between two [`Point`]s.
#[inline]
#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub fn get_vector(first: Point, second: Point) -> Vector {
    (
        second.0 as isize - first.0 as isize,
        second.1 as isize - first.1 as isize,
    )
}

/// Orthogonal direction offsets.
///
/// Runs clockwise from "directly up".
pub const ORTH_OFFSETS: [Vector; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
/// Orthogonal and diagonal direction offsets.
///
/// Runs clockwise from "directly up".
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

impl Grid<()> {
    /// Create a `Grid` with no elements.
    ///
    /// Useful for index and point calculations without needing to allocate memory.
    #[must_use]
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            elems: Vec::new(),
        }
    }
}

impl<T> Grid<T> {
    /// Create a new `Grid`.
    ///
    /// Assumes that the dimensions of `elems` are final and infers `Grid` dimensions from that.
    /// If `elems` is [sparsely represented](https://en.wikipedia.org/wiki/Sparse_matrix#Storage), use [`Grid::new_unaligned`] intead.
    #[must_use]
    pub fn new(elems: Vec<Vec<T>>) -> Self {
        Self {
            width: elems[0].len(),
            height: elems.len(),
            elems,
        }
    }

    /// Create a new `Grid` with explicit dimensions.
    #[must_use]
    pub fn new_unaligned(width: usize, height: usize, elems: Vec<Vec<T>>) -> Self {
        Self {
            width,
            height,
            elems,
        }
    }

    /// Get the dimensions of `self`.
    #[inline]
    #[must_use]
    pub fn dims(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Offset a point in `self` by a vector.
    ///
    /// Returns `None` if the resulting [`Point`] would be out of bounds.
    #[must_use]
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

    /// Iterate over orthogonally-adjacent coordinates to `point` in `self`.
    pub fn adj_coords_orth(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        ORTH_OFFSETS
            .into_iter()
            .filter_map(move |offset| self.offset_point(point, offset))
    }

    /// Iterate over adjacent coordinates to `point` in `self`, including diagonals.
    pub fn adj_coords(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        ALL_OFFSETS
            .into_iter()
            .filter_map(move |offset| self.offset_point(point, offset))
    }

    /// Iterate over orthogonally-adjacent elements to that at `point` in `self`.
    ///
    /// Returns an iterator over coordinate-element tuples.
    pub fn neighbours_orth(&self, point: Point) -> impl Iterator<Item = (Point, &T)> {
        self.adj_coords_orth(point).map(|p| (p, &self[p]))
    }

    /// Iterate over adjacent elements to that at `point` in `self`, including diagonals.
    ///
    /// Returns an iterator over coordinate-element tuples.
    pub fn neighbours(&self, point: Point) -> impl Iterator<Item = (Point, &T)> {
        self.adj_coords(point).map(|p| (p, &self[p]))
    }

    /// Iterate over a raycast in a particular [`Vector`].
    #[must_use]
    pub fn raycast(&self, start: Point, offset: Vector) -> SteppedRaycast<T> {
        SteppedRaycast {
            grid: self,
            cursor: Some(start),
            offset,
        }
    }

    /// Iterate over all points in the grid (and their coordinates).
    pub fn iter_all(&self) -> impl Iterator<Item = (Point, &T)> {
        self.elems
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, sq)| ((x, y), sq)))
    }
}

/// Iterator over coordinates in a particular raycast direction.
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
impl<T> std::iter::FusedIterator for SteppedRaycast<'_, T> {}

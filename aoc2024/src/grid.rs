#![warn(missing_docs)]
//! Helper types and functions for 2D gridlike problems.

use std::marker::PhantomData;

/// Adjacency method for a 2D grid.
pub trait Adjacency {
    /// Relative offsets of adjacents points to a given location in the grid.
    const OFFSETS: &[Vector];
}

/// Orthgonal adjacency system marker.
pub struct Orthogonal;
impl Adjacency for Orthogonal {
    /// Orthogonal direction offsets.
    ///
    /// Runs clockwise from "directly up".
    const OFFSETS: &[Vector] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
}

/// Diagonal **and orthogonal** adjacency marker.
pub struct Diagonal;
impl Adjacency for Diagonal {
    /// Orthogonal and diagonal direction offsets.
    ///
    /// Runs clockwise from "directly up".
    const OFFSETS: &[Vector] = &[
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
}

/// Two-dimensional row-major ordered grid of `T`.
///
/// # Note
/// It is valid for `elems` to be **smaller** than the dimensions of the grid (see [`Self::new_unaligned`]).
/// However, if this is case, it is the responsibility of the caller to ensure that indexing operations are valid.
#[derive(Debug, Clone)]
pub struct Grid<T, A = Diagonal> {
    width: usize,
    height: usize,
    /// Elements stored by `self`.
    pub elems: Vec<Vec<T>>,
    _adj: PhantomData<A>,
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

impl<T, A> std::ops::Index<Point> for Grid<T, A> {
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

impl<A> Grid<(), A> {
    /// Create a `Grid` with no elements.
    ///
    /// Useful for index and point calculations without needing to allocate memory.
    #[must_use]
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            elems: Vec::new(),
            _adj: PhantomData,
        }
    }
}

impl<T, A> Grid<T, A> {
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
            _adj: PhantomData,
        }
    }

    /// Create a new `Grid` with explicit dimensions.
    #[must_use]
    pub fn new_unaligned(width: usize, height: usize, elems: Vec<Vec<T>>) -> Self {
        Self {
            width,
            height,
            elems,
            _adj: PhantomData,
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

    /// Convert this grid to use a different adjacency system.
    #[must_use]
    pub fn into_adjacency<AN>(self) -> Grid<T, AN> {
        Grid {
            width: self.width,
            height: self.height,
            elems: self.elems,
            _adj: PhantomData,
        }
    }
}

impl<T, A: Adjacency> Grid<T, A> {
    /// Iterate over adjacent coordinates to `point` in `self`.
    pub fn adj_coords(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        A::OFFSETS
            .iter()
            .filter_map(move |&offset| self.offset_point(point, offset))
    }

    /// Iterate over orthogonally-adjacent elements to that at `point` in `self`.
    ///
    /// Returns an iterator over coordinate-element tuples.
    pub fn neighbours(&self, point: Point) -> impl Iterator<Item = (Point, &T)> {
        self.adj_coords(point).map(|p| (p, &self[p]))
    }

    /// Iterate over a raycast in a particular [`Vector`].
    #[must_use]
    pub fn raycast(&self, start: Point, offset: Vector) -> SteppedRaycast<T, A> {
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
pub struct SteppedRaycast<'a, T, A> {
    grid: &'a Grid<T, A>,
    cursor: Option<Point>,
    offset: Vector,
}

impl<T, A> Iterator for SteppedRaycast<'_, T, A> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor = self.grid.offset_point(self.cursor?, self.offset);
        self.cursor
    }
}
impl<T, A> std::iter::FusedIterator for SteppedRaycast<'_, T, A> {}

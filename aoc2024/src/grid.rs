#![warn(missing_docs)]
//! Helper types and functions for 2D gridlike problems.

use std::marker::PhantomData;

/// Two-dimensional directional pair.
pub type Vector = (isize, isize);

/// Two-dimensional coordinate pair.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Option<Self>;

    fn add(self, rhs: Vector) -> Self::Output {
        Some(Self {
            x: self.x.checked_add_signed(rhs.0)?,
            y: self.y.checked_add_signed(rhs.1)?,
        })
    }
}

impl Point {
    /// Calculate the [`Vector`] offset between two [`Point`]s.
    #[inline]
    #[allow(clippy::cast_possible_wrap)]
    #[must_use]
    pub fn vector(self, right: Self) -> Vector {
        (
            right.x as isize - self.x as isize,
            right.y as isize - self.y as isize,
        )
    }

    /// Calculate the absolute distance between two [`Point`]s.
    pub fn dist(self, Point { x: rx, y: ry }: Point) -> usize {
        self.x.abs_diff(rx) + self.y.abs_diff(ry)
    }
}

/// Adjacency method for a 2D grid.
pub trait Adjacency {
    /// Relative offsets of adjacents points to a given location in the grid.
    const OFFSETS: &[Vector];

    /// Check if two [`Point`]s are adjacent in this system.
    fn adjacent(left: Point, right: Point) -> bool;

    /// Get all adjacent coordinates to `start`.
    fn adjacent_coords(start: Point) -> impl Iterator<Item = Point> {
        Self::OFFSETS.iter().filter_map(move |&off| start + off)
    }
}

/// Orthgonal adjacency system marker.
pub struct Orthogonal;
impl Adjacency for Orthogonal {
    /// Orthogonal direction offsets.
    ///
    /// Runs clockwise from "directly up".
    const OFFSETS: &[Vector] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn adjacent(Point { x: lx, y: ly }: Point, Point { x: rx, y: ry }: Point) -> bool {
        lx.abs_diff(rx) + ly.abs_diff(ry) == 1
    }
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

    fn adjacent(Point { x: lx, y: ly }: Point, Point { x: rx, y: ry }: Point) -> bool {
        let xdiff = lx.abs_diff(rx);
        let ydiff = ly.abs_diff(ry);
        xdiff + ydiff == 1 || (xdiff == 1 && ydiff == 1)
    }
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

impl<T, A> std::ops::Index<Point> for Grid<T, A> {
    type Output = T;

    fn index(&self, Point { x, y }: Point) -> &Self::Output {
        &self.elems[y][x]
    }
}

impl<T> std::ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, Point { x, y }: Point) -> &mut Self::Output {
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
    pub fn offset_point(&self, point: Point, offset: Vector) -> Option<Point> {
        let offed = (point + offset)?;

        if offed.x >= self.width {
            None
        } else if offed.y >= self.height {
            None
        } else {
            Some(offed)
        }
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

    /// Iterate over all points in the grid (and their coordinates).
    pub fn iter_all(&self) -> impl Iterator<Item = (Point, &T)> {
        self.elems.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, sq)| ((x, y).into(), sq))
        })
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

    /// Check if two points are adjacent in the grid.
    ///
    /// Forwards to [`Adjacency::adjacent`] on `A`.
    #[must_use]
    pub fn adjacent(left: Point, right: Point) -> bool {
        A::adjacent(left, right)
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

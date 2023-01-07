//! Grids, with custom indexing logic

use super::{IPoint, UPoint};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use delegate::delegate;
use ndarray::{s, Array2, Axis, Ix1};

/// A 2-d `ndarray` which has inbuilt indexing logic to work with non-0-based indexing
///
/// The first (x) axis is indexed from the "top".
#[derive(Clone, Debug)]
pub struct Offset<E> {
    /// The underlying grid
    pub grid: Array2<E>,
    /// (Top-left, bottom-right), both inclusive
    pub limits: (UPoint, UPoint),
}

impl<E> Offset<E>
where
    E: Copy,
{
    pub fn new(min @ (x0, y0): UPoint, max @ (x1, y1): UPoint, elem: E) -> Self {
        Self {
            grid: Array2::from_elem((x1 - x0 + 1, y1 - y0 + 1), elem),
            limits: (min, max),
        }
    }

    pub fn expand(&mut self, top: usize, bottom: usize, left: usize, right: usize, elem: E) {
        let (ox, oy) = self.grid.dim();
        let (otl, obr) = self.limits;

        let nx = ox + left + right;
        let ny = oy + top + bottom;

        let mut new_arr = Array2::from_elem((nx, ny), elem);
        new_arr
            .slice_mut(s![left..(left + ox), top..(top + oy)])
            .assign(&self.grid);

        self.grid = new_arr;
        self.limits.0 = (
            otl.0
                .checked_sub(left)
                .expect("Cannot expand this far left"),
            otl.1
                .checked_sub(top)
                .expect("Cannot expand this far upwards"),
        );
        self.limits.1 = (obr.0 + right, obr.1 + bottom);
    }
}

#[allow(clippy::inline_always)]
impl<E> Offset<E> {
    delegate! {
        to self.grid {
            #[call(dim)]
            pub fn true_dim(&self) -> UPoint;
            pub fn axis_iter(&self, axis: Axis) -> ndarray::iter::AxisIter<E, Ix1>;
        }
    }

    #[must_use]
    pub fn contains(&self, (x, y): UPoint) -> bool {
        let ((x0, y0), (x1, y1)) = self.limits;
        (x0..=x1).contains(&x) && (y0..=y1).contains(&y)
    }

    #[must_use]
    fn convert_index(&self, (x, y): UPoint) -> UPoint {
        let ((left, top), _) = self.limits;
        assert!(x >= left, "x={x} is less than the left limit ({left})");
        assert!(y >= top, "y={y} is less than the top limit ({top})");
        (x - left, y - top)
    }
}

impl<E> Index<UPoint> for Offset<E> {
    type Output = E;
    fn index(&self, index: UPoint) -> &Self::Output {
        &self.grid[self.convert_index(index)]
    }
}

impl<E> IndexMut<UPoint> for Offset<E> {
    fn index_mut(&mut self, index: UPoint) -> &mut Self::Output {
        let index = self.convert_index(index);
        &mut self.grid[index]
    }
}

impl<E: Display> Display for Offset<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for column in self.grid.columns() {
            for elem in column {
                write!(f, "{elem}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Toroidal<T>(pub Array2<T>);

#[must_use]
pub fn toroidal_index_single(mut index: isize, len: usize) -> usize {
    if index < 0 {
        let mult = (-index) as usize / len;
        index += ((mult + 1) * len) as isize;
    }
    (index % len as isize) as usize
}

impl<T> Toroidal<T> {
    delegate! {
        to self.0 {
            pub fn dim(&self) -> (usize, usize);
        }
    }

    #[must_use]
    pub fn convert_index((x, y): IPoint, (xlim, ylim): UPoint) -> UPoint {
        (
            toroidal_index_single(x, xlim),
            toroidal_index_single(y, ylim),
        )
    }
}

impl<T> Index<IPoint> for Toroidal<T> {
    type Output = T;

    fn index(&self, index: IPoint) -> &Self::Output {
        &self.0[Self::convert_index(index, self.dim())]
    }
}

impl<T> IndexMut<IPoint> for Toroidal<T> {
    fn index_mut(&mut self, index: IPoint) -> &mut Self::Output {
        let dim = self.dim();
        &mut self.0[Self::convert_index(index, dim)]
    }
}

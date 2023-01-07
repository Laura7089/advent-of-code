#![warn(clippy::pedantic)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::similar_names)]

#[macro_use]
extern crate aoc_runner_derive;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day20;
mod day25;

aoc_lib! { year = 2022 }

#[cfg(test)]
fn get_input(day: u32) -> String {
    std::fs::read_to_string(format!("./input/2022/day{day}.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

use delegate::delegate;
use ndarray::prelude::*;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

type Pair<T> = (T, T);

type UPoint = Pair<usize>;
type IPoint = Pair<isize>;

/// Manhattan distances
mod manhattan {
    use num_traits::Signed;
    use std::ops::{Add, Sub};

    use super::{Pair, UPoint};

    /// Manhattan distance between two (signed) points
    pub fn dists<T>((lx, ly): Pair<T>, (rx, ry): Pair<T>) -> T
    where
        T: Add<T, Output = T> + Sub<T, Output = T> + Ord + Signed,
    {
        let x = if lx > rx { lx - rx } else { rx - lx };
        let y = if ly > ry { ly - ry } else { ry - ly };
        x.abs() + y.abs()
    }

    /// Manhattan distance between two (unsigned) points
    pub fn distu<T>((lx, ly): Pair<T>, (rx, ry): Pair<T>) -> T
    where
        T: Add<T, Output = T> + Sub<T, Output = T> + Ord,
    {
        let x = if lx > rx { lx - rx } else { rx - lx };
        let y = if ly > ry { ly - ry } else { ry - ly };
        x + y
    }

    /// Generate a "ring" shape of points a fixed distance around a point
    #[derive(Debug, Clone)]
    pub enum Ring {
        NonZero {
            centre: UPoint,
            dist: usize,
            quad_dist: usize,
            quadrant: usize,
        },
        Zero {
            centre: UPoint,
            done: bool,
        },
    }

    impl Ring {
        pub fn new(centre: UPoint, dist: usize) -> Self {
            if dist == 0 {
                Self::Zero {
                    done: false,
                    centre,
                }
            } else {
                Self::NonZero {
                    centre,
                    dist,
                    quad_dist: 0,
                    quadrant: 1,
                }
            }
        }
    }

    impl Iterator for Ring {
        type Item = UPoint;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                // End of iteration, ie:
                // - Zero distance and we're done
                // - We're on the "fifth" quadrant
                Self::Zero { done: true, .. } | Self::NonZero { quadrant: 5, .. } => None,
                Self::Zero { done, centre } => {
                    *done = true;
                    Some(*centre)
                }

                // If we're at the end of the quadrant, move to the next
                Self::NonZero {
                    quad_dist,
                    dist,
                    quadrant,
                    ..
                } if quad_dist == dist => {
                    *quad_dist = 0;
                    *quadrant += 1;
                    self.next()
                }
                // Base case, actually return :)
                Self::NonZero {
                    centre: (sx, sy),
                    dist,
                    quad_dist,
                    quadrant,
                } => {
                    // Pull current values out
                    let inc = *quad_dist;
                    let dec = *dist - *quad_dist;

                    // Increment for next loop
                    *quad_dist += 1;

                    Some(match quadrant {
                        1 => (sx.saturating_add(inc), sy.saturating_add(dec)),
                        2 => (sx.saturating_add(dec), sy.saturating_sub(inc)),
                        3 => (sx.saturating_sub(inc), sy.saturating_sub(dec)),
                        4 => (sx.saturating_sub(dec), sy.saturating_add(inc)),
                        _ => unreachable!(),
                    })
                }
            }
        }
    }
}

/// Parsing helpers
mod parse {
    use nom::{
        character::complete::{i64, u64},
        combinator::map,
    };

    pub type IResult<'a, T> = nom::IResult<&'a str, T>;

    pub fn usize(input: &str) -> IResult<usize> {
        map(u64, |x| x.try_into().unwrap())(input)
    }

    pub fn isize(input: &str) -> IResult<isize> {
        map(i64, |x| x.try_into().unwrap())(input)
    }
}

/// A 2-d `ndarray` which has inbuilt indexing logic to work with non-0-based indexing
///
/// The first (x) axis is indexed from the "top".
#[derive(Clone, Debug)]
struct OffsetGrid<E> {
    /// The underlying grid
    pub grid: Array2<E>,
    /// (Top-left, bottom-right), both inclusive
    pub limits: (UPoint, UPoint),
}

impl<E> OffsetGrid<E>
where
    E: Copy,
{
    fn new(min @ (x0, y0): UPoint, max @ (x1, y1): UPoint, elem: E) -> Self {
        Self {
            grid: Array2::from_elem((x1 - x0 + 1, y1 - y0 + 1), elem),
            limits: (min, max),
        }
    }

    fn expand(&mut self, top: usize, bottom: usize, left: usize, right: usize, elem: E) {
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
impl<E> OffsetGrid<E> {
    delegate! {
        to self.grid {
            #[call(dim)]
            fn true_dim(&self) -> UPoint;
            fn axis_iter(&self, axis: Axis) -> ndarray::iter::AxisIter<E, Ix1>;
        }
    }

    #[must_use]
    fn contains(&self, (x, y): UPoint) -> bool {
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

impl<E> Index<UPoint> for OffsetGrid<E> {
    type Output = E;
    fn index(&self, index: UPoint) -> &Self::Output {
        &self.grid[self.convert_index(index)]
    }
}

impl<E> IndexMut<UPoint> for OffsetGrid<E> {
    fn index_mut(&mut self, index: UPoint) -> &mut Self::Output {
        let index = self.convert_index(index);
        &mut self.grid[index]
    }
}

impl<E: Display> Display for OffsetGrid<E> {
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

// TODO: toroidal grid struct?
fn wrapping_index<T>(slice: &[T], orig: usize, modifier: isize) -> &T {
    wrapping_index_len(slice, orig, modifier, slice.len())
}

#[must_use]
fn wrapping_index_len<T>(
    collection: &(impl IndexMut<usize, Output = T> + ?Sized),
    orig: usize,
    modifier: isize,
    len: usize,
) -> &T {
    &collection[index_mod(orig, modifier, len)]
}

#[must_use]
fn index_mod(orig: usize, modifier: isize, len: usize) -> usize {
    let mut index = modifier.saturating_add_unsigned(orig);
    if index < 0 {
        let mult = (-index) as usize / len;
        index += ((mult + 1) * len) as isize;
    }
    (index % len as isize) as usize
}

/// Iterate over the points adjacent to another
///
/// The first four given are the directly adjacent ones, the next four are adjacent diagonally.
#[derive(Clone, Debug)]
struct Adjacents<const N: usize> {
    base: UPoint,
    i: usize,
}

impl<const N: usize> Adjacents<N> {
    // TODO: generate this somehow?
    const MODIFIERS: [IPoint; 8] = [
        (1, 0),
        (0, -1),
        (-1, 0),
        (0, 1),
        (1, -1),
        (-1, -1),
        (-1, 1),
        (1, 1),
    ];

    #[must_use]
    const fn new(base: UPoint) -> Self {
        Self { base, i: 0 }
    }
}

impl<const N: usize> Iterator for Adjacents<N> {
    type Item = IPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == N {
            None
        } else {
            let (dx, dy) = Self::MODIFIERS[self.i];
            self.i += 1;
            let x = dx.saturating_add_unsigned(self.base.0);
            let y = dy.saturating_add_unsigned(self.base.1);
            Some((x, y))
        }
    }
}

impl<const N: usize> Adjacents<N> {
    fn constrain(self, (mx, my): UPoint) -> impl Iterator<Item = UPoint> {
        let (mx, my) = (mx as isize, my as isize);
        self.filter_map(move |(x, y)| {
            if (0..mx).contains(&x) && (0..my).contains(&y) {
                Some((x as usize, y as usize))
            } else {
                None
            }
        })
    }
}

mod ranges {
    /// An inclusive, contiguous range of `usize`
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Range {
        pub start: usize,
        pub end: usize,
    }

    /// Relationship between one range and another
    ///
    /// Value | Meaning
    /// ---|---
    /// `NoIntersect` | the ranges do not overlap at all
    /// `IntersectBeginning` | the other range overlaps this one at the beginning
    /// `IntersectEnd` | the other range overlaps this one at the end
    /// `Contains` | this range fully contains the other
    /// `ContainedBy` | the other range fully contains this one
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum RangeRel {
        NoIntersect,
        IntersectBeginning,
        IntersectEnd,
        Contains,
        ContainedBy,
    }

    pub enum DiffResult {
        /// rhs completely contains lhs thus the result is the empty set
        Empty,
        /// lhs does not change (no overlap)
        NoChange(Range),
        /// successful diff
        Success(Range),
        /// successful diff, lhs is bisected
        SuccessBisect(Range, Range),
    }

    impl Range {
        #[must_use]
        #[allow(clippy::len_without_is_empty)]
        pub fn len(self) -> usize {
            self.end - self.start + 1
        }

        #[must_use]
        pub fn to_single(self) -> Option<usize> {
            if self.len() == 1 {
                Some(self.start)
            } else {
                None
            }
        }

        #[must_use]
        pub fn relationship(self, other: Range) -> RangeRel {
            if self.is_superset(other) {
                RangeRel::Contains
            } else if other.is_superset(self) {
                RangeRel::ContainedBy
            } else if self.contains(other.start) {
                RangeRel::IntersectEnd
            } else if other.contains(self.start) {
                RangeRel::IntersectBeginning
            } else {
                RangeRel::NoIntersect
            }
        }

        #[must_use]
        pub fn is_superset(self, rhs: Range) -> bool {
            self.start <= rhs.start && self.end >= rhs.end
        }

        #[must_use]
        pub fn contains(self, x: usize) -> bool {
            (self.start..=self.end).contains(&x)
        }

        #[must_use]
        pub fn union(self, rhs: Range) -> Option<Self> {
            match self.relationship(rhs) {
                RangeRel::Contains => Some(self),
                RangeRel::ContainedBy => Some(rhs),
                RangeRel::IntersectEnd => Some(Self {
                    start: self.start,
                    end: rhs.end,
                }),
                RangeRel::IntersectBeginning => Some(Self {
                    start: rhs.start,
                    end: self.end,
                }),
                RangeRel::NoIntersect => None,
            }
        }

        /// Tries to "subtract" the right range pair from the left
        ///
        /// That is, it finds the result of `lhs / rhs` in set logic.
        #[must_use]
        pub fn diff(self, rhs @ Range { start: rs, end: re }: Range) -> DiffResult {
            use DiffResult::*;

            let Range { start: ls, end: le } = self;
            if self == rhs {
                return Empty;
            }
            match self.relationship(rhs) {
                RangeRel::ContainedBy => Empty,
                RangeRel::NoIntersect => NoChange(self),
                RangeRel::IntersectBeginning => Success(Self {
                    start: re + 1,
                    end: le,
                }),
                RangeRel::IntersectEnd => Success(Self {
                    start: ls,
                    end: rs - 1,
                }),
                RangeRel::Contains if ls == rs => Success(Self {
                    start: re + 1,
                    end: le,
                }),
                RangeRel::Contains if le == re => Success(Self {
                    start: ls,
                    end: rs - 1,
                }),
                RangeRel::Contains => SuccessBisect(
                    Self {
                        start: ls,
                        end: rs - 1,
                    },
                    Self {
                        start: re + 1,
                        end: le,
                    },
                ),
            }
        }

        /// Repeatedly [`Range::diff`] the elements of `diffs` from `self`
        ///
        /// # Note
        ///
        /// Assumes that only one range will remain.
        #[must_use]
        pub fn demolish(self, mut diffs: impl Iterator<Item = Self> + Clone) -> Option<Self> {
            let mut current = self;
            while let Some(rhs) = diffs.next() {
                use DiffResult::*;
                match current.diff(rhs) {
                    Empty => return None,
                    NoChange(_) => {}
                    Success(new) => current = new,
                    SuccessBisect(l, r) => {
                        let l = l.demolish(diffs.clone());
                        let r = r.demolish(diffs.clone());
                        return l.or(r);
                    }
                }
            }

            Some(current)
        }
    }

    impl IntoIterator for Range {
        type Item = usize;
        type IntoIter = std::ops::RangeInclusive<usize>;

        fn into_iter(self) -> Self::IntoIter {
            self.start..=self.end
        }
    }
}
pub use ranges::Range;

#[cfg(test)]
mod tests {
    use super::*;

    mod adjacents {
        use super::Adjacents;

        #[test]
        fn sanity() {
            assert_eq!(
                Adjacents::<4>::new((10, 10)).collect::<Vec<_>>(),
                vec![(11, 10), (10, 9), (9, 10), (10, 11)]
            );
        }

        #[test]
        fn filtered() {
            assert_eq!(
                Adjacents::<4>::new((10, 10))
                    .constrain((20, 20))
                    .collect::<Vec<_>>(),
                vec![(11, 10), (10, 9), (9, 10), (10, 11)]
            );

            assert_eq!(
                Adjacents::<4>::new((0, 0))
                    .constrain((2, 2))
                    .collect::<Vec<_>>(),
                vec![(1, 0), (0, 1)]
            );
        }
    }

    #[test]
    fn manhattan_diamond() {
        use super::manhattan::Ring;
        assert_eq!(
            Ring::new((5, 5), 2).collect::<Vec<_>>(),
            vec![
                (5, 7),
                (6, 6),
                (7, 5),
                (6, 4),
                (5, 3),
                (4, 4),
                (3, 5),
                (4, 6),
            ]
        );

        assert_eq!(Ring::new((0, 0), 0).collect::<Vec<_>>(), vec![(0, 0)]);
    }
}

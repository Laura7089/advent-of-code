#![warn(clippy::pedantic)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::clone_on_copy)]

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

use ndarray::prelude::*;
use std::ops::{Index, IndexMut};

type IResult<'a, T> = nom::IResult<&'a str, T>;

type UPoint = (usize, usize);
type IPoint = (isize, isize);

fn make_usize(input: &str) -> IResult<usize> {
    use nom::character::complete::u32;
    nom::combinator::map(u32, |x| x as usize)(input)
}

fn array2_inner<'a, A>(array: &'a Array2<A>) -> ArrayView2<'a, A> {
    array.slice(s![1..-1, 1..-1])
}

/// A 2-d `ndarray` which has inbuilt indexing logic to work with non-0-based indexing
#[derive(Clone, Debug)]
struct OffsetGrid<E> {
    /// The underlying grid
    pub grid: Array2<E>,
    /// Top-left, bottom-right
    pub limits: (UPoint, UPoint),
}

impl<E> OffsetGrid<E>
where
    E: Copy,
{
    fn new(min @ (x0, y0): UPoint, max @ (x1, y1): UPoint, elem: E) -> Self {
        Self {
            grid: Array2::from_elem((x1 - x0, y1 - y0), elem),
            limits: (min, max),
        }
    }
}

impl<E> OffsetGrid<E> {
    #[must_use]
    fn contains_vert(&self, y: usize) -> bool {
        ((self.limits.0 .1)..(self.limits.1 .1)).contains(&y)
    }
}

impl<E> Index<UPoint> for OffsetGrid<E> {
    type Output = E;
    fn index(&self, mut index: UPoint) -> &Self::Output {
        index.0 -= self.limits.0 .0;
        index.1 -= self.limits.0 .1;
        &self.grid[index]
    }
}

impl<E> IndexMut<UPoint> for OffsetGrid<E> {
    fn index_mut(&mut self, mut index: UPoint) -> &mut Self::Output {
        index.0 -= self.limits.0 .0;
        index.1 -= self.limits.0 .1;
        &mut self.grid[index]
    }
}

// Former display implementation, could rewrite to fit
// impl Display for Cave {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let full: String = self
//             .grid
//             .columns()
//             .into_iter()
//             .map(|column| {
//                 column
//                     .iter()
//                     .copied()
//                     .map(Into::into)
//                     .chain(['\n'].into_iter())
//                     .collect::<Vec<_>>()
//             })
//             .flatten()
//             .collect();
//         write!(f, "{full}")
//     }
// }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacents_sanity() {
        assert_eq!(
            Adjacents::<4>::new((10, 10)).collect::<Vec<_>>(),
            vec![(11, 10), (10, 9), (9, 10), (10, 11)]
        );
    }

    #[test]
    fn adjacents_filtered_sanity() {
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

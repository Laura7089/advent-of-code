//! Manhattan distances

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhattan_diamond() {
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

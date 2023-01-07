//! Simple arithmetic operations and reduction

use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Rem, Sub};

/// A simple arithmetic operation
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    /// Applies the operation on `left` with `right`
    pub fn apply<T, R>(self, left: T, right: T) -> R
    where
        T: Add<T, Output = R> + Sub<T, Output = R> + Mul<T, Output = R> + Div<T, Output = R>,
    {
        match self {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
        }
    }

    /// Applies the "opposite" of the operation with `right` on `left`
    pub fn apply_rev<T, R>(self, left: T, right: T) -> R
    where
        T: Add<T, Output = R> + Sub<T, Output = R> + Mul<T, Output = R> + Div<T, Output = R>,
    {
        match self {
            Op::Add => left - right,
            Op::Sub => left + right,
            Op::Mul => left / right,
            Op::Div => left * right,
        }
    }
}

/// A simple arithmetic operation loaded with an operand
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FullOp<T: Copy + Clone + PartialEq + Eq>(pub Op, pub T);

impl<T> FullOp<T>
where
    T: Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Rem<T, Output = T>
        + std::cmp::Ord
        + num_traits::Zero
        + num_traits::One
        + Copy
        + Clone
        + PartialEq
        + Eq,
{
    pub fn is_noop(self) -> bool {
        match self.0 {
            Op::Add | Op::Sub => self.1 == T::zero(),
            Op::Mul | Op::Div => self.1 == T::one(),
        }
    }

    pub fn apply(self, input: T) -> T {
        self.0.apply(input, self.1)
    }

    pub fn apply_rev(self, input: T) -> T {
        self.0.apply_rev(input, self.1)
    }

    /// Try to combine a single pair of operations
    ///
    /// If `self` and `other` reduce, returns `Some(op)` where `op` is the result of the reduction.
    pub fn try_combine(self, other: Self) -> Option<Self> {
        use Op::*;

        match (self, other) {
            // Two Adds or two Subs combine
            (Self(l @ (Add | Sub), _), Self(r, _)) if l == r => Some(Self(l, self.1 + other.1)),
            (Self(Add, a), Self(Sub, s)) | (Self(Sub, s), Self(Add, a)) => Some(match a.cmp(&s) {
                Ordering::Greater => Self(Add, a - s),
                Ordering::Less => Self(Sub, s - a),
                Ordering::Equal => Self(Add, T::zero()),
            }),
            // Two Muls or two Divs combine
            (Self(lo @ (Mul | Div), l), Self(ro, r)) if lo == ro => Some(Self(lo, l * r)),
            (Self(Mul, m), Self(Div, d)) | (Self(Div, d), Self(Mul, m)) => {
                if m == d {
                    Some(Self(Mul, T::one()))
                } else if m % d == T::zero() {
                    Some(Self(Mul, m / d))
                } else if d % m == T::zero() {
                    Some(Self(Div, d / m))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

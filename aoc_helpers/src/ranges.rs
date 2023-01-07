//! Utilities for dealing with ranges and their combinations

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

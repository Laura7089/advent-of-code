//! Utilities for dealing with ranges and their combinations

/// An inclusive, contiguous range of `usize`
// TODO: make generic over the element type
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Range {
    /// Start of the range, inclusive
    pub start: usize,
    /// End of the range, inclusive
    pub end: usize,
}

/// Relationship between one range and another
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RangeRel {
    /// ranges do not overlap at all
    NoIntersect,
    /// other range overlaps this one's beginning
    IntersectBeginning,
    /// other range overlaps this one's end
    IntersectEnd,
    /// this range fully contains the other
    Contains,
    /// other range fully contains this one
    ContainedBy,
}

/// Enumeration of possible `diff` interactions
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
    /// The number of elements in the range
    ///
    /// Guaranteed to be always at least `1`.
    #[must_use]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(self) -> usize {
        self.end - self.start + 1
    }

    /// Tries to extract one number from `self`
    ///
    /// If `self` covers more than one number, returns `None`.
    #[must_use]
    pub fn to_single(self) -> Option<usize> {
        if self.len() == 1 {
            Some(self.start)
        } else {
            None
        }
    }

    /// Find the [`RangeRel`] of `self` to `other`
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

    /// Check is `self` is a superset of `rhs`
    #[must_use]
    pub fn is_superset(self, rhs: Range) -> bool {
        self.start <= rhs.start && self.end >= rhs.end
    }

    /// Check if `self` contains a particular value
    #[must_use]
    pub fn contains(self, x: usize) -> bool {
        (self.start..=self.end).contains(&x)
    }

    /// Try to find the union of `self` and `rhs`
    ///
    /// Returns `None` if `self` and `rhs` are completely disjoint.
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

#![warn(missing_docs)]
//! Convenience extensions for iterators.

/// Utility trait that extends [`Iterator`].
pub trait IterExt: Iterator
where
    Self: Sized,
{
    /// Iterate over the cartesian product of `self` and `other`.
    fn cart_prod<'a, T, I>(
        self,
        other: I,
    ) -> impl Iterator<Item = (<Self as Iterator>::Item, T)> + 'a
    where
        I: Iterator<Item = T> + Clone + 'a,
        Self: 'a,
        <Self as Iterator>::Item: Copy,
    {
        self.flat_map(move |elem| std::iter::repeat(elem).zip(other.clone()))
    }

    /// Apply `callback` to each element of `self` and iterate over tuples of the element and result.
    fn cart_prod_with<'a, T, I, F>(
        self,
        mut callback: F,
    ) -> impl Iterator<Item = (<Self as Iterator>::Item, T)> + 'a
    where
        I: Iterator<Item = T> + 'a,
        F: FnMut(<Self as Iterator>::Item) -> I + 'a,
        Self: 'a,
        <Self as Iterator>::Item: Copy,
    {
        self.flat_map(move |elem| std::iter::repeat(elem).zip(callback(elem)))
    }
}

impl<I: Iterator> IterExt for I {}

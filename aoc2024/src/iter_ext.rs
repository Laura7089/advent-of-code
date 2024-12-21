pub trait IterExt: Iterator
where
    Self: Sized,
{
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
        self.flat_map(move |elem| {
            let other = callback(elem);
            std::iter::repeat(elem).zip(other)
        })
    }
}

impl<I: Iterator> IterExt for I {}

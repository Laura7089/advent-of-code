pub trait IterExt: Iterator
where
    <Self as Iterator>::Item: Copy,
    Self: Sized,
{
    fn cart_prod_with<'a, T, I, F>(
        self,
        mut callback: F,
    ) -> impl Iterator<Item = (<Self as Iterator>::Item, T)> + 'a
    where
        I: Iterator<Item = T> + 'a,
        F: FnMut(<Self as Iterator>::Item) -> I + 'a,
        Self: 'a,
    {
        self.flat_map(move |elem| {
            let other = callback(elem);
            std::iter::repeat(elem).zip(other)
        })
    }
}

impl<E: Copy, T: Iterator<Item = E>> IterExt for T {}

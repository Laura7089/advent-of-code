pub type Idx = (usize, usize);

pub trait Field2D {
    fn height(&self) -> usize;
    fn width(&self) -> usize;

    fn adjacents(&self, (x, y): Idx) -> [Option<Idx>; 8] {
        // All adjacent squares, clockwise from top
        let mut adjacents = [None; 8];

        let (x_big, x_small) = (x > 0, x < self.width() - 1);
        let (y_big, y_small) = (y > 0, y < self.height() - 1);

        if y_small {adjacents[0] = Some((x, y + 1))}
        if x_small {
            if y_small {adjacents[1] = Some((x + 1, y + 1))}
            adjacents[2] = Some((x + 1, y));
            if y_big {adjacents[3] = Some((x + 1, y - 1))}
        }
        if y_big {adjacents[4] = Some((x, y - 1))}
        if x_big {
            if y_big {adjacents[5] = Some((x - 1, y - 1))};
            adjacents[6] = Some((x - 1, y));
            if y_small {adjacents[7] = Some((x - 1, y + 1))}
        }

        adjacents
    }
}

pub mod compressed_field {
    use std::ops::{Index, IndexMut};
    use super::*;

    #[derive(Debug, Clone)]
    pub struct CompressedField<T> {
        pub field: Vec<T>,
        row_len: usize,
    }

    impl<T> CompressedField<T> {
        pub fn new(field: Vec<T>, row_len: usize) -> Self {
            Self { field, row_len }
        }
    }

    impl<T> Field2D for CompressedField<T> {
        #[inline(always)]
        fn height(&self) -> usize {
            self.field.len() / self.row_len
        }

        #[inline(always)]
        fn width(&self) -> usize {
            self.row_len
        }
    }

    impl<T> Index<Idx> for CompressedField<T> {
        type Output = T;

        fn index(&self, (x, y): Idx) -> &Self::Output {
            assert!(x < self.width());
            assert!(y < self.height());
            &self.field[(y * self.row_len) + x]
        }
    }

    impl<T> IndexMut<Idx> for CompressedField<T> {
        fn index_mut(&mut self, (x, y): Idx) -> &mut Self::Output {
            assert!(x < self.width());
            assert!(y < self.height());
            &mut self.field[(y * self.row_len) + x]
        }
    }
}

pub mod array_field {
    use std::ops::{Index, IndexMut};
    use super::*;

    #[derive(Debug, Clone)]
    pub struct ArrayField<T, const HEIGHT: usize, const WIDTH: usize> {
        pub field: [[T; HEIGHT]; WIDTH]
    }

    impl<T, const HEIGHT: usize, const WIDTH: usize> Field2D for ArrayField<T, HEIGHT, WIDTH> {
        #[inline(always)]
        fn height(&self) -> usize {
            HEIGHT
        }

        #[inline(always)]
        fn width(&self) -> usize {
            WIDTH
        }
    }

    impl<T, const HEIGHT: usize, const WIDTH: usize> Index<Idx> for ArrayField<T, HEIGHT, WIDTH> {
        type Output = T;

        #[inline(always)]
        fn index(&self, (x, y): Idx) -> &Self::Output {
            &self.field[x][y]
        }
    }

    impl<T, const HEIGHT: usize, const WIDTH: usize> IndexMut<Idx> for ArrayField<T, HEIGHT, WIDTH> {
        #[inline(always)]
        fn index_mut(&mut self, (x, y): Idx) -> &mut Self::Output {
            &mut self.field[x][y]
        }
    }
}

use std::cmp::Ordering::*;

pub type Idx = (usize, usize);

pub trait Field2D {
    fn height(&self) -> usize;
    fn width(&self) -> usize;

    fn adjacents(&self, (x, y): Idx) -> [Option<Idx>; 4] {
        // Members of this are adjacent squares, clockwise from top
        let mut adjacents = [None; 4];
        assert!(x < self.width() && y < self.height());

        match (x.cmp(&0), x.cmp(&(self.width() - 1))) {
            (Equal, _) => adjacents[1] = Some((x + 1, y)),
            (_, Equal) => adjacents[3] = Some((x - 1, y)),
            _ => {
                adjacents[1] = Some((x + 1, y));
                adjacents[3] = Some((x - 1, y))
            }
        }

        match (y.cmp(&0), y.cmp(&(self.height() - 1))) {
            (Equal, _) => adjacents[0] = Some((x, y + 1)),
            (_, Equal) => adjacents[2] = Some((x, y - 1)),
            _ => {
                adjacents[0] = Some((x, y + 1));
                adjacents[2] = Some((x, y - 1))
            }
        }

        adjacents
    }

    fn adjacents_diag(&self, (x, y): Idx) -> [Option<Idx>; 8] {
        // First four are the non-diagonal adjacents
        // Second four are the diagonals, clockwise from top-right
        let mut adjacents = [None; 8];
        adjacents[..4].clone_from_slice(&self.adjacents((x, y)));

        assert!(x < self.width());
        assert!(y < self.height());

        // TODO: generate an array of bools and iterate the points over it to compress this horror
        match (
            x.cmp(&0),
            x.cmp(&(self.width() - 1)),
            y.cmp(&0),
            y.cmp(&(self.height() - 1)),
        ) {
            // Point is in the bottom-left
            (Equal, _, Equal, _) => adjacents[4] = Some((x + 1, y + 1)),
            // Point is in the top-left
            (Equal, _, _, Equal) => adjacents[5] = Some((x + 1, y - 1)),
            // Point is in the top-right
            (_, Equal, _, Equal) => adjacents[6] = Some((x - 1, y - 1)),
            // Point is in the bottom-right
            (_, Equal, Equal, _) => adjacents[7] = Some((x - 1, y + 1)),

            // Point is on the bottom edge
            (Greater, Less, Equal, _) => {
                adjacents[7] = Some((x - 1, y + 1));
                adjacents[4] = Some((x + 1, y + 1));
            }

            // Point is on the left edge
            (Equal, _, Greater, Less) => {
                adjacents[5] = Some((x + 1, y - 1));
                adjacents[4] = Some((x + 1, y + 1));
            }

            // Point is on the top edge
            (Greater, Less, _, Equal) => {
                adjacents[6] = Some((x - 1, y - 1));
                adjacents[5] = Some((x + 1, y - 1));
            }

            // Point is on the right edge
            (_, Equal, Greater, Less) => {
                adjacents[7] = Some((x - 1, y + 1));
                adjacents[6] = Some((x - 1, y - 1));
            }

            // Point is floating
            _ => {
                adjacents[4] = Some((x + 1, y + 1));
                adjacents[5] = Some((x - 1, y - 1));
                adjacents[6] = Some((x + 1, y - 1));
                adjacents[7] = Some((x - 1, y + 1));
            }
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

        fn index(&self, (x, y): Idx) -> &Self::Output {
            &self.field[x][y]
        }
    }

    impl<T, const HEIGHT: usize, const WIDTH: usize> IndexMut<Idx> for ArrayField<T, HEIGHT, WIDTH> {
        fn index_mut(&mut self, (x, y): Idx) -> &mut Self::Output {
            &mut self.field[x][y]
        }
    }
}

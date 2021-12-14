use std::cmp::Ordering::*;
use std::ops::{Index, IndexMut};

pub type Idx = (usize, usize);

#[derive(Debug, Clone)]
pub struct CompressedField<T> {
    pub map: Vec<T>,
    pub row_len: usize,
}

impl<T> CompressedField<T> {
    pub fn height(&self) -> usize {
        self.map.len() / self.row_len
    }

    pub fn adjacents(&self, (x, y): Idx) -> [Option<Idx>; 4] {
        // Members of this are adjacent squares, clockwise from top
        let mut adjacents = [None; 4];
        assert!(x < self.row_len && y < self.height());

        match (x.cmp(&0), x.cmp(&(self.row_len - 1))) {
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

    pub fn adjacents_diag(&self, (x, y): Idx) -> [Option<Idx>; 8] {
        // First four are the non-diagonal adjacents
        // Second four are the diagonals, clockwise from top-right
        let mut adjacents = [None; 8];
        self.adjacents((x, y))
            .into_iter()
            .enumerate()
            .for_each(|(i, n)| adjacents[i] = n);

        // TODO: generate an array of bools and iterate the points over it to compress code
        match (
            x.cmp(&0),
            x.cmp(&(self.row_len - 1)),
            y.cmp(&0),
            y.cmp(&(self.row_len - 1)),
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

impl<T> Index<Idx> for CompressedField<T> {
    type Output = T;

    fn index(&self, (x, y): Idx) -> &Self::Output {
        assert!(x < self.row_len);
        assert!(y < self.height());
        &self.map[(y * self.row_len) + x]
    }
}

impl<T> IndexMut<Idx> for CompressedField<T> {
    fn index_mut(&mut self, (x, y): Idx) -> &mut Self::Output {
        assert!(x < self.row_len);
        assert!(y < self.height());
        &mut self.map[(y * self.row_len) + x]
    }
}

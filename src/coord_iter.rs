use crate::prelude::*;

pub struct CoordIter {
    cur:     Coord,
    x_start: i32,
    max:     Coord,
}

impl CoordIter {
    pub fn new(start: Coord, end: Coord) -> CoordIter {
        CoordIter {
            cur:     start,
            x_start: start.x,
            max:     end,
        }
    }

    pub fn skip_lines(mut self, lines: i32) -> Self {
        self.cur.y += lines;
        self
    }
}

impl Iterator for CoordIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.cur;
        if self.cur.y >= self.max.y { return None }

        self.cur.x += 1;

        if self.cur.x >= self.max.x {
            self.cur.x = self.x_start;
            self.cur.y += 1;
        }

        Some(temp)
    }
}
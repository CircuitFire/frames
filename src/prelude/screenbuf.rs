use crate::prelude::*;

pub struct Vec2D {
    values: Vec<Pixel>,
    size:   Coord,
}

impl Vec2D {
    pub fn new(size: Coord) -> Self {
        Vec2D {
            values: vec![Pixel::Clear; (size.y * size.x) as usize],
            size:   size,
        }
    }

    pub fn set_size(&mut self, size: Coord) {
        self.size = size;

        let flat = (size.x * size.y) as usize;

        if flat < self.values.len() {
            self.values.truncate(flat);
        }
        else {
            for _ in 0..(flat - self.values.len()) {
                self.values.push(Pixel::Clear);
            }
        }
    }

    pub fn set(&mut self, index: Coord, value: Pixel) {
        self.values[((index.y * self.size.x) + index.x) as usize] = value;
    }

    pub fn get_flat(&self, index: usize) -> Pixel {
        self.values[index]
    }

    pub fn get(&self, index: Coord) -> Pixel {
        self.values[((index.y * self.size.x) + index.x) as usize]
    }

    pub fn size(&self) -> Coord {
        self.size
    }

    pub fn values(&self) -> &Vec<Pixel> {
        &self.values
    }
}

/// start of the drawing area.
/// end of the drawing area.
/// offset is the difference between the drawing area and the frame area.
/// adding the offset to start should result in >= 0
#[derive(Clone, Copy)]
struct Pos {
    size:   Coord,
    offset: Coord,
}

pub struct ScreenBuf {
    pub buffer: Vec2D,
    modifiers:  Vec<Modifier>,
    pos:        Vec<Pos>,
}

impl ScreenBuf {
    pub fn new(size: Coord) -> Self {
        ScreenBuf {
            buffer:    Vec2D::new(size),
            modifiers: Vec::new(),
            pos:       Vec::new(),
        }
    }

    pub fn set_size(&mut self, size: Coord) {
        self.buffer.set_size(size);
    }

    pub fn set(&mut self, pos: Coord, pixel: Pixel) {
        if pixel == Pixel::Clear { return }

        let mut pos_pixel = PosPixel {
            pos: pos,
            pixel: pixel,
        };

        for modifier in self.modifiers.iter().rev() {
            //println!("in: {:?}", pos_pixel.pos);
            modifier.borrow_mut().modify(&mut pos_pixel);
        
            if pos_pixel.pixel == Pixel::Clear { return }
        }

        self.buffer.set(pos_pixel.pos, pos_pixel.pixel)
    }

    fn add_mod(&mut self, modifier: Modifier) {
        let pos_mod = modifier.borrow_mut().mod_position(self.size(), self.offset());
        let mut new_pos = if let Some(pos) = self.pos.last() {
            pos.clone()
        }
        else {
            Pos {
                size:   self.buffer.size(),
                offset: Coord{x: 0, y: 0}
            }
        };

        if let Some(size) = pos_mod.size {
            new_pos.size = size;
        }

        if let Some(offset) = pos_mod.offset {
            new_pos.offset = offset;
        }

        self.pos.push(new_pos);
        self.modifiers.push(modifier);
    }

    fn remove_mod(&mut self) {
        self.pos.pop();
        self.modifiers.pop();
    }

    pub fn use_modifier_on(&mut self, modifier: Modifier, frame: &Frame, mut offset: Coord, mut size: Coord) {
        {
            let mut modifier = modifier.borrow_mut();
            modifier.init(&self);
            offset = modifier.mod_offset(offset);
            size   = modifier.mod_size(size);
        }

        self.add_mod(modifier);

        if size.x > 0 && size.y > 0 {
            frame.borrow().get_draw_data(self, offset, size);
        }

        self.remove_mod();
    }

    pub fn end(&self) -> Coord {
        self.size() + self.offset()
    }

    pub fn offset(&self) -> Coord {
        if let Some(pos) = self.pos.last() {
            return pos.offset
        }

        Coord{x: 0, y: 0}
    }

    pub fn size(&self) -> Coord {
        if let Some(pos) = self.pos.last() {
            return pos.size
        }

        self.buffer.size()
    }

    pub fn draw_to(&self) -> CoordIter {
        CoordIter::new(
            self.offset(),
            self.end(),
        )
    }
}

pub struct CoordIter {
    cur:     Coord,
    x_start: i32,
    max:     Coord,
}

impl CoordIter {
    fn new(start: Coord, end: Coord) -> CoordIter {
        CoordIter {
            cur:     start,
            x_start: start.x,
            max:     end,
        }
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
//size stack is broken fix it!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_to() {
        let expected = vec![
            Coord{x: 0, y: 0}, Coord{x: 1, y: 0}, Coord{x: 2, y: 0}, Coord{x: 3, y: 0}, Coord{x: 4, y: 0},
            Coord{x: 0, y: 1}, Coord{x: 1, y: 1}, Coord{x: 2, y: 1}, Coord{x: 3, y: 1}, Coord{x: 4, y: 1},
            Coord{x: 0, y: 2}, Coord{x: 1, y: 2}, Coord{x: 2, y: 2}, Coord{x: 3, y: 2}, Coord{x: 4, y: 2},
            Coord{x: 0, y: 3}, Coord{x: 1, y: 3}, Coord{x: 2, y: 3}, Coord{x: 3, y: 3}, Coord{x: 4, y: 3},
            Coord{x: 0, y: 4}, Coord{x: 1, y: 4}, Coord{x: 2, y: 4}, Coord{x: 3, y: 4}, Coord{x: 4, y: 4},
        ];

        let buf = ScreenBuf::new(Coord{x: 5, y: 5});
        
        for (i, x) in buf.draw_to().enumerate() {
            println!("{}", i);
            assert_eq!(expected[i], x)
        }
    }
}
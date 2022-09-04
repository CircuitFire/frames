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

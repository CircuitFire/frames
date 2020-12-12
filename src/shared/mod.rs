extern crate crossterm;
pub use crossterm::style::Color;

extern crate coord;

pub use std::{
    rc::Rc, cell::RefCell
};

mod drawsegment;
pub use drawsegment::Drawsegment;

mod drawdata;
pub use drawdata::DrawData;

pub type Coord = coord::Coord<i32>;

#[derive(Copy, Clone, Debug)]
pub struct PixelData {
    pub character: char,
    pub fg: Color,
    pub bg: Color,
}

impl PixelData {
    pub fn new(character: char, fg: Color, bg: Color) -> PixelData {
        PixelData {
            character: character,
            fg: fg,
            bg: bg,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Pixel {
    Clear,
    Opaque(PixelData),
}

impl Pixel {
    pub fn new(character: char, fg: Color, bg: Color) -> Pixel {
        Pixel::Opaque(
            PixelData {
                character: character,
                fg: fg,
                bg: bg,
            }
        )
    }

    pub fn new_basic(character: char, fg: Color) -> Pixel {
        Pixel::Opaque(
            PixelData {
                character: character,
                fg: fg,
                bg: fg,
            }
        )
    }
}

pub trait Frame {
    fn size(&self) -> Option<Coord>;

    fn get_draw_data(&self, area: &Vec<Drawsegment>, offset: Coord, size: Coord) -> Vec<DrawData>;
}

#[derive(Copy, Clone, Debug)]
pub struct Rec {
    pub start: Coord,
    pub end: Coord,
}

impl Rec {
    pub fn in_range(&mut self, range: &Rec) -> bool {
        //checks if a rectangle is in range if it isnt returns false.
        //if it is trims off the parts that arnt and returns true.

        if ((self.end.y <= range.start.y) && (self.end.x <= range.start.x)) ||
        ((self.start.y >= range.end.y) && (self.start.x >= range.end.x)) {
            false
        }
        else {
            if self.start.y < range.start.y { self.start.y = range.start.y }
            if self.start.x < range.start.x { self.start.x = range.start.x }
            if self.end.y > range.end.y { self.end.y = range.end.y }
            if self.end.x > range.end.x { self.end.x = range.end.x }

            true
        }
    }

    pub fn pull_drawseg(&mut self) -> Option<Drawsegment> {
        if self.start.y < self.end.y {
            let seg = Drawsegment {
                start: self.start,
                len: (self.end.x - self.start.x) as usize,
            };

            self.start.y += 1;
            Some(seg)
        }
        else { None }
    }

    pub fn squish(&mut self) -> bool {
        self.start.y += 1;

        if self.start.y >= self.end.y { true }
        else { false }
    }
}

pub enum Task {
    UpdateAll,
    Update(Rec),
    UpdateMult(Vec<Rec>),
}

pub fn into_ref<T>(object: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(object))
}

pub fn clone_ref<T>(object: &Rc<RefCell<T>>) -> Rc<RefCell<T>> {
    Rc::clone(object)
}
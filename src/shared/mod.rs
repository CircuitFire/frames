extern crate crossterm;
pub use crossterm::style::Color;

use std::ops;

pub use std::{
    rc::Rc, cell::RefCell
};

mod drawsegment;
pub use drawsegment::Drawsegment;

mod drawdata;
pub use drawdata::DrawData;

#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl ops::Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Coord {
    fn add_assign(&mut self, other: Coord) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign for Coord {
    fn sub_assign(&mut self, other: Coord) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::Rem for Coord {
    type Output = Coord;

    fn rem(self, other: Coord) -> Coord {
        Coord {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl ops::RemAssign for Coord {
    fn rem_assign(&mut self, other: Coord) {
        self.x %= other.x;
        self.y %= other.y;
    }
}

impl ops::Mul for Coord {
    type Output = Coord;

    fn mul(self, other: Coord) -> Coord {
        Coord {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl ops::Div for Coord {
    type Output = Coord;

    fn div(self, other: Coord) -> Coord {
        Coord {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        if(self.x == other.x) && (self.y == other.x) { true }
        else { false }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PixleData {
    pub character: char,
    pub fg: Color,
    pub bg: Color,
}

impl PixleData {
    pub fn new(character: char, fg: Color, bg: Color) -> PixleData {
        PixleData {
            character: character,
            fg: fg,
            bg: bg,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Pixle {
    Clear,
    Opaque(PixleData),
}

impl Pixle {
    pub fn new(character: char, fg: Color, bg: Color) -> Pixle {
        Pixle::Opaque(
            PixleData {
                character: character,
                fg: fg,
                bg: bg,
            }
        )
    }

    pub fn new_basic(character: char, fg: Color) -> Pixle {
        Pixle::Opaque(
            PixleData {
                character: character,
                fg: fg,
                bg: fg,
            }
        )
    }
}

pub trait Frame {
    fn size(&self) -> Coord;

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
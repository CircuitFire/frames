pub use crossterm::style::Color;
pub use crossterm::event::{KeyEvent, MouseEvent};

pub use std::{
    rc::Rc, cell::RefCell
};

mod screenbuf;
pub use screenbuf::*;

pub type Coord = coord::Coord<i32>;

/// - get_draw_data
/// - update
pub trait IFrame {
    fn get_draw_data(&self, screen: &mut ScreenBuf, offset: Coord, size: Coord);

    fn update(&mut self, new_size: Coord) {}
}

pub type Frame = Rc<RefCell<dyn IFrame>>;

pub struct PosPixel {
    pub pos:   Coord,
    pub pixel: Pixel,
}

pub struct PositionModifier {
    pub size:   Option<Coord>,
    pub offset: Option<Coord>,
}

/// - init
/// - modify
/// - mod_position
/// - update
pub trait IModifier {
    /// Called when the modifier is added to the screen buffer.
    fn init(&mut self, screen: &ScreenBuf) {}

    /// Pixels getting written to the buffer are passed through this function before being written.
    fn modify(&mut self, pos_pixel: &mut PosPixel);

    /// If the modifier changes 
    fn mod_position(&mut self, cur_size: Coord, cur_offset: Coord) -> PositionModifier {
        PositionModifier {
            size:   None,
            offset: None,
        }
    }

    fn mod_size(&mut self, size: Coord) -> Coord {
        size
    }

    fn mod_offset(&mut self, offset: Coord) -> Coord {
        offset
    }

    fn update(&mut self, new_size: Coord) {}
}

pub type Modifier = Rc<RefCell<dyn IModifier>>;

#[derive(Copy, Clone)]
pub struct ColorSet {
    pub fg: Color,
    pub bg: Color,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

    pub fn new_color_set(character: char, colors: ColorSet) -> PixelData {
        PixelData {
            character: character,
            fg: colors.fg,
            bg: colors.bg,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

    pub fn new_color_set(character: char, colors: ColorSet) -> Pixel {
        Pixel::Opaque(
            PixelData {
                character: character,
                fg: colors.fg,
                bg: colors.bg,
            }
        )
    }
}

pub enum Input {
    KeyBoard(KeyEvent),
    Mouse(MouseEvent),
}
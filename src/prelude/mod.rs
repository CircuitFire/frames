pub use crossterm::style::Color;
pub use crossterm::event::{KeyEvent, MouseEvent};

pub use std::{
    rc::Rc, cell::RefCell
};

mod screenbuf;
pub use screenbuf::*;

pub type Coord = coord::Coord<i32>;

pub fn wrap<T>(x: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(x))
}

/// - get_draw_data
/// - update
pub trait IFrame {
    fn get_draw_data(&self, screen: &mut ScreenBuf, offset: Coord, size: Coord);

    fn update(&mut self, _new_size: Coord) {}
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
    fn init(&mut self, _screen: &ScreenBuf) {}

    /// Pixels getting written to the buffer are passed through this function before being written.
    fn modify(&mut self, pos_pixel: &mut PosPixel);

    /// If the modifier changes 
    fn mod_position(&mut self, _cur_size: Coord, _cur_offset: Coord) -> PositionModifier {
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

    fn update(&mut self, _new_size: Coord) {}
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
            character,
            fg,
            bg,
        }
    }

    pub fn new_color_set(character: char, colors: ColorSet) -> PixelData {
        PixelData {
            character,
            fg: colors.fg,
            bg: colors.bg,
        }
    }

    pub fn get_color_set(&self) -> ColorSet {
        ColorSet { fg: self.fg, bg: self.bg }
    }

    pub fn set_color_set(&mut self, colors: ColorSet) {
        self.fg = colors.fg;
        self.bg = colors.bg;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pixel {
    Clear,
    Opaque(PixelData),
}

impl Pixel {
    pub fn new(character: char, fg: Color, bg: Color) -> Pixel {
        Self::Opaque(
            PixelData {
                character,
                fg,
                bg,
            }
        )
    }

    pub fn new_color_set(character: char, colors: ColorSet) -> Pixel {
        Self::Opaque(
            PixelData {
                character,
                fg: colors.fg,
                bg: colors.bg,
            }
        )
    }

    pub fn as_ref(&self) -> Option<&PixelData> {
        match *self {
            Self::Opaque(ref x) => Some(x),
            Self::Clear => None,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut PixelData> {
        match *self {
            Self::Opaque(ref mut x) => Some(x),
            Self::Clear => None,
        }
    }
}

pub enum Input {
    KeyBoard(KeyEvent),
    Mouse(MouseEvent),
    FocusGained,
    FocusLost,
    Paste(String),
}
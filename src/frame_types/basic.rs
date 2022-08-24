use crate::prelude::*;

pub type Basic = Rc<RefCell<IBasic>>;

pub fn new(size: Coord, pixels: Vec<Pixel>) -> Result<Basic, &'static str> {
    match IBasic::new(size, pixels) {
        Ok(x) => Ok(Rc::new(RefCell::new(x))),
        Err(x) => Err(x),
    }
}

/// The most basic frame consisting of a vec of pixels and a size
/// ## Functions
/// - new
/// 
/// ## Methods
/// - replace
/// - get_pixel
/// - get_pixels
/// - set_pixel
pub struct IBasic {
    size: Coord,
    pixels: Vec<Pixel>,
}

impl IFrame for IBasic {
    fn get_draw_data(&self, screenbuf: &mut ScreenBuf, offset: Coord, _: Coord) {

        for pos in screenbuf.draw_to() {
            screenbuf.set(pos, self.get_pixel(pos + offset));
        }
    }
}

impl IBasic {
    pub fn new(size: Coord, pixels: Vec<Pixel>) -> Result<Self, &'static str> {
        if (size.x * size.y) != pixels.len() as i32 {
            Err("size != number of pixels.")
        }
        else {
            Ok(
                Self {
                    size: size,
                    pixels: pixels,
                }
            )
        }
    }

    pub fn replace(&mut self, size: Coord, pixels: Vec<Pixel>) -> Result<(), &'static str> {
        if (size.x * size.y) != pixels.len() as i32 {
            Err("size != number of pixels.")
        }
        else {
            *self = Self {
                size: size,
                pixels: pixels,
            };
            Ok(())
        }
    }

    pub fn get_pixel(&self, coord: Coord) -> Pixel {
        let new_coord = coord % self.size;
        self.pixels[self.flat_pos(new_coord)]
    }

    pub fn get_pixels(&self) -> &Vec<Pixel> {
        &self.pixels
    }

    fn flat_pos(&self, coord: Coord) -> usize {
        ((coord.y * self.size.x) + coord.x) as usize
    } 

    pub fn set_pixel(&mut self, coord: Coord, pixel: Pixel) {
        let index = self.flat_pos(coord);

        self.pixels[index] = pixel;
    }
}
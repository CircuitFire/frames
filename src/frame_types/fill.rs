use crate::prelude::*;
//use crate::object::update_types::MatchSize;

pub type Fill = Rc<RefCell<IFill>>;

pub fn new(pixel: Pixel) -> Fill {
    Rc::new(RefCell::new(IFill::new(pixel)))
}

/// fills whatever any area with the pixel it has
/// ## Functions
/// - new
/// - new_struct
/// 
/// ## Methods
/// - set_pixel
/// - get_pixel
pub struct IFill {
    pub pixel: Pixel,
}

impl IFrame for IFill {
    fn get_draw_data(&self, screenbuf: &mut ScreenBuf, _: Coord, _: Coord) {
        for pos in screenbuf.draw_to() {
            //println!("{:?}", pos);
            screenbuf.set(pos, self.pixel);
        }
    }
}

impl IFill {
    pub fn new(pixel: Pixel) -> Self {
        Self {
            pixel: pixel,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn fill() {
        let p = Pixel::new('x', Color::Red, Color::Black);

        let expected = vec![p; 25];

        let mut buf = ScreenBuf::new(Coord{x: 5, y: 5});

        let fill = new(p);

        fill.borrow().get_draw_data(&mut buf, Coord{x: 0, y: 0}, Coord{x: 0, y: 0});

        print_buffer(&buf);

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }
}
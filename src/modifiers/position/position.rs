use crate::prelude::*;
use std::cmp::{max, min};

pub trait SizeUpdate{
    fn size_update(&mut self, pos: &mut PosData, new_size: Coord);
}

pub type Position = Rc<RefCell<IPosition>>;

pub fn new() -> Position {
    Rc::new(RefCell::new(IPosition::new()))
}

pub fn craft() -> CraftPosition {
    CraftPosition{0: IPosition::new()}
}

pub struct CraftPosition (IPosition);

impl CraftPosition {
    pub fn pos(mut self, pos: Coord) -> Self {
        self.0.data.pos = pos;
        self
    }

    pub fn size(mut self, size: Coord) -> Self {
        self.0.data.size = size;
        self
    }

    pub fn offset(mut self, offset: Coord) -> Self {
        self.0.data.offset = offset;
        self
    }

    pub fn rot(mut self, rot: bool) -> Self {
        self.0.data.rot = rot;
        self
    }

    pub fn yflip(mut self, yflip: bool) -> Self {
        self.0.data.yflip = yflip;
        self
    }

    pub fn xflip(mut self, xflip: bool) -> Self {
        self.0.data.xflip = xflip;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.0.data.enabled = enabled;
        self
    }

    pub fn update<T: SizeUpdate + Sized + 'static>(mut self, update: T) -> Self {
        self.0.set_update(update);
        self
    }

    pub fn done(self) -> Position {
        Rc::new(RefCell::new(self.0))
    }
}

pub struct PosData {
    pub pos: Coord,
    pub size: Coord,
    pub offset: Coord,
    pub rot: bool,
    pub yflip: bool,
    pub xflip: bool,
    pub enabled: bool,
}

/// An Object holds a reference to a frame and all of the positional data for how it is drawn onto the screen.
/// ## Functions
/// - new
/// 
/// ## Methods
/// - rot_cw
/// - rot_ccw
/// - rot_180
/// - get_frame
/// - set_frame_struct
/// - set_frame_rc
/// - inc_offset
/// - update_size
pub struct IPosition {
    pub data: PosData,
    true_size: Coord,
    pub size_update: Option<Box<dyn SizeUpdate>>,
}

impl IModifier for IPosition {
    fn modify(&mut self, pos_pixel: &mut PosPixel) {
        pos_pixel.pos = self.translate_coord(pos_pixel.pos, self.true_size) + self.data.pos;
    }

    fn mod_position(&mut self, cur_size: Coord, _cur_offset: Coord) -> PositionModifier {
        let end   = self.data.pos + self.match_rot(self.data.size);

        let new_start = take_bigger(self.data.pos, Coord{x: 0, y: 0});
        let new_end   = take_smaller(end, self.match_rot(cur_size));

        self.true_size = new_end - new_start;

        PositionModifier {
            size:   Some(self.true_size),
            offset: Some(self.calc_offset()),
        }
    }

    fn mod_offset(&mut self, _offset: Coord) -> Coord {
        self.data.offset
    }

    fn mod_size(&mut self, _size: Coord) -> Coord {
        self.data.size
    }

    fn update(&mut self, new_size: Coord) {
        if let Some(func) = &mut self.size_update {
            func.size_update(&mut self.data, new_size);
        }
    }
}

impl IPosition {
    /// Create a new Object manually setting each of the properties.
    pub fn new() -> Self {
        IPosition {
            data: PosData {
                pos:         Coord{x:0, y:0},
                size:        Coord{x:0, y:0},
                offset:      Coord{x:0, y:0},
                rot:         false,
                yflip:       false,
                xflip:       false,
                enabled:     true,
            },
            true_size:   Coord{x:0, y:0},
            size_update: None,
        }
    }

    pub fn set_update<T: SizeUpdate + Sized + 'static>(&mut self, update: T) {
        self.size_update = Some(Box::new(update))
    }

    pub fn remove_update(&mut self) {
        self.size_update = None
    }

    /// Sets the Objects Center position of the object to the provided coord.
    pub fn set_center(&mut self, new: Coord) {
        self.data.pos = new - (self.data.size / Coord{ x: 2, y: 2 });
    }

    /// Flips the frame over the x axis.
    pub fn flipx(&mut self) {
        self.data.xflip = !self.data.xflip;
    }

    /// Flips the frame over the y axis.
    pub fn flipy(&mut self) {
        self.data.yflip = !self.data.yflip;
    }

    /// rotates the frame clockwise.
    pub fn rot_cw(&mut self) {
        if self.data.rot { self.data.xflip = !self.data.xflip; }
        else { self.data.yflip = !self.data.yflip }
        self.data.rot = !self.data.rot;
    }

    /// rotates the frame counter clockwise.
    pub fn rot_ccw(&mut self) {
        if !self.data.rot { self.data.xflip = !self.data.xflip; }
        else { self.data.yflip = !self.data.yflip }
        self.data.rot = !self.data.rot;
    }

    /// rotates the frame 180 degrees.
    pub fn rot_180(&mut self) {
        self.data.yflip = !self.data.yflip;
        if !self.data.rot {
            self.data.xflip = !self.data.xflip;
        }
    }

    /// Increments the offset of the frame by the given amount.
    pub fn inc_offset(&mut self, inc: &Coord) {
        self.data.offset += *inc;
        self.data.offset %= self.data.size * Coord{ x: 2, y: 2 };
    }

    fn calc_offset(&self) -> Coord {
        Coord {
            x: min(self.data.pos.x, 0) * -1,
            y: min(self.data.pos.y, 0) * -1,
        }
    }

    fn match_rot(&self, c: Coord) -> Coord {
        if self.data.rot { rot_coord(c) }
                    else { c }
    }

    fn translate_coord(&self, index: Coord, size: Coord) -> Coord {
        let temp = Coord {
            x: if self.data.xflip { size.x - index.x }
                             else { index.x },
            y: if self.data.yflip { size.y - index.y }
                             else { index.y },
        };

        self.match_rot(temp)
    }
}

fn take_smaller(c1: Coord, c2: Coord) -> Coord {
    Coord {
        x: min(c1.x, c2.x),
        y: min(c1.y, c2.y),
    }
}

fn take_bigger(c1: Coord, c2: Coord) -> Coord {
    Coord {
        x: max(c1.x, c2.x),
        y: max(c1.y, c2.y),
    }
}

fn rot_coord(c: Coord) -> Coord {
    Coord {
        x: c.y,
        y: c.x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use crate::frame_types::{basic, with_modifier};
    use Color::Rgb;

    fn test_smile() -> basic::Basic {
        let b = Pixel::new('B', Rgb{r: 0, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let r = Pixel::new('R', Rgb{r: 255, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let w = Pixel::new('W', Rgb{r: 255, g: 255, b: 255}, Rgb{r: 0, g: 0, b: 0});

        let sprite = vec![
            b,r,r,r,w,w,w,b,
            r,b,b,r,w,b,b,w,
            r,b,b,r,w,b,b,w,
            r,r,r,r,w,w,w,w,
            r,b,r,r,w,w,b,w,
            r,r,b,b,b,b,w,w,
            b,r,r,r,w,w,w,b,
        ];

        basic::new(Coord{x: 8, y: 7}, sprite).unwrap()
    }

    #[test]
    fn no_move() {
        let mut buf = ScreenBuf::new(Coord{x: 9, y: 8});

        let smile = with_modifier::new(test_smile(), craft().size(Coord{x: 8, y: 7}).done());

        smile.borrow().get_draw_data(&mut buf, Coord{x: 0, y: 0}, Coord{x: 9, y: 8});

        let b = Pixel::new('B', Rgb{r: 0, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let r = Pixel::new('R', Rgb{r: 255, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let w = Pixel::new('W', Rgb{r: 255, g: 255, b: 255}, Rgb{r: 0, g: 0, b: 0});
        let c = Pixel::Clear;

        print_buffer(&buf);

        let expected = vec![
            b,r,r,r,w,w,w,b,c,
            r,b,b,r,w,b,b,w,c,
            r,b,b,r,w,b,b,w,c,
            r,r,r,r,w,w,w,w,c,
            r,b,r,r,w,w,b,w,c,
            r,r,b,b,b,b,w,w,c,
            b,r,r,r,w,w,w,b,c,
            c,c,c,c,c,c,c,c,c,
        ];

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }

    #[test]
    fn move_in_range() {
        let mut buf = ScreenBuf::new(Coord{x: 9, y: 8});

        let smile = with_modifier::new(test_smile(), craft().size(Coord{x: 8, y: 7}).pos(Coord{x: 1, y: 1}).done());

        smile.borrow().get_draw_data(&mut buf, Coord{x: 0, y: 0}, Coord{x: 9, y: 8});

        let b = Pixel::new('B', Rgb{r: 0, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let r = Pixel::new('R', Rgb{r: 255, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let w = Pixel::new('W', Rgb{r: 255, g: 255, b: 255}, Rgb{r: 0, g: 0, b: 0});
        let c = Pixel::Clear;

        let expected = vec![
            c,c,c,c,c,c,c,c,c,
            c,b,r,r,r,w,w,w,b,
            c,r,b,b,r,w,b,b,w,
            c,r,b,b,r,w,b,b,w,
            c,r,r,r,r,w,w,w,w,
            c,r,b,r,r,w,w,b,w,
            c,r,r,b,b,b,b,w,w,
            c,b,r,r,r,w,w,w,b,
        ];

        print_buffer(&buf);

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }

    #[test]
    fn move_out_neg_range() {
        let mut buf = ScreenBuf::new(Coord{x: 9, y: 8});

        let smile = with_modifier::new(test_smile(), craft().size(Coord{x: 8, y: 7}).pos(Coord{x: -1, y: -1}).done());

        smile.borrow().get_draw_data(&mut buf, Coord{x: 0, y: 0}, Coord{x: 9, y: 8});

        let b = Pixel::new('B', Rgb{r: 0, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let r = Pixel::new('R', Rgb{r: 255, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let w = Pixel::new('W', Rgb{r: 255, g: 255, b: 255}, Rgb{r: 0, g: 0, b: 0});
        let c = Pixel::Clear;

        let expected = vec![
            b,b,r,w,b,b,w,c,c,
            b,b,r,w,b,b,w,c,c,
            r,r,r,w,w,w,w,c,c,
            b,r,r,w,w,b,w,c,c,
            r,b,b,b,b,w,w,c,c,
            r,r,r,w,w,w,b,c,c,
            c,c,c,c,c,c,c,c,c,
            c,c,c,c,c,c,c,c,c,
        ];

        print_buffer(&buf);
        
        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }

    #[test]
    fn move_out_pos_range() {
        let mut buf = ScreenBuf::new(Coord{x: 9, y: 8});

        let smile = with_modifier::new(test_smile(), craft().size(Coord{x: 8, y: 7}).pos(Coord{x: 2, y: 2}).done());

        smile.borrow().get_draw_data(&mut buf, Coord{x: 0, y: 0}, Coord{x: 9, y: 8});

        let b = Pixel::new('B', Rgb{r: 0, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let r = Pixel::new('R', Rgb{r: 255, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let w = Pixel::new('W', Rgb{r: 255, g: 255, b: 255}, Rgb{r: 0, g: 0, b: 0});
        let c = Pixel::Clear;

        let expected = vec![
            c,c,c,c,c,c,c,c,c,
            c,c,c,c,c,c,c,c,c,
            c,c,b,r,r,r,w,w,w,
            c,c,r,b,b,r,w,b,b,
            c,c,r,b,b,r,w,b,b,
            c,c,r,r,r,r,w,w,w,
            c,c,r,b,r,r,w,w,b,
            c,c,r,r,b,b,b,b,w,
        ];

        print_buffer(&buf);
        
        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }

    #[test]
    fn move_fill_range() {
        let mut buf = ScreenBuf::new(Coord{x: 9, y: 8});

        let smile = with_modifier::new(test_smile(), craft().size(Coord{x: 9, y: 8}).done());

        smile.borrow().get_draw_data(&mut buf, Coord{x: 0, y: 0}, Coord{x: 9, y: 8});

        let b = Pixel::new('B', Rgb{r: 0, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let r = Pixel::new('R', Rgb{r: 255, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
        let w = Pixel::new('W', Rgb{r: 255, g: 255, b: 255}, Rgb{r: 0, g: 0, b: 0});
        let c = Pixel::Clear;

        let expected = vec![
            b,r,r,r,w,w,w,b,b,
            r,b,b,r,w,b,b,w,r,
            r,b,b,r,w,b,b,w,r,
            r,r,r,r,w,w,w,w,r,
            r,b,r,r,w,w,b,w,r,
            r,r,b,b,b,b,w,w,r,
            b,r,r,r,w,w,w,b,b,
            b,r,r,r,w,w,w,b,b,
        ];

        print_buffer(&buf);
        
        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }
}
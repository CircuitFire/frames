use crate::prelude::*;

use std::cmp::max;

pub type CircleMask = Rc<RefCell<ICircleMask>>;

pub fn new(invert: bool) -> CircleMask {
    Rc::new(RefCell::new(ICircleMask::new(invert)))
}

enum Kind {
    Even,
    Odd,
}

pub struct ICircleMask {
    kind:       Kind,
    center:     Coord,
    radius_sqr: i32,
    invert:     bool,
}

impl IModifier for ICircleMask {
    fn init(&mut self, screen: &ScreenBuf) {
        let size = screen.size();

        let biggest = max(size.x, size.y);

        let radius = biggest / 2;

        if biggest % 2 == 0 { self.kind = Kind::Even }
                       else { self.kind = Kind::Odd }

        self.center = Coord {
            x: size.x / 2,
            y: size.y / 2,
        };
        self.radius_sqr = radius * radius;
    }

    fn modify(&mut self, pos_pixel: &mut PosPixel) {
        match self.kind {
            Kind::Even => {
                if (calc_dis(pos_pixel.pos, self.center) < self.radius_sqr) ^ self.invert {
                    return
                }
            }
            Kind::Odd => {
                if (calc_dis(pos_pixel.pos, self.center) <= self.radius_sqr) ^ self.invert {
                    return
                }
            }
        }

        pos_pixel.pixel = Pixel::Clear;
    }
}

impl ICircleMask {
    pub fn new(invert: bool) -> Self {
        Self {
            kind:       Kind::Even,
            center:     Coord{x: 0, y: 0},
            radius_sqr: 0,
            invert:     invert,
        }
    }
}

fn calc_dis(pos: Coord, center: Coord) -> i32 {
    let rel_x = pos.x - center.x;
    let rel_y = pos.y - center.y;
    (rel_x * rel_x) + (rel_y * rel_y)
}
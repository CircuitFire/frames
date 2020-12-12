use crate::shared::*;
use crate::mask_rules::*;

pub struct Circle {}

impl Circle {
    pub fn new() -> Box<Self> {
        Box::new(Circle{})
    }
}

impl MaskRule for Circle {
    fn init(&self, size: Coord) -> Box<dyn MaskLogic> {
        let biggest = if size.x >= size.y { size.x }
                      else                { size.y };

        let radius = biggest / 2;

        Box::new(CircleLogic{
            even: if biggest % 2 == 0 { true }
                  else                { false },
            center: Coord{
                x: size.x / 2,
                y: size.y / 2,
            },
            radius_sqr: radius * radius,
        })
    }
}

struct CircleLogic {
    even: bool,
    center: Coord,
    radius_sqr: i32,
}

impl MaskLogic for CircleLogic {
    fn mask(&self, pos: Coord) -> bool {
        let mut mask = false;

        let rel_x = pos.x - self.center.x;
        let rel_y = pos.y - self.center.y;
        let dis = (rel_x * rel_x) + (rel_y * rel_y);


        if self.even {
            if dis >= self.radius_sqr {
                mask = true;
            }
        }
        else {
            if dis > self.radius_sqr {
                mask = true;
            }
        }

        mask
    }
}
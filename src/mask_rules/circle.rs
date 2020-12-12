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

        if biggest % 2 == 0 {
            Box::new(CircleLogicEven{
                center: Coord{
                    x: size.x / 2,
                    y: size.y / 2,
                },
                radius_sqr: radius * radius,
            })
        }
        else{
            Box::new(CircleLogicOdd{
                center: Coord{
                    x: size.x / 2,
                    y: size.y / 2,
                },
                radius_sqr: radius * radius,
            })
        }
        
    }
}

struct CircleLogicEven {
    center: Coord,
    radius_sqr: i32,
}

impl MaskLogic for CircleLogicEven {
    fn mask(&self, pos: Coord) -> bool {
        if calc_dis(pos, self.center) >= self.radius_sqr {
            true
        }
        else{
            false
        }
    }
}

struct CircleLogicOdd {
    center: Coord,
    radius_sqr: i32,
}

impl MaskLogic for CircleLogicOdd {
    fn mask(&self, pos: Coord) -> bool {
        if calc_dis(pos, self.center) > self.radius_sqr {
            true
        }
        else{
            false
        }
    }
}

fn calc_dis(pos: Coord, center: Coord) -> i32 {
    let rel_x = pos.x - center.x;
    let rel_y = pos.y - center.y;
    (rel_x * rel_x) + (rel_y * rel_y)
}
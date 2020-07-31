use crate::shared::*;
use crate::mask_rules::MaskRule;

pub struct Circle {}

impl MaskRule for Circle {
    fn show(&self, data: &mut Vec<DrawData>, fill: &Pixle, size: Coord){
        
        let bigest = if size.x >= size.y { size.x }
                     else                { size.y };
        
        let even = if bigest % 2 == 0 { true }
                  else                { false };

        let center_x = size.x / 2;
        let center_y = size.y / 2;
        let radius = bigest / 2;

        for seg in data.iter_mut() {
            let mut x_off = 0;

            for pix in seg.data.iter_mut() {
                let rel_x = (seg.start.x + x_off) - center_x;
                let rel_y = seg.start.y - center_y;
                let dis = (rel_x * rel_x) + (rel_y * rel_y);


                if even {
                    if dis >= radius * radius {
                        *pix = *fill;
                    }
                }
                else {
                    if dis > radius * radius {
                        *pix = *fill;
                    }
                }
                
                x_off += 1;
            }
        }
    }
}

impl Circle {
    pub fn new() -> Box<Self> {
        Box::new(Circle{})
    }
}
use crate::shared::*;
use crate::mask_rules::*;

/// contains a frame and only displays the portion set by the rule
/// ## Functions
/// - new
/// 
/// ## Methods
/// - set_frame
/// - set_pixel
/// - set_rule
/// - set_invert
/// - toggle_invert
pub struct Mask {
    frame: Rc<RefCell<dyn Frame>>,
    pixel: Pixel,
    rule: Box<dyn MaskRule>,
    invert: bool,
}

impl Frame for Mask {
    fn size(&self) -> Option<Coord> {
        self.frame.borrow().size()
    }

    fn get_draw_data(&self, area: &Vec<Drawsegment>, offset: Coord, size: Coord) -> Vec<DrawData> {

        let mut data = self.frame.borrow().get_draw_data(area, offset, size);

        let rule = self.rule.init(size);

        for seg in data.iter_mut() {
            let mut pos = seg.start;
            for pix in seg.data.iter_mut() {
                if rule.mask(pos) ^ self.invert { *pix = self.pixel; }
                pos.x += 1;
            }
        }

        data
    }
}

impl Mask {
    pub fn new(frame: Rc<RefCell<dyn Frame>>, pixel: Pixel, rule: Box<dyn MaskRule>, invert_rule: bool) -> Rc<RefCell<Mask>> {
        Rc::new(RefCell::new(
            Mask {
                frame: frame,
                pixel: pixel,
                rule: rule,
                invert: invert_rule,
            }
        ))
    }

    pub fn set_frame(&mut self, frame: Rc<RefCell<dyn Frame>>) {
        self.frame = frame;
    }

    pub fn set_pixel(&mut self, pixel: &Pixel) {
        self.pixel = *pixel;
    }

    pub fn set_rule(&mut self, rule: Box<dyn MaskRule>) {
        self.rule = rule;
    }

    pub fn set_invert(&mut self, invert_rule: bool) {
        self.invert = invert_rule;
    }

    pub fn toggle_invert(&mut self) {
        self.invert = !self.invert;
    }
}





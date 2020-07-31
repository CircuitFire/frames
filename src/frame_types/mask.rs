use crate::shared::*;
use crate::mask_rules::MaskRule;

pub struct Mask {
    frame: Rc<RefCell<dyn Frame>>,
    pixle: Pixle,
    rule: Box<dyn MaskRule>,
}

impl Frame for Mask {
    fn size(&self) -> Coord {
        self.frame.borrow().size()
    }

    fn get_draw_data(&self, area: &Vec<Drawsegment>, offset: Coord, size: Coord) -> Vec<DrawData> {

        let mut data = self.frame.borrow().get_draw_data(area, offset, size);

        self.rule.show(&mut data, &self.pixle, size);

        data
    }
}

impl Mask {
    pub fn new(frame: Rc<RefCell<dyn Frame>>, pixle: Pixle, rule: Box<dyn MaskRule>) -> Rc<RefCell<Mask>> {
        Rc::new(RefCell::new(
            Mask {
                frame: frame,
                pixle: pixle,
                rule: rule,
            }
        ))
    }

    //pub fn new_struct(pixle: &Pixle) -> Fill {
    //    Fill {
    //        pixle: *pixle,
    //    }
    //}
//
    //pub fn set_pixle(&mut self, pixle: &Pixle) {
    //    self.pixle = *pixle;
    //}
}





use crate::prelude::*;

pub type WithModifier = Rc<RefCell<IWithModifier>>;

pub fn new(frame: Frame, modifier: Modifier) -> WithModifier {
    Rc::new(RefCell::new(IWithModifier::new(frame, modifier)))
}

pub struct IWithModifier{
    pub frame:    Frame,
    pub modifier: Modifier,
}

impl IFrame for IWithModifier {
    fn get_draw_data(&self, screen: &mut ScreenBuf, offset: Coord, size: Coord) {
        screen.use_modifier_on(self.modifier.clone(), &self.frame, offset, size)
    }

    fn update(&mut self, new_size: Coord) {
        self.frame.borrow_mut().update(new_size);
        self.modifier.borrow_mut().update(new_size);
    }
}

impl IWithModifier {
    pub fn new(frame: Frame, modifier: Modifier) -> Self {
        Self {
            frame: frame,
            modifier: modifier,
        }
    }
}
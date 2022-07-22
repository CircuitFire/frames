use crate::prelude::*;
use crate::modifiers::position::Position;
use crate::frame_types::fill;
use crate::modifiers::position;

pub struct Object {
    pub frame: Frame,
    pub pos:   Position,
}

pub type Layout = Rc<RefCell<ILayout>>;

pub fn new() -> Layout {
    Rc::new(RefCell::new(ILayout::new()))
}

pub struct ILayout {
    pub objects: Vec<Object>,
}

impl IFrame for ILayout {
    fn get_draw_data(&self, screenbuf: &mut ScreenBuf, offset: Coord, size: Coord) {
        for obj in &self.objects {
            if obj.pos.borrow().data.enabled {
                screenbuf.use_modifier_on(obj.pos.clone(), &obj.frame, offset, size);
            }
        }
    }

    fn update(&mut self, new_position: Coord) {
        for obj in &self.objects {
            obj.pos.borrow_mut().update(new_position);
            obj.frame.borrow_mut().update(new_position);
        }
    }
}

impl ILayout {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn add_background(&mut self, pixel: Pixel) {
        let temp = position::new();
        temp.borrow_mut().set_update(position::update_types::MatchSize{});

        self.objects.insert(0, 
            Object {
                frame: fill::new(pixel),
                pos: temp,
            }
        )
    }
}
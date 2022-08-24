use crate::prelude::*;

pub type Bundle = Rc<RefCell<IBundle>>;

pub fn new() -> Bundle {
    wrap(IBundle::new())
}

/// holds multiple frames and displays the one indicated by the index
/// ## Functions
/// - new
/// 
/// ## Methods
/// - frames
/// - set_index
/// - inc_index
pub struct IBundle {
    pub frames: Vec<Frame>,
    pub index: usize,
}

impl IFrame for IBundle {
    fn get_draw_data(&self, screenbuf: &mut ScreenBuf, offset: Coord, size: Coord){
        self.frames[self.index].borrow().get_draw_data(screenbuf, offset, size)
    }
}

impl IBundle {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            index: 0,
        }
    }

    pub fn inc_index(&mut self, mut inc: i32) {
        inc %= self.frames.len() as i32;

        if inc < 0 { inc += self.frames.len() as i32 }

        self.index = inc as usize;
    }
}
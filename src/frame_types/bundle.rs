use crate::shared::*;

/// holds multiple frames and displays the one indicated by the index
/// ## Functions
/// - new
/// 
/// ## Methods
/// - frames
/// - set_index
/// - inc_index
pub struct Bundle {
    pub frames: Vec<Rc<RefCell<dyn Frame>>>,
    pub index: usize,
}

impl Frame for Bundle {
    fn size(&self) -> Option<Coord> {
        self.frames[self.index].borrow().size()
    }

    fn get_draw_data(&self, area: &Vec<Drawsegment>, offset: Coord, size: Coord) -> Vec<DrawData> {
        self.frames[self.index].borrow().get_draw_data(area, offset, size)
    }
}

impl Bundle {
    pub fn new() -> Rc<RefCell<Bundle>> {
        Rc::new(RefCell::new(
            Bundle {
                frames: Vec::new(),
                index: 0,
            }
        ))
    }

    pub fn inc_index(&mut self, mut inc: i32) {
        inc %= self.frames.len() as i32;

        if inc < 0 { inc += self.frames.len() as i32 }

        self.index = inc as usize;
    }
}
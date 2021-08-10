use crate::shared::*;

pub trait SizeUpdate{
    fn size_update(&mut self, new_size: &Coord, pos: &mut Coord, size: &mut Coord, offset: &mut Coord, enabled: &mut bool);
}

/// An Object holds a reference to a frame and all of the positional data for how it is drawn onto the screen.
/// ## Functions
/// - new
/// - new_basic
/// - new_min
/// - new_frame_size
/// 
/// ## Methods
/// - rot_cw
/// - rot_ccw
/// - rot_180
/// - get_frame
/// - set_frame_struct
/// - set_frame_rc
/// - inc_offset
/// - is_enabled
/// - update_size
pub struct Object {
    pub frame: Rc<RefCell<dyn Frame>>,
    pub pos: Coord,
    pub size: Coord,
    pub offset: Coord,
    pub rot: bool,
    pub yflip: bool,
    pub xflip: bool,
    pub enabled: bool,
    pub size_update: Option<Box<dyn SizeUpdate>>,
}

impl Object {
    /// Create a new Object manually setting each of the properties.
    pub fn new(frame: Rc<RefCell<dyn Frame>>, pos: Coord, size: Coord, offset: Coord, rot: bool, yflip: bool, xflip: bool, enabled: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Object {
                frame: frame,
                pos: pos,
                size: size,
                offset: offset,
                rot: rot,
                yflip: yflip,
                xflip: xflip,
                enabled: enabled,
                size_update: None,
            }
        ))
    }

    /// Create a new Object with the default orientation.
    pub fn new_basic(frame: Rc<RefCell<dyn Frame>>, size: Coord) -> Rc<RefCell<Self>> {
        Object::new(frame, Coord{x:0, y:0}, size, Coord{x:0, y:0}, false, false, false, true)
    }

    /// Create a new Object with the default orientation.
    pub fn new_min(frame: Rc<RefCell<dyn Frame>>) -> Rc<RefCell<Self>> {
        Object::new(frame, Coord{x:0, y:0}, Coord{x:0, y:0}, Coord{x:0, y:0}, false, false, false, true)
    }

    /// Create a new Object with the default orientation and matching the size of the given frame.
    /// Some frames don't have normal sizes like Fill and Text.
    pub fn new_frame_sized(frame: Rc<RefCell<dyn Frame>>) -> Option<Rc<RefCell<Self>>> {
        let size = frame.borrow().size();
        
        if let Some(size) = size {
            Some(Object::new(frame, Coord{x:0, y:0}, size, Coord{x:0, y:0}, false, false, false, true))
        }
        else{
            None
        }
    }

    /// Gets called by the manager every time the the screen size is updated.
    pub fn size_update(&mut self, new_size: &Coord){
        if let Some(func) = &mut self.size_update {
            func.size_update(new_size, &mut self.pos, &mut self.size, &mut self.offset, &mut self.enabled);
        }
    }

    /// Sets the Objects Center position of the object to the provided coord.
    pub fn set_center(&mut self, new: Coord) {
        self.pos = new - (self.size / Coord{ x: 2, y: 2 });
    }

    /// Flips the frame over the x axis.
    pub fn flipx(&mut self) {
        self.xflip = !self.xflip;
    }

    /// Flips the frame over the y axis.
    pub fn flipy(&mut self) {
        self.yflip = !self.yflip;
    }

    /// rotates the frame clockwise.
    pub fn rot_cw(&mut self) {
        if self.rot { self.xflip = !self.xflip; }
        else { self.yflip = !self.yflip }
        self.rot = !self.rot;
    }

    /// rotates the frame counter clockwise.
    pub fn rot_ccw(&mut self) {
        if !self.rot { self.xflip = !self.xflip; }
        else { self.yflip = !self.yflip }
        self.rot = !self.rot;
    }

    /// rotates the frame 180 degrees.
    pub fn rot_180(&mut self) {
        self.yflip = !self.yflip;
        if !self.rot {
            self.xflip = !self.xflip;
        }
    }

    /// Sets the Objects frame using an object.
    pub fn set_frame_struct<T: Frame + 'static>(&mut self, frame: T) -> &Rc<RefCell<dyn Frame>> {
        self.frame = Rc::new(RefCell::new(frame));
        &self.frame
    }

    /// Increments the offset of the frame by the given amount.
    pub fn inc_offset(&mut self, inc: &Coord) {
        self.offset += *inc;
        self.offset %= self.size * Coord{ x: 2, y: 2 };
    }

    /// Offsets and flips the local drawsegs depending on the objects orientation before being fed into the local frame.
    fn translate(&self, area: &mut Vec<Drawsegment>) -> Option<Vec<Drawsegment>> {
        if self.yflip {
            for seg in area.iter_mut() {
                seg.start.x = self.size.x - seg.end_pos();
            }
        }

        if self.xflip {
            for seg in area.iter_mut() {
                seg.start.y = self.size.y - (seg.start.y + 1);
            }
            Drawsegment::sort(area, self.size.x);
        }

        if self.rot {
            let old = area.clone();

            *area = Drawsegment::make_vertical(area);

            Some(old)
        }
        else { None }
    }

    /// Reverts the translated positions beck to their original position so they match with the full area.
    fn translate_back(&self, drawdata: &mut Vec<DrawData>, old_segs: Option<Vec<Drawsegment>>) {
        if let Some(old) = old_segs {
            *drawdata = DrawData::make_vertical(drawdata, &old);
        }

        if self.yflip {
            for seg in drawdata.iter_mut() {
                seg.start.x = self.size.x - (seg.start.x + seg.data.len() as i32);
                seg.data.reverse();
            }
        }

        if self.xflip {
            for seg in drawdata.iter_mut() {
                seg.start.y = self.size.y - (seg.start.y + 1);
            }
            DrawData::sort(drawdata, self.size.x);
        }

        for data in drawdata {
            data.start += self.pos;
        }
    }

    /// Takes in the area being drawn to and copies the relevant local data from the held frame.
    pub fn get_draw_data(&self, mut drawsegs: &mut Vec<DrawData>) { 
        let mut local_area = self.local_area(drawsegs);
        if !local_area.is_empty() {

            let old_area = self.translate(&mut local_area);

            //println!("{:?}", local_area);

            let mut local_data = self.frame.borrow().get_draw_data(&local_area, self.offset, self.size);

            //for seg in &local_data {
            //    println!("start: {:?} , len: {}", seg.start, seg.data.len());
            //}

            self.translate_back(&mut local_data, old_area);

            //println!("moved:");
            //for seg in &local_data {
            //    println!("start: {:?} , len: {}", seg.start, seg.data.len());
            //}

            copy_onto_area(&mut drawsegs, &local_data);
        }
    }

    /// Makes drawsegments that overlap with the total area and the objects local area.
    fn local_area(&self, area: &Vec<DrawData>) -> Vec<Drawsegment> {
        let mut local_area: Vec<Drawsegment> = Vec::new();
        let max = self.pos + self.size;
        //println!("========================================================================================");
        //println!("pos: {:?}, size: {:?}, max: {:?}", self.pos, self.size, max);

        for segment in area {
            if  ((self.pos.y <= segment.start.y) && (segment.start.y < max.y)) && 
            //  ^- check if y is in range        V- check if line is in range
                !((segment.end_pos() < self.pos.x) || (max.x <= segment.start.x)) {

                //println!("seg: \n\tstart: {:?}\n\tend: {}", segment.start, seg_end);
                let mut newseg = segment.make_drawseg();
                let old_endpos = newseg.end_pos();
                if newseg.start.x < self.pos.x {
                    newseg.start.x = self.pos.x
                }
                newseg.set_len_end_pos(old_endpos);
                if newseg.end_pos() > max.x {
                    newseg.set_len_end_pos(max.x);
                }
                //println!("pre: {:?}", newseg);
                //translate position
                newseg.start -= self.pos;
                //println!("aft: {:?}\n", newseg);
                local_area.push(newseg);
            }
        }
        //println!("locals:\n{:?}", local_area);
        local_area
    }

    /// Outputs a update task for the frame manager containing the area of the object.
    pub fn update(&self) -> Task {
        Task::Update(self.get_rec())
    }

    /// Outputs a update task for the frame manager containing the areas of the objects old and new position.
    pub fn move_to(&mut self, newpos: Coord) -> Task {
        let old_rec = self.get_rec();
        self.pos = newpos;

        //println!("old: {:?}\n\nnew:{:?}", old_rec, self.get_rec());
        Task::UpdateMult(
            vec![old_rec, self.get_rec()]
        )
    }
    
    /// Makes a rectangle struct of the area the object is in.
    pub fn get_rec(&self) -> Rec {
        Rec {
            start: self.pos,
            end: self.pos + self.size,
        }
    }
}

/// Copies the draw data gotten from the local frame into the full segment being updated.
fn copy_onto_area(drawsegs: &mut Vec<DrawData>, local_data: &Vec<DrawData>) {
    let mut i = 0;
    for seg in local_data {
        while !seg.overlaps(&drawsegs[i]) {
            i += 1;
        }

        let mut j = (seg.start.x - drawsegs[i].start.x) as usize;
        for pixel in &seg.data {
            match pixel {
                Pixel::Opaque(_) => {
                    drawsegs[i].data[j] = *pixel;
                },
                _ => (),
            }
            j += 1;
        }
    }
}
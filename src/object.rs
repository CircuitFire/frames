use crate::shared::*;

/// An Object holds a reference to a frame and all of the positional data for how it is drawn onto the screen.
/// ## Functions
/// - new
/// - new_basic
/// - new_min
/// 
/// ## Methods
/// - set_enabled
/// - set_pos
/// - set_center
/// - set_size
/// - set_offset
/// - set_yflip
/// - set_xflip
/// - rot_cw
/// - rot_ccw
/// - rot_180
/// - get_frame
/// - set_frame_struct
/// - set_frame_rc
/// - inc_offset
/// - is_enabled
/// - get_pos
/// - get_size
/// - get_offset
/// - get_center
/// - get_rot
/// - get_xflip
/// - get_yflip
pub struct Object {
    frame: Rc<RefCell<dyn Frame>>,
    pos: Coord,
    size: Coord,
    offset: Coord,
    rot: bool,
    yflip: bool,
    xflip: bool,
    enabled: bool,
}

impl Object {
    /// Creates creates an Object with all unspecified felids set to defaults. To be used with new_from_default() and struct update syntax.
    pub fn default(frame: Rc<RefCell<dyn Frame>>, pos: Coord, size: Coord) -> Self {
        Object {
            frame: frame,
            pos: pos,
            size: size,
            offset: Coord {x:0, y:0},
            rot: false,
            yflip: false,
            xflip: false,
            enabled: true,
        }
    }

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
            }
        ))
    }

    /// Create a new Object with the default orientation.
    pub fn new_from_default(default: Object) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(default))
    }

    /// Create a new Object with the default orientation and matching the size of the given frame.
    /// 
    /// Some frames don't have normal sizes like Fill and Text.
    pub fn new_min(frame: Rc<RefCell<dyn Frame>>, pos: Coord) -> Option<Rc<RefCell<Self>>> {
        let size = frame.borrow().size();
        
        if let Some(size) = size {
            Some(Object::new(frame, pos, size, Coord {x:0, y:0}, false, false, false, true))
        }
        else{
            None
        }
    }

    /// Sets if the object is enabled to the provided bool.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Gets the Objects enabled value.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Sets the Objects position to the provided coord.
    pub fn set_pos(&mut self, new: &Coord) {
        self.pos = *new;
    }

    /// Sets the Objects Center position of the object to the provided coord.
    pub fn set_center(&mut self, new: &Coord) {
        self.pos = *new - (self.size / Coord{ x: 2, y: 2 });
    }

    /// Sets the Objects size to the provided coord.
    pub fn set_size(&mut self, new: &Coord) {
        self.size = *new;
    }

    /// Sets the Objects offset to the provided coord.
    pub fn set_offset(&mut self, new: &Coord) {
        self.offset = *new;
    }

    /// Sets whether the Object rotates it frame by 90 degrees.
    pub fn set_rot(&mut self, new: bool) {
        self.rot = new;
    }

    /// Sets whether the Object flips it frame over the Y axis.
    pub fn set_yflip(&mut self, new: bool) {
        self.yflip = new;
    }

    /// Sets whether the Object flips it frame over the X axis.
    pub fn set_xflip(&mut self, new: bool) {
        self.xflip = new;
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

    /// returns a reference to the held frame.
    pub fn get_frame(&self) -> &Rc<RefCell<dyn Frame>> {
        &self.frame
    }

    /// Sets the Objects frame using an object.
    pub fn set_frame_struct<T: Frame + 'static>(&mut self, frame: T) -> &Rc<RefCell<dyn Frame>> {
        self.frame = Rc::new(RefCell::new(frame));
        &self.frame
    }

    /// Sets the Objects frame using a reference.
    pub fn set_frame_rc(&mut self, frame: Rc<RefCell<dyn Frame>>) {
        self.frame = frame;
    }

    /// Increments the offset of the frame by the given amount.
    pub fn inc_offset(&mut self, inc: &Coord) {
        self.offset += *inc;
        self.offset %= self.size * Coord{ x: 2, y: 2 };
    }

    /// Returns the current position of the object.
    pub fn get_pos(&self) -> Coord {
        self.pos
    }

    /// Returns the current size of the object.
    pub fn get_size(&self) -> Coord {
        self.size
    }

    /// Returns the current offset of the frame.
    pub fn get_offset(&self) -> Coord {
        self.offset
    }

    /// Returns the current position of the center of the object.
    pub fn get_center(&self) -> Coord {
        self.pos + (self.size / Coord{ x: 2, y: 2 })
    }

    /// Returns if the frame is currently rotated 90 degrees.
    pub fn get_rot(&self) -> bool {
        self.rot
    }

    /// Returns if the frame is currently flipped over the X axis.
    pub fn get_xflip(&self) -> bool {
        self.xflip
    }

    /// Returns if the frame is currently flipped over the Y axis.
    pub fn get_yflip(&self) -> bool {
        self.yflip
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
    pub fn move_to(&mut self, newpos: &Coord) -> Task {
        let old_rec = self.get_rec();
        self.pos = *newpos;

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
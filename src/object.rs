use crate::shared::*;

pub struct Object {
    frame: Rc<RefCell<dyn Frame>>,
    pos: Coord,
    size: Coord,
    offset: Coord,
    rot: bool,
    yflip: bool,
    xflip: bool,
}

impl Object {
    pub fn new(frame: Rc<RefCell<dyn Frame>>, pos: Coord, size: Coord, offset: Coord, rot: bool, yflip: bool, xflip: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Object {
                frame: frame,
                pos: pos,
                size: size,
                offset: offset,
                rot: rot,
                yflip: yflip,
                xflip: xflip,
            }
        ))
    }

    pub fn new_basic(frame: Rc<RefCell<dyn Frame>>, pos: Coord, size: Coord) -> Rc<RefCell<Self>> {
        Object::new(frame, pos, size, Coord {x:0, y:0}, false, false, false)
    }

    pub fn new_min(frame: Rc<RefCell<dyn Frame>>, pos: Coord) -> Rc<RefCell<Self>> {
        let size = frame.borrow().size();
        Object::new(frame, pos, size, Coord {x:0, y:0}, false, false, false)
    }

    pub fn set_pos(&mut self, new: &Coord) {
        self.pos = *new;
    }

    pub fn set_center(&mut self, new: &Coord) {
        self.pos = *new - (self.size / Coord{ x: 2, y: 2 });
    }

    pub fn set_size(&mut self, new: &Coord) {
        self.size = *new;
    }

    pub fn set_offset(&mut self, new: &Coord) {
        self.offset = *new;
    }

    pub fn set_rot(&mut self, new: bool) {
        self.rot = new;
    }

    pub fn set_yflip(&mut self, new: bool) {
        self.yflip = new;
    }

    pub fn set_xflip(&mut self, new: bool) {
        self.xflip = new;
    }

    pub fn flipx(&mut self) {
        self.xflip = !self.xflip;
    }

    pub fn flipy(&mut self) {
        self.yflip = !self.yflip;
    }

    pub fn rot_cw(&mut self) {
        if self.rot { self.xflip = !self.xflip; }
        else { self.yflip = !self.yflip }
        self.rot = !self.rot;
    }

    pub fn rot_ccw(&mut self) {
        if !self.rot { self.xflip = !self.xflip; }
        else { self.yflip = !self.yflip }
        self.rot = !self.rot;
    }

    pub fn rot_180(&mut self) {
        self.yflip = !self.yflip;
        if !self.rot {
            self.xflip = !self.xflip;
        }
    }

    pub fn frame(&self) -> &Rc<RefCell<dyn Frame>> {
        &self.frame
    }

    pub fn set_frame_struct<T: Frame + 'static>(&mut self, frame: T) -> &Rc<RefCell<dyn Frame>> {
        self.frame = Rc::new(RefCell::new(frame));
        &self.frame
    }

    pub fn set_frame_rc(&mut self, frame: Rc<RefCell<dyn Frame>>) {
        self.frame = frame;
    }

    pub fn inc_offset(&mut self, inc: &Coord) {
        self.offset += *inc;
        self.offset %= self.size * Coord{ x: 2, y: 2 };
    }

    pub fn get_pos(&self) -> Coord {
        self.pos
    }

    pub fn get_size(&self) -> Coord {
        self.size
    }

    pub fn get_offset(&self) -> Coord {
        self.offset
    }

    pub fn get_center(&self) -> Coord {
        self.pos + (self.size / Coord{ x: 2, y: 2 })
    }

    pub fn get_rot(&self) -> bool {
        self.rot
    }

    pub fn get_xflip(&self) -> bool {
        self.xflip
    }

    pub fn get_yflip(&self) -> bool {
        self.yflip
    }

    fn translate(&self, area: &mut Vec<Drawsegment>) -> Option<Vec<Drawsegment>> {
        //ofsets and flips the local drawsegs depending on the objects orentaion before being fed into the local frame.
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

    fn translate_back(&self, drawdata: &mut Vec<DrawData>, old_segs: Option<Vec<Drawsegment>>) {
        //reverts the transated positions beck to their original position so they match with the full area.

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

    pub fn get_draw_data(&self, mut drawsegs: &mut Vec<DrawData>) {
        // takes in the area being drawn to and copies the relivant local data from the held fram.
        
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

    fn local_area(&self, area: &Vec<DrawData>) -> Vec<Drawsegment> {
        // Makes drawsegments that overlap with the total area and the objects local area.

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

    pub fn update(&self) -> Task {
        //outputs a update task for the frame manager containting the area of the object.
        Task::Update(self.get_rec())
    }

    pub fn move_to(&mut self, newpos: &Coord) -> Task {
        //outputs a update task for the frame manager containting the areas of the objects old and new position.
        let old_rec = self.get_rec();
        self.pos = *newpos;

        //println!("old: {:?}\n\nnew:{:?}", old_rec, self.get_rec());
        Task::UpdateMult(
            vec![old_rec, self.get_rec()]
        )
    }
    
    pub fn get_rec(&self) -> Rec {
        //makes a rectangle struct of the area the object is in.
        Rec {
            start: self.pos,
            end: self.pos + self.size,
        }
    }
}

fn copy_onto_area(drawsegs: &mut Vec<DrawData>, local_data: &Vec<DrawData>) {
    //copys the draw data gotten from the local frame into the full segment being updated.
    let mut i = 0;
    for seg in local_data {
        while !seg.overlaps(&drawsegs[i]) {
            i += 1;
        }

        let mut j = (seg.start.x - drawsegs[i].start.x) as usize;
        for pixle in &seg.data {
            match pixle {
                Pixle::Opaque(_) => {
                    drawsegs[i].data[j] = *pixle;
                },
                _ => (),
            }
            j += 1;
        }
    }
}
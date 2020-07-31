use crate::shared::*;

#[derive(Copy, Clone, Debug)]
pub struct Drawsegment {
    pub start: Coord,
    pub len: usize,
}

impl Drawsegment {
    pub fn merge_into_list(list: &mut Vec<Drawsegment>, new: &Drawsegment) {
        //adds segments into a list with no overlap ignoring y component.
        let mut inserted = false;
        for i in 0..list.len() {
            if list[i].merge(&new) {
                inserted = true;

                while i < list.len() - 1 {
                    let next = list[i + 1];
                    if list[i].merge(&next) {
                        list.remove(i + 1);
                    }
                    else
                    {
                        break;
                    }
                }
                break;
            }
        }

        if !inserted {
            list.push(*new);
        }
    }

    pub fn merge(&mut self, new: &Drawsegment) -> bool {
        //merges two segments if possible, on the x components.
        if !((new.end_pos() < self.start.x) || (self.end_pos() < new.start.x)) {
            if new.start.x < self.start.x { self.start.x = new.start.x; }
            if new.end_pos() > self.end_pos() { self.set_len_end_pos(new.end_pos()) }

            true
        }
        else { false }
    }

    pub fn end_pos(&self) -> i32 {
        self.start.x + self.len as i32
    }

    pub fn set_len_end_pos(&mut self, pos: i32) {
        self.len = (pos - self.start.x) as usize;
    }

    pub fn contains_pos(&self, pos: i32) -> bool {
        if self.start.x <= pos && pos < self.end_pos() {
            true
        }
        else { false }
    }

    pub fn make_vertical(segs: &Vec<Drawsegment>) -> Vec<Drawsegment> {

        let mut x_pos_list: Vec<Drawsegment> = Vec::new();

        for seg in segs {
            Drawsegment::merge_into_list(&mut x_pos_list, seg)
        }

        let mut new_segs:Vec<Drawsegment> = Vec::new();

        for x_seg in x_pos_list {
            for x in x_seg.start.x..x_seg.end_pos() {
                new_segs.append(&mut Drawsegment::make_vertical_seg(segs, x))
            }
        }

        new_segs
    }

    pub fn make_vertical_seg(segs: &Vec<Drawsegment>, x: i32) -> Vec<Drawsegment> {

        let mut new_segs:Vec<Drawsegment> = Vec::new();
        let mut cur_seg = None;

        for seg in segs {
            if seg.contains_pos(x) {
                match &mut cur_seg {
                None => {
                    cur_seg = Some(Drawsegment {
                        start: Coord{ y: x, x: seg.start.y},
                        len: 1,
                    });
                }
                Some(working_seg) => {
                    if seg.start.y == working_seg.end_pos() {
                        working_seg.len += 1;
                    }
                    else {
                        new_segs.push(*working_seg);
                        working_seg.start = Coord{ y: x, x: seg.start.y};
                        working_seg.len = 1;
                    }
                }
                }
            }
        }

        if let Some(seg) = cur_seg {
            new_segs.push(seg);
        }
        
        new_segs
    }

    pub fn sort(segs: &mut Vec<Drawsegment>, width: i32) {
        if segs.is_empty() { return }

        let first_x = segs[0].start.x;
        let first_y = segs[0].start.y;
        let mut sorted = true;

        for seg in segs.iter() {
            if seg.start.x < first_x {
                sorted = false;
                break;
            }
            else if seg.start.y != first_y {
                if seg.start.y < first_y {
                    sorted = false;
                }
                break;
            }
        }

        if !sorted {
            segs.sort_unstable_by_key(|k| (k.start.y * width) + k.start.x);
        }
    }
}
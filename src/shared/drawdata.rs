use crate::shared::*;

#[derive(Debug)]
pub struct DrawData {
    pub start: Coord,
    pub data: Vec<Pixel>
}

impl DrawData {
    pub fn from_drawsemgnet(segment: &Drawsegment) -> DrawData {
        DrawData {
            start: segment.start,
            data: Vec::with_capacity((segment.len) as usize)
        }
    }

    pub fn make_drawseg(&self) -> Drawsegment {
        Drawsegment {
            start: self.start,
            len: self.data.len(),
        }
    }

    pub fn end_pos(&self) -> i32 {
        self.start.x + self.data.len() as i32
    }

    pub fn contains_pos(&self, pos: i32) -> bool {
        if self.start.x <= pos && pos < self.end_pos() {
            true
        }
        else { false }
    }

    pub fn overlaps(&self, other: &DrawData) -> bool {
        if  self.start.y == other.start.y &&
            !(other.end_pos() <= self.start.x) &&
            !(self.end_pos() <= other.start.x) {

            true
        }
        else {
            false
        }
    }

    pub fn make_vertical(segs: &Vec<DrawData>, old_segs: &Vec<Drawsegment>) -> Vec<DrawData> {

        let mut new_segs:Vec<DrawData> = Vec::new();
        let mut last_y: Option<i32> = None;

        for seg in old_segs {
            if let Some(y) = last_y {
                if seg.start.y == y {
                    continue;
                }
            }
            last_y = Some(seg.start.y);

            new_segs.append(&mut DrawData::make_vertical_seg(segs, seg.start.y))
        }

        new_segs
    }

    pub fn make_vertical_seg(segs: &Vec<DrawData>, x: i32) -> Vec<DrawData> {

        let mut new_segs:Vec<DrawData> = Vec::new();
        let mut cur_start = None;
        let mut cur_data = Vec::new();

        for seg in segs {
            if seg.contains_pos(x) {
                match &mut cur_start {
                None => {
                    cur_start = Some(Coord{ y: x, x: seg.start.y});
                    cur_data.push(seg.data[x as usize]);
                }
                Some(start) => {
                    if seg.start.y == (start.x + cur_data.len() as i32) {
                        cur_data.push(seg.data[x as usize]);
                    }
                    else {
                        new_segs.push(
                            DrawData {
                                start: *start,
                                data: cur_data,
                            }
                        );
                        cur_start = Some(Coord{ y: x, x: seg.start.y});
                        cur_data = vec![seg.data[x as usize]];
                    }
                }
                }
            }
        }

        if let Some(start) = cur_start {
            new_segs.push(
                DrawData {
                    start: start,
                    data: cur_data,
                }
            );
        }
        
        new_segs
    }

    pub fn sort(segs: &mut Vec<DrawData>, width: i32) {
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
use crate::shared::*;

pub struct Fill {
    pixle: Pixle,
}

impl Frame for Fill {
    fn size(&self) -> Coord {
        Coord{
            x: 0,
            y: 0,
        }
    }

    fn get_draw_data(&self, area: &Vec<Drawsegment>, _: Coord, _: Coord) -> Vec<DrawData> {

        let mut data: Vec<DrawData> = Vec::with_capacity(area.len());

        //println!("{:?}\n\n", area);
        for segment in area {
            //println!("{:?}\n\n{}", segment, segment.end - segment.start.x);
            data.push(DrawData {
                start: segment.start,
                data: vec![self.pixle; segment.len as usize]
            });
        }
        data
    }
}

impl Fill {
    pub fn new(pixle: Pixle) -> Rc<RefCell<Fill>> {
        Rc::new(RefCell::new(
            Fill {
                pixle: pixle,
            }
        ))
    }
    
    pub fn new_struct(pixle: &Pixle) -> Fill {
        Fill {
            pixle: *pixle,
        }
    }

    pub fn set_pixle(&mut self, pixle: &Pixle) {
        self.pixle = *pixle;
    }

    pub fn get_pixle(&self) -> Pixle {
        self.pixle
    }
}
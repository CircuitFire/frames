use crate::shared::*;

pub struct Basic {
    size: Coord,
    pixles: Vec<Pixle>,
}

impl Frame for Basic {

    fn size(&self) -> Coord {
        self.size
    }

    fn get_draw_data(&self, area: &Vec<Drawsegment>, offset: Coord, _: Coord) -> Vec<DrawData> {

        let mut datasegments: Vec<DrawData> = Vec::with_capacity(area.len());
        
        for segment in area {
            let mut draw_data = DrawData::from_drawsemgnet(segment);

            //println!("seg:\nstart: {:?}, end: {}", segment.start, segment.end);
            for x in segment.start.x..segment.end_pos() {
                let pos = Coord {y: segment.start.y, x: x};
                //println!("position: {}", self.flat_pos(&((pos + offset) % self.size)));
                draw_data.data.push(
                    self.get_pixle(&(pos + offset))
                )
            }

            datasegments.push(draw_data);
        }

        datasegments
    }
}

impl Basic {
    pub fn new(size: Coord, pixles: Vec<Pixle>) -> Result<Rc<RefCell<Basic>>, &'static str> {
        if (size.x * size.y) != pixles.len() as i32 {
            Err("size != number of pixles.")
        }
        else {
            Ok(
                Rc::new(RefCell::new(
                    Basic {
                        size: size,
                        pixles: pixles,
                    }
                ))
            )
        }
    }

    pub fn replace(&mut self, size: Coord, pixles: Vec<Pixle>) -> Result<(), &'static str> {
        if (size.x * size.y) != pixles.len() as i32 {
            Err("size != number of pixles.")
        }
        else {
            *self = Basic {
                size: size,
                pixles: pixles,
            };
            Ok(())
        }
    }

    pub fn get_pixle(&self, coord: &Coord) -> Pixle {
        let new_coord = *coord % self.size;
        //println!("old: {:?}, new: {:?}", coord, new_coord);
        self.pixles[self.flat_pos(&new_coord)]
    }

    pub fn get_pixles(&self) -> &Vec<Pixle> {
        &self.pixles
    }

    fn flat_pos(&self, coord: &Coord) -> usize {
        ((coord.y * self.size.x) + coord.x) as usize
    } 

    pub fn set_pixle(&mut self, coord: &Coord, pixle: &Pixle) {
        let index = self.flat_pos(coord);

        self.pixles[index] = *pixle;
    }
}
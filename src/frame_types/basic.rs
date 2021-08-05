use crate::shared::*;

/// The most basic frame consisting of a vec of pixels and a size
/// ## Functions
/// - new
/// 
/// ## Methods
/// - replace
/// - get_pixel
/// - get_pixels
/// - set_pixel
pub struct Basic {
    size: Coord,
    pixels: Vec<Pixel>,
}

impl Frame for Basic {

    fn size(&self) -> Option<Coord> {
        Some(self.size)
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
                    self.get_pixel(&(pos + offset))
                )
            }

            datasegments.push(draw_data);
        }

        datasegments
    }
}

impl Basic {
    pub fn new(size: Coord, pixels: Vec<Pixel>) -> Result<Rc<RefCell<Basic>>, &'static str> {
        if (size.x * size.y) != pixels.len() as i32 {
            Err("size != number of pixels.")
        }
        else {
            Ok(
                Rc::new(RefCell::new(
                    Basic {
                        size: size,
                        pixels: pixels,
                    }
                ))
            )
        }
    }

    pub fn replace(&mut self, size: Coord, pixels: Vec<Pixel>) -> Result<(), &'static str> {
        if (size.x * size.y) != pixels.len() as i32 {
            Err("size != number of pixels.")
        }
        else {
            *self = Basic {
                size: size,
                pixels: pixels,
            };
            Ok(())
        }
    }

    pub fn get_pixel(&self, coord: &Coord) -> Pixel {
        let new_coord = *coord % self.size;
        //println!("old: {:?}, new: {:?}", coord, new_coord);
        self.pixels[self.flat_pos(&new_coord)]
    }

    pub fn get_pixels(&self) -> &Vec<Pixel> {
        &self.pixels
    }

    fn flat_pos(&self, coord: &Coord) -> usize {
        ((coord.y * self.size.x) + coord.x) as usize
    } 

    pub fn set_pixel(&mut self, coord: &Coord, pixel: Pixel) {
        let index = self.flat_pos(coord);

        self.pixels[index] = pixel;
    }
}
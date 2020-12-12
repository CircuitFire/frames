use crate::shared::*;

/// fills whatever any area with the pixel it has
/// ## Functions
/// - new
/// - new_struct
/// 
/// ## Methods
/// - set_pixel
/// - get_pixel
pub struct Fill {
    pixel: Pixel,
}

impl Frame for Fill {
    fn size(&self) -> Option<Coord> {
        None
    }

    fn get_draw_data(&self, area: &Vec<Drawsegment>, _: Coord, _: Coord) -> Vec<DrawData> {

        let mut data: Vec<DrawData> = Vec::with_capacity(area.len());

        //println!("{:?}\n\n", area);
        for segment in area {
            //println!("{:?}\n\n{}", segment, segment.end - segment.start.x);
            data.push(DrawData {
                start: segment.start,
                data: vec![self.pixel; segment.len as usize]
            });
        }
        data
    }
}

impl Fill {
    pub fn new(pixel: Pixel) -> Rc<RefCell<Fill>> {
        Rc::new(RefCell::new(
            Fill {
                pixel: pixel,
            }
        ))
    }
    
    pub fn new_struct(pixel: &Pixel) -> Fill {
        Fill {
            pixel: *pixel,
        }
    }

    pub fn set_pixel(&mut self, pixle: &Pixel) {
        self.pixel = *pixle;
    }

    pub fn get_pixel(&self) -> Pixel {
        self.pixel
    }
}
use crate::shared::*;

pub trait MaskRule {
    fn show(&self, data: &mut Vec<DrawData>, fill: &Pixle, size: Coord);
}

mod circle;
pub use circle::Circle;
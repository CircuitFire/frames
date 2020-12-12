//! The default rules to be used with the Mask frame.
//! 
//! Rules are split up into two parts.
//! 
//! MaskRules which is an object that holds static data, and returns a MaskLogic struct.
//! 
//! MaskLogic holds dynamic data, and returns a bool indicating if the frame is covered by the mask or not.
//! 
//! - Circle

use crate::shared::*;

pub trait MaskRule {
    fn init(&self, size: Coord) -> Box<dyn MaskLogic>;
}

pub trait MaskLogic {
    fn mask(&self, pos: Coord) -> bool;
}

mod circle;
pub use circle::Circle;
use crate::prelude::*;
use super::{SizeUpdate, PosData};

pub struct NoUpdate {}

impl SizeUpdate for NoUpdate {
    fn size_update(&mut self, pos: &mut PosData, new_size: Coord){}
}

pub struct MatchSize {}

impl SizeUpdate for MatchSize {
    fn size_update(&mut self, pos: &mut PosData, new_size: Coord){
        pos.size = new_size;
    }
}
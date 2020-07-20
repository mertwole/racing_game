use image::*;
use std::rc::Rc;

use super::billboard_lods::*;

pub struct Billboard{
    pub road_distance : f32,
    pub offset : f32,

    lods : Rc<BillboardLods>
}

impl Billboard{
    pub fn new(lods : Rc<BillboardLods>, road_distance : f32, offset : f32) -> Billboard {
        Billboard { lods, road_distance, offset } 
    }

    pub fn render(&self, pos_x : i32, pos_y : i32, scale : f32, buffer : &mut RgbImage) {
        self.lods.render(pos_x, pos_y, scale, buffer);
    }
}
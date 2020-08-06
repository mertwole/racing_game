use image::*;
use std::rc::Rc;

mod billboard_lods;
use billboard_lods::*;

#[derive(Clone)]
pub struct Billboard{
    pub road_distance : f32,
    pub offset : f32,

    lods : Rc<BillboardLods>
}

pub struct BillboardFactory {
    lods : Rc<BillboardLods>
}

impl BillboardFactory {
    pub fn new(spritesheet : &RgbaImage, meta_file_content : &[u8]) -> BillboardFactory {
        BillboardFactory { lods : Rc::from(BillboardLods::new(spritesheet, meta_file_content)) }
    }

    pub fn construct(&self, road_distance : f32, offset : f32) -> Billboard {
        Billboard { lods : self.lods.clone(), road_distance, offset }
    }
}  

impl Billboard{
    pub fn render(&self, pos_x : i32, pos_y : i32, scale : f32, buffer : &mut RgbImage) {
        self.lods.render(pos_x, pos_y, scale, buffer);
    }
}
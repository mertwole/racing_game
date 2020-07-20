use std::rc::Rc;
use std::fs::File;

use crate::image::{ RgbImage };

use super::road::RoadYData;
use super::camera::Camera;

mod billboard;
mod billboard_lods;

use billboard::*;
use billboard_lods::*;

pub struct Billboards {
    billboards : Vec<Billboard>
}

impl Billboards {
    pub fn new() -> Billboards {
        let mut billboards : Vec<Billboard> = Vec::new();

        let spritesheet = image::open("resources/test_spritesheet.png").unwrap().to_rgba();
        let meta_file = File::open("resources/test_spritesheet.meta").unwrap();
        let billboard_lods = Rc::from(BillboardLods::new(&spritesheet, &meta_file));

        billboards.push(Billboard::new(billboard_lods.clone(), 10.0, 0.5));
        billboards.push(Billboard::new(billboard_lods.clone(), 11.0, -0.5));
        billboards.push(Billboard::new(billboard_lods.clone(), 12.0, 0.5));
        billboards.push(Billboard::new(billboard_lods.clone(), 13.0, -0.5));
        billboards.push(Billboard::new(billboard_lods.clone(), 14.0, 0.5));
        billboards.push(Billboard::new(billboard_lods.clone(), 15.0, -0.5));
        billboards.push(Billboard::new(billboard_lods.clone(), 16.0, 0.5));

        Billboards { billboards }
    }

    pub fn render(&self, camera : &Camera, y_data : &Vec<RoadYData>, buffer : &mut RgbImage) {
        let mut curr_billboard_render_id = 0;

        for billboard in &self.billboards {
            if billboard.road_distance < camera.road_distance { 
                curr_billboard_render_id += 1; 
            } else { 
                break; 
            }
        }

        'outer : for y in 0..y_data.len() {
            if curr_billboard_render_id >= self.billboards.len() { break; }

            loop {
                if y_data[y].distance + camera.road_distance > self.billboards[curr_billboard_render_id].road_distance {
                    let billboard_scale = camera.screen_dist / y_data[y].distance;
                    let billboard_offset = 0.5 + self.billboards[curr_billboard_render_id].offset * billboard_scale + y_data[y].norm_road_offset;

                    self.billboards[curr_billboard_render_id].render((billboard_offset * (buffer.width() as f32)) as i32, y as i32, billboard_scale, buffer);
                    
                    curr_billboard_render_id += 1;

                    if curr_billboard_render_id >= self.billboards.len() { break 'outer; }
                    if self.billboards[curr_billboard_render_id].road_distance > camera.road_distance + camera.far_plane { break 'outer; }
                } else { break; }
            }
        }
    }
}


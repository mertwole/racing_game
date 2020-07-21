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

    pub fn render(&self, camera : &Camera, y_data : &Vec<RoadYData>, buffer : &mut RgbImage, road_lendth : f32) {
        let mut global_distance;
        let mut prev_global_distance = camera.far_plane;

        let mut curr_render_billboard : i32 = self.billboards.len() as i32 - 1;
        let far_plane_global = camera.road_distance % road_lendth + camera.far_plane;

        for billboard in self.billboards.iter().rev() {
            if billboard.road_distance <= far_plane_global { break; }

            curr_render_billboard -= 1;

            if curr_render_billboard == -1 {
                if self.billboards.last().unwrap().road_distance > camera.road_distance % road_lendth {
                    curr_render_billboard = self.billboards.len() as i32 - 1;
                    break;
                } else { return; }
            }
        }

        let mut curr_render_billboard = curr_render_billboard as usize;

        for y in (0..y_data.len()).rev() {
            global_distance = (y_data[y].distance + camera.road_distance) % road_lendth;

            let first_rendered_billboard = curr_render_billboard;

            loop {
                let billboard_distance = self.billboards[curr_render_billboard].road_distance;
                if billboard_distance < global_distance || billboard_distance > prev_global_distance {
                    break;
                }

                let billboard_scale = camera.screen_dist / y_data[y].distance;
                let billboard_offset = 0.5 + self.billboards[curr_render_billboard].offset * billboard_scale + y_data[y].norm_road_offset;

                self.billboards[curr_render_billboard].render((billboard_offset * (buffer.width() as f32)) as i32, y as i32, billboard_scale, buffer);
                
                if curr_render_billboard == 0 {
                    curr_render_billboard = self.billboards.len();
                }

                curr_render_billboard -= 1;

                // To avoid situation when all the billboards are inside the camera frustrum.
                if curr_render_billboard == first_rendered_billboard { break; }
            }

            prev_global_distance = global_distance;
        }
    }
}


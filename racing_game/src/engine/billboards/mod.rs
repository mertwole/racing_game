use crate::image::{ RgbImage };

use super::road::RoadYData;
use super::camera::Camera;

mod billboard;
pub use billboard::*;

#[derive(Copy, Clone)]
pub struct BillboardId(pub u32);

pub struct Billboards {
    static_billboards : Vec<Billboard>,
    dynamic_billboards : Vec<(BillboardId, Billboard)>
}

impl Billboards {
    pub fn new() -> Billboards {
        Billboards { static_billboards : Vec::new(), dynamic_billboards : Vec::new() }
    }

    pub fn add_static(&mut self, billboard : Billboard) {
        for i in 0..self.static_billboards.len() {
            if self.static_billboards[i].road_distance > billboard.road_distance {
                self.static_billboards.insert(i, billboard);
                return;
            }
        }

        self.static_billboards.push(billboard);
    }   

    pub fn add_dynamic(&mut self, billboard : Billboard) -> BillboardId {
        let id = BillboardId(self.dynamic_billboards.len() as u32);

        for i in 0..self.dynamic_billboards.len() {
            if self.dynamic_billboards[i].1.road_distance > billboard.road_distance {
                self.dynamic_billboards.insert(i, (id, billboard));
                return id;
            }
        }

        self.dynamic_billboards.push((id, billboard));
        id
    }

    pub fn get_dynamic_mut(&mut self, id : BillboardId) -> &mut Billboard {
        for dyn_billboard in &mut self.dynamic_billboards {
            if (dyn_billboard.0).0 == id.0 { return &mut dyn_billboard.1; }
        }
        
        panic!();
    }

    fn get_sorted_billboards(&self) -> Vec<&Billboard> {
        let mut billboards : Vec<&Billboard> = Vec::with_capacity(self.static_billboards.len() + self.dynamic_billboards.len());

        for i in &self.static_billboards { billboards.push(i); }
        for i in &self.dynamic_billboards { billboards.push(&i.1); }

        billboards.sort_by(|a, b| a.road_distance.partial_cmp(&b.road_distance).unwrap());

        return billboards;
    }

    pub fn render_all(&self, camera : &Camera, y_data : &Vec<RoadYData>, buffer : &mut RgbImage, road_lendth : f32) {
        let billboards = self.get_sorted_billboards();

        let mut global_distance;
        let mut prev_global_distance = camera.far_plane;

        let mut curr_render_billboard : i32 = billboards.len() as i32 - 1;
        let far_plane_global = camera.road_distance % road_lendth + camera.far_plane;

        // Find farthest visible billboard.
        for billboard in billboards.iter().rev() {
            if billboard.road_distance <= far_plane_global { break; }

            curr_render_billboard -= 1;

            if curr_render_billboard == -1 {
                if billboards.last().unwrap().road_distance > camera.road_distance % road_lendth {
                    curr_render_billboard = billboards.len() as i32 - 1;
                    break;
                } else { return; }
            }
        }

        let mut curr_render_billboard = curr_render_billboard as usize;

        // Render from back to front.
        for y in (0..y_data.len()).rev() {
            global_distance = (y_data[y].distance + camera.road_distance) % road_lendth;

            let first_rendered_billboard = curr_render_billboard;

            loop {
                let billboard_distance = billboards[curr_render_billboard].road_distance;
                if billboard_distance < global_distance || billboard_distance > prev_global_distance {
                    break;
                }

                let billboard_scale = camera.screen_dist / y_data[y].distance;
                let billboard_offset = 0.5 + billboards[curr_render_billboard].offset * billboard_scale + y_data[y].norm_road_offset;

                billboards[curr_render_billboard].render((billboard_offset * (buffer.width() as f32)) as i32, y as i32, billboard_scale, buffer);
                
                if curr_render_billboard == 0 {
                    curr_render_billboard = billboards.len();
                }

                curr_render_billboard -= 1;

                // To avoid situation when all the billboards are inside the camera frustrum.
                if curr_render_billboard == first_rendered_billboard { break; }
            }

            prev_global_distance = global_distance;
        }
    }
}


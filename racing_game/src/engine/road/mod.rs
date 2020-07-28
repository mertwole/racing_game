use crate::image::{RgbImage, Rgb};
use super::camera::Camera;
use super::common::Math;

mod road_data;
use road_data::*;

#[derive(Clone, Copy)]
pub struct RoadYData {
    pub distance : f32,
    pub norm_road_offset : f32,
    pub norm_road_width : f32
}

pub struct Road {
    data : RoadData,
    width : f32,
    lines_density : f32,
    pub y_data : Vec<RoadYData>,

    ground_color_main : Rgb<u8>,
    ground_color_secondary : Rgb<u8>,

    texture : RgbImage
}

impl Road {
    pub fn new(texture : RgbImage) -> Road {
        Road { 
            data : RoadData::new(), 
            width : 1.0, 
            lines_density : 0.5, 
            texture, 
            ground_color_main : Rgb([0, 100, 0]), 
            ground_color_secondary : Rgb([0, 120, 0]),
            y_data : Vec::new()
        }
    }

    pub fn compute_y_data(&mut self, camera : &Camera, frame_height : u32) {
        let mut prev_y_vis_road_dist = 0.0;
        let mut prev_segment_offset = 0.0;
        let mut norm_road_offset = 0.0;

        let mut hill_width_multiplier = 1.0;
        self.y_data.clear();

        let mut pitch = camera.pitch;
        pitch += self.data.get_camera_pitch_delta(camera.road_distance);

        for y in 0..frame_height {
            // Visible road segment distance.
            let y_norm = (y as f32) / (frame_height as f32);
            let screen_point_ground_height = camera.y_pos - (1.0 - y_norm * pitch) * camera.viewport_height; 
            let mut vis_road_dist = screen_point_ground_height * camera.screen_dist / (camera.y_pos - screen_point_ground_height);
            vis_road_dist += camera.screen_dist;

            // Occlusion culling.
            if vis_road_dist > camera.far_plane || vis_road_dist < 0.0 { continue; }

            // Horizontal offset.
            if prev_y_vis_road_dist != 0.0 { 
                let segment_length = vis_road_dist - prev_y_vis_road_dist;

                let norm_segment_offset = self.data.get_norm_segment_offset(prev_segment_offset, vis_road_dist + camera.road_distance);
                norm_road_offset += norm_segment_offset * segment_length;
                prev_segment_offset = norm_segment_offset;
            }
 
            prev_y_vis_road_dist = vis_road_dist;

            // Hills.
            hill_width_multiplier += self.data.get_hill_width_multiplier_delta(vis_road_dist + camera.road_distance);

            // Road width.
            let norm_road_width = hill_width_multiplier * camera.screen_dist / vis_road_dist;

            self.y_data.push(RoadYData { distance : vis_road_dist, norm_road_offset, norm_road_width });
        }
    }

    pub fn render_from_y_data(&self, image : &mut RgbImage, camera : &Camera) {
        let mut prev_y_vis_road_dist = 0.0;

        let mut road_lines_accum = camera.road_distance % (2.0 * self.lines_density);
        let mut is_horz_line = false;

        for y in 0..self.y_data.len() as u32 {
            let y_data = self.y_data[y as usize];

            // Horz lines. 
            if prev_y_vis_road_dist != 0.0 { 
                let segment_length = y_data.distance - prev_y_vis_road_dist;
                road_lines_accum += segment_length; 
            }
            if road_lines_accum > self.lines_density { 
                is_horz_line = !is_horz_line; 
                road_lines_accum = road_lines_accum % self.lines_density;            
            }
            
            prev_y_vis_road_dist = y_data.distance;

            // Road borders.
            let road_width = self.width * y_data.norm_road_width;
            let norm_left_border = (1.0 - road_width) * 0.5 + y_data.norm_road_offset;
            let norm_right_border = norm_left_border + road_width;

            let left_border_px = (norm_left_border * (image.width() as f32)) as i32;
            let right_border_px = (norm_right_border * (image.width() as f32)) as i32;      
            let road_width_px = (right_border_px - left_border_px + 1) as u32;

            let ground_color = if is_horz_line { self.ground_color_main } else { self.ground_color_secondary };

            // Left ground.
            for x in 0..Math::min(left_border_px, image.width() as i32 - 1) {
                image.put_pixel(x as u32, y, ground_color);
            } 

            // Road.
            // Render main texture if there is horz line, secondary texture elsewhere.
            if road_width_px < self.texture.width() - 1 {              
                let mut road_tex_sample_x = if is_horz_line { 0 } else { self.texture.width() - road_width_px };
                let mut road_tex_sample_y = if is_horz_line { self.texture.height() - road_width_px } else { road_width_px - 1 };
                road_tex_sample_y = self.texture.height() - road_tex_sample_y - 1;

                for x in left_border_px..right_border_px + 1{
                    let tex_pixel = self.texture.get_pixel(road_tex_sample_x, road_tex_sample_y);
                    road_tex_sample_x += 1;
                    if x < 0 || x >= image.width() as i32{ continue; }
                    image.put_pixel(x as u32, y, *tex_pixel);
                }
            }

            // Right ground
            for x in Math::max(right_border_px + 1, 0)..(image.width() as i32) {
                image.put_pixel(x as u32, y, ground_color);
            } 
        }
    }
}
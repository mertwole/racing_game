use crate::image::{RgbImage, Rgb};
use super::camera::Camera;

mod road;
pub use road::*;

mod track_data;
pub use track_data::*;

#[derive(Clone, Copy)]
pub struct YData {
    pub distance : f32,
    pub norm_road_offset : f32,
    pub road_scale : f32,
    pub is_visible : bool,
    pub is_horz_line : bool
}

pub struct Track {
    data : TrackData,
    lines_density : f32,
    pub y_data : Vec<YData>,

    ground_color_main : Rgb<u8>,
    ground_color_secondary : Rgb<u8>
}

impl Track {
    pub fn new(data : TrackData) -> Track {
        Track { 
            data,
            lines_density : 0.5,
            ground_color_main : Rgb([0, 100, 0]), 
            ground_color_secondary : Rgb([0, 120, 0]),
            y_data : Vec::new()
        }
    }

    pub fn compute_y_data(&mut self, camera : &Camera, frame_height : u32) {
        let mut hill_width_multiplier = 1.0;
        self.y_data.clear();

        let mut pitch = camera.pitch;
        pitch += self.data.get_camera_pitch_delta(camera.road_distance);

        let mut offset_delta = 0.0;
        let mut prev_norm_offset = 0.0;
        let mut global_offset = 0.0;
        let mut prev_norm_offset_segment_id = -1;

        let mut prev_y_vis_road_dist = 0.0;
        let mut road_lines_accum = camera.road_distance % (2.0 * self.lines_density);
        let mut is_horz_line = false;

        for y in 0..frame_height {
            // Visible road segment distance.
            let y_norm = (y as f32) / (frame_height as f32);
            let screen_point_ground_height = camera.y_pos - (1.0 - y_norm * pitch) * camera.viewport_height;
            let mut vis_road_dist = screen_point_ground_height * camera.screen_dist / (camera.y_pos - screen_point_ground_height);
            vis_road_dist += camera.screen_dist;

            // Horz lines. 
            if prev_y_vis_road_dist != 0.0 { 
                let segment_length = vis_road_dist - prev_y_vis_road_dist;
                road_lines_accum += segment_length; 
            }
            if road_lines_accum > self.lines_density { 
                is_horz_line = !is_horz_line; 
                road_lines_accum = road_lines_accum % self.lines_density;            
            }
            
            prev_y_vis_road_dist = vis_road_dist;

            // Occlusion culling.
            if vis_road_dist > camera.far_plane || vis_road_dist < 0.0 { continue; }
            if !self.data.is_visible(vis_road_dist + camera.road_distance) { 
                self.y_data.push(YData { distance : vis_road_dist, norm_road_offset : 0.0, road_scale : 0.0, is_visible : false, is_horz_line });
                continue; 
            }

            // Horizontal offset.
            let segment_offset = self.data.get_segment_offset(camera.road_distance + camera.screen_dist, vis_road_dist + camera.road_distance);
            let mut norm_road_offset = match segment_offset {
                OffsetMode::Normal(offset, segment_id) => { 
                    if prev_norm_offset_segment_id != segment_id as isize {
                        prev_norm_offset_segment_id = segment_id as isize;
                        global_offset = prev_norm_offset;
                    }
                    
                    let norm_offset = global_offset + offset * camera.screen_dist / vis_road_dist;
                    offset_delta = norm_offset - prev_norm_offset; 
                    norm_offset
                }
                OffsetMode::AsIs => { 
                    global_offset = prev_norm_offset + offset_delta;
                    prev_norm_offset + offset_delta
                }
            };
            prev_norm_offset = norm_road_offset;

            norm_road_offset -= camera.x_offset * camera.screen_dist / vis_road_dist;

            // Hills.
            hill_width_multiplier += self.data.get_hill_width_multiplier_delta(vis_road_dist + camera.road_distance);

            // Road width.
            let road_scale = hill_width_multiplier * camera.screen_dist / vis_road_dist;

            self.y_data.push(YData { distance : vis_road_dist, norm_road_offset : norm_road_offset, road_scale, is_visible : true, is_horz_line });
        }
    }

    pub fn get_horz_speed(&self, camera : &Camera) -> f32 {
        let offset = self.data.get_segment_offset(camera.road_distance, camera.road_distance + camera.screen_dist);
        match offset {
            OffsetMode::Normal(offset, _) => { offset }
            OffsetMode::AsIs => { 0.0 }
        }
    }

    pub fn get_bounds(&self) -> (f32, f32) {
        let offset = self.y_data[0].norm_road_offset;
        let half_width = self.y_data[0].road_scale * 0.5;

        (offset - half_width, offset + half_width)
    }

    pub fn render_from_y_data(&self, image : &mut RgbImage, camera : &Camera) {
        // Render ground.
        for y in 0..self.y_data.len() {
            let ground_color = if self.y_data[y].is_horz_line { 
                self.ground_color_main
            } else { 
                self.ground_color_secondary 
            };
            for x in 0..image.width() { image.put_pixel(x, y as u32, ground_color); }
        }

        for road in &self.data.roads {
            road.render_from_y_data(image, &self.y_data, camera);
        }
    }
}
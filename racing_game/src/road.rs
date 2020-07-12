use crate::image::{RgbImage, Rgb};
use crate::Camera;

struct RoadData {

}

impl RoadData {
    fn new() -> RoadData {
        RoadData { }
    }

    fn get_segment_offset(&self, start_road_dist : f32, end_road_dist : f32) -> f32 {
        0.01 * (end_road_dist - start_road_dist)
    }
}

pub struct Road {
    data : RoadData,
    width : f32,
    lines_density : f32
}

impl Road {
    pub fn new() -> Road {
        Road { data : RoadData::new(), width : 1.5, lines_density : 4.0 }
    }

    pub fn render(&self, image : &mut RgbImage, camera : &Camera) {
        let mut prev_y_vis_road_dist = 0.0;
        let mut road_lines_accum = camera.road_distance % 2.0 * self.lines_density;
        let mut is_horz_line = false;

        let mut norm_road_offset = 0.0;

        for y in 0..image.height() {
            // Visible road segment distance.
            let y_norm = camera.pitch * (y as f32) / (image.height() as f32);
            let screen_point_ground_height = camera.y_pos - (1.0 - y_norm) * camera.viewport_height; 
            let mut vis_road_dist = screen_point_ground_height * camera.screen_dist / (camera.y_pos - screen_point_ground_height);
            vis_road_dist += camera.screen_dist;

            // Occlusion culling.
            if vis_road_dist > camera.far_plane || vis_road_dist < 0.0 { continue; }

            // Horz lines.
            if prev_y_vis_road_dist != 0.0 { 
                road_lines_accum += vis_road_dist - prev_y_vis_road_dist; 
                norm_road_offset += self.data.get_segment_offset(prev_y_vis_road_dist, vis_road_dist);
            }
            if road_lines_accum > self.lines_density { 
                is_horz_line = !is_horz_line; 
                road_lines_accum = road_lines_accum % self.lines_density;            
            }
            prev_y_vis_road_dist = vis_road_dist;

            // Road borders.
            let norm_road_width = self.width * (camera.screen_dist / vis_road_dist);
            let norm_left_border = (1.0 - norm_road_width) * 0.5 + norm_road_offset;
            let norm_right_border = norm_left_border + norm_road_width;
            let left_border = (norm_left_border * (image.width() as f32)) as i32;
            let right_border = (norm_right_border * (image.width() as f32)) as i32;

            for x in 0..image.width() {   
                let is_road = (x as i32) > left_border && (x as i32) < right_border;
                
                image.put_pixel(x, y, Rgb([if is_road { 255 } else { 0 } , 0, if is_horz_line { 0 } else { 255 }]));
            }
        }
    }
}
use crate::image::{RgbImage, Rgb};
use crate::Camera;
use crate::math::*;

struct CurvatureSegment {
    start : f32,
    end : f32,
    curvature : f32
}

struct Heel{
    start : f32,
    end : f32,
    start_steepness : f32,
    end_steepness : f32
}

struct RoadData {
    track_length : f32,
    curvatures : Vec<CurvatureSegment>,
    heels : Vec<Heel>
}

impl RoadData {
    fn new() -> RoadData {
        let curvatures = vec![
            CurvatureSegment { start : 10.0, end : 30.0, curvature : 0.00005},
            CurvatureSegment { start : 20.0, end : 40.0, curvature : -0.00001}
        ];

        let heels = vec![
            Heel { start : 10.0, end : 50.0, start_steepness : 0.004, end_steepness : -0.004 }
        ];

        RoadData { track_length : 100.0, curvatures, heels }
    }

    fn get_norm_segment_offset(&self, prev_segment_offset : f32, curr_segment_start : f32) -> f32 {
        let seg_start_norm = curr_segment_start % self.track_length;

        let mut curr_curvature = 0.0;

        for curvature_seg in &self.curvatures {
            if seg_start_norm > curvature_seg.start && seg_start_norm < curvature_seg.end { 
                curr_curvature = curvature_seg.curvature;
                break;
            };
        }

        curr_curvature + prev_segment_offset
    }

    fn get_hill_width_multiplier_delta(&self, vis_road_dist : f32) -> f32 {
        let vis_road_dist_norm = vis_road_dist % self.track_length;

        for heel in &self.heels {
            if vis_road_dist_norm > heel.start && vis_road_dist_norm < heel.end {
                let steepness_t = (heel.end - vis_road_dist_norm) / (heel.end - heel.start);
                return Math::lerp(heel.start_steepness, heel.end_steepness, steepness_t);
            }
        }
        
        0.0
    }

    fn get_camera_pitch_delta(&self, camera_road_dist : f32) -> f32 {

        let camera_road_dist_norm = camera_road_dist % self.track_length;

        let mut pitch_del = 0.0;

        if camera_road_dist_norm > 9.0 && camera_road_dist_norm < 15.0 {
            pitch_del = Math::lerp(0.0, -0.1, (camera_road_dist_norm - 9.0) / 6.0);
        }

        if camera_road_dist_norm > 15.0 && camera_road_dist_norm < 35.0 {
            pitch_del = Math::lerp(-0.1, 0.1, (camera_road_dist_norm - 15.0) / 20.0);
        }

        if camera_road_dist_norm > 35.0 && camera_road_dist_norm < 40.0 {
            pitch_del = Math::lerp(0.1, 0.0, (camera_road_dist_norm - 35.0) / 5.0);
        }

        pitch_del
    }
}

pub struct Road {
    data : RoadData,
    width : f32,
    lines_density : f32
}

impl Road {
    pub fn new() -> Road {
        Road { data : RoadData::new(), width : 3.5, lines_density : 5.0 }
    }

    pub fn render(&self, image : &mut RgbImage, camera : &Camera) {
        let mut prev_y_vis_road_dist = 0.0;
        let mut prev_segment_offset = 0.0;
        let mut norm_road_offset = 0.0;

        let mut road_lines_accum = camera.road_distance % 2.0 * self.lines_density;
        let mut is_horz_line = false;

        let mut hill_width_multiplier = 1.0;

        let mut pitch = camera.pitch;
        pitch += self.data.get_camera_pitch_delta(camera.road_distance);

        for y in 0..image.height() {
            // Visible road segment distance.
            let y_norm = (y as f32) / (image.height() as f32);
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

            // Horz lines. 
            if prev_y_vis_road_dist != 0.0 { 
                let segment_length = vis_road_dist - prev_y_vis_road_dist;
                road_lines_accum += segment_length; 
            }
            if road_lines_accum > self.lines_density { 
                is_horz_line = !is_horz_line; 
                road_lines_accum = road_lines_accum % self.lines_density;            
            }
 
            // Hills.
            hill_width_multiplier += self.data.get_hill_width_multiplier_delta(vis_road_dist + camera.road_distance);

            prev_y_vis_road_dist = vis_road_dist;

            // Road borders.
            let mut norm_road_width = self.width * (camera.screen_dist / vis_road_dist);
            norm_road_width *= hill_width_multiplier; 
            let norm_left_border = (1.0 - norm_road_width) * 0.5 + norm_road_offset;
            let norm_right_border = norm_left_border + norm_road_width;
            let left_border = (norm_left_border * (image.width() as f32)) as i32;
            let right_border = (norm_right_border * (image.width() as f32)) as i32;

            for x in 0..image.width() {   
                let is_road = (x as i32) > left_border && (x as i32) < right_border;
                
                image.put_pixel(x, y, Rgb([if is_road { 255 } else { 100 } , 0, if is_horz_line { 100 } else { 255 }]));
            }
        }
    }
}
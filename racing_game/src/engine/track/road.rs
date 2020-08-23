use std::rc::Rc;

use image::{RgbImage, Rgb};

use crate::engine::camera::Camera;
use crate::engine::common::Math;

use super::*;

#[derive(Clone)]
pub struct KeyPoint{
    distance : f32,
    offset : f32
}

impl KeyPoint {
    pub fn new(distance : f32, offset : f32) -> KeyPoint {
        KeyPoint { distance, offset }
    }
}

#[derive(Clone)]
pub struct Road {
    start : f32,
    end : f32,
    width : f32,
    
    keypoints : Vec<KeyPoint>, // Sorted.
    
    texture : Rc<RgbImage>
}

impl Road {
    pub fn new(width : f32, mut keypoints : Vec<KeyPoint>, texture : Rc<RgbImage>) -> Road {
        keypoints.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        Road { 
            start : keypoints[0].distance, 
            end : keypoints.last().unwrap().distance, 
            width, 
            keypoints, 
            texture 
        }
    }

    pub fn roadside_dist(&self, car_left : f32, car_right : f32, car_road_dist : f32) -> Option<f32> {
        let offset = self.get_segment_offset(car_road_dist);
        if offset.is_none() { return None; }
        let offset = offset.unwrap();
        
        let road_left = offset - self.width * 0.5;
        if road_left > car_left { return Some(car_left - road_left); }

        let road_right = offset + self.width * 0.5;
        if road_right < car_right { return Some(car_right - road_right); }

        None
    }

    fn get_segment_offset(&self, road_distance : f32) -> Option<f32> {
        if road_distance < self.start || road_distance > self.end { return None; }

        for i in 0..self.keypoints.len() {
            if self.keypoints[i].distance > road_distance {
                let prev_keypoint = self.keypoints[i - 1].clone();
                let t = (road_distance - prev_keypoint.distance) / (self.keypoints[i].distance - prev_keypoint.distance);
                return Some(Math::smoothstep(prev_keypoint.offset, self.keypoints[i].offset, t));
            }
        }

        None
    }

    pub fn render_from_y_data(&self, image : &mut RgbImage, y_data : &Vec<YData>, camera : &Camera) {
        for y in 0..y_data.len() as u32 {
            let y_data = y_data[y as usize];

            if !y_data.is_visible { continue; }
            let offset = self.get_segment_offset(y_data.distance + camera.road_distance);
            if offset.is_none() { continue; }
            let mut offset = offset.unwrap();
            offset *= camera.screen_dist / y_data.distance;

            // Road borders.
            let road_width = self.width * y_data.road_scale;
            let norm_left_border = 0.5 - road_width * 0.5 + y_data.norm_road_offset + offset;
            let norm_right_border = norm_left_border + road_width;

            let left_border_px = (norm_left_border * (image.width() as f32)) as i32;
            let right_border_px = (norm_right_border * (image.width() as f32)) as i32;      
            let road_width_px = (right_border_px - left_border_px + 1) as u32;

            // Render main texture if there is horz line, secondary texture elsewhere.
            if road_width_px < self.texture.width() - 1 {              
                let mut road_tex_sample_x = if y_data.is_horz_line { 0 } else { self.texture.width() - road_width_px };
                let mut road_tex_sample_y = if y_data.is_horz_line { self.texture.height() - road_width_px } else { road_width_px - 1 };
                road_tex_sample_y = self.texture.height() - road_tex_sample_y - 1;

                for x in left_border_px..right_border_px + 1{
                    let tex_pixel = self.texture.get_pixel(road_tex_sample_x, road_tex_sample_y);
                    road_tex_sample_x += 1;
                    if x < 0 || x >= image.width() as i32{ continue; }
                    image.put_pixel(x as u32, y, *tex_pixel);
                }
            }
        }
    }
}
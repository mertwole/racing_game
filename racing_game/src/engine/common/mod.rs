use image::{RgbImage, RgbaImage, Rgb};

pub mod math;

use math::*;

pub struct ImageOps { }

impl ImageOps {
    pub fn overlay_rgba(bottom : &mut RgbImage, top : &RgbaImage, position : &IVec2) {
        for x in Math::max(0, -position.x)..Math::min(top.width() as isize, bottom.width() as isize - position.x) {
            for y in Math::max(0, -position.y)..Math::min(top.height() as isize, bottom.height() as isize - position.y){
                let image_pixel = top.get_pixel(x as u32, top.height() - y as u32 - 1);
                if image_pixel[3] == 0 { continue; }
                bottom.put_pixel((position.x + x) as u32, (position.y + y) as u32, Rgb([image_pixel[0], image_pixel[1], image_pixel[2]]));
            }
        }
    }
}
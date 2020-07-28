use image::{RgbaImage, RgbImage, Rgb};

use super::common::Math;

pub struct Horizon {
    image : RgbaImage
}

impl Horizon {
    pub fn new(image : RgbaImage) -> Horizon {
        Horizon { image }
    }

    pub fn render(&self, y_pos : u32, horz_offset : f32, buffer : &mut RgbImage) {
        let sample_start_x = (self.image.width() as f32 * horz_offset) as u32;

        for y in 0..Math::min(buffer.height() - y_pos - 1, self.image.height()) {
            for x in 0..buffer.width() {
                let sample_x = (sample_start_x + x) % self.image.width();

                let pixel = self.image.get_pixel(sample_x, self.image.height() - y - 1);
                if pixel[3] == 0 { continue; }
                buffer.put_pixel(x, y + y_pos, Rgb([pixel[0], pixel[1], pixel[2]]));
            }
        }
    }
}
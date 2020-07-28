use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::math::{IVec2, Math};

use super::{UIControl, Pivot};

pub struct UIImage {
    image : Rc<RgbaImage>,
    position : IVec2
}

impl UIImage {
    pub fn new(image : Rc<RgbaImage>, position : IVec2, pivot : Pivot) -> UIImage {
        let image_size = IVec2::new(image.width() as isize, image.height() as isize);
        let mut position = position;
        match pivot {
            Pivot::Center => { position = &position - &(&image_size / 2) },
            Pivot::LeftBottom => { }
        }

        UIImage { image, position } 
    }
}

impl UIControl for UIImage {
    fn draw(&self, buffer: &mut RgbImage) {
        for x in Math::max(0, -self.position.x)..Math::min(self.image.width() as isize, buffer.width() as isize - self.position.x) {
            for y in Math::max(0, -self.position.y)..Math::min(self.image.height() as isize, buffer.height() as isize - self.position.y){
                let image_pixel = self.image.get_pixel(x as u32, self.image.height() - y as u32 - 1);
                if image_pixel[3] == 0 { continue; }
                buffer.put_pixel((self.position.x + x) as u32, (self.position.y + y) as u32, Rgb([image_pixel[0], image_pixel[1], image_pixel[2]]));
            }
        }
    }
}
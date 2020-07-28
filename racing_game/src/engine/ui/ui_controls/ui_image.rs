use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::math::{IVec2, Math};
use crate::engine::ImageOps;
use super::UIControl;

pub struct UIImage {
    image : Rc<RgbaImage>,
    position : IVec2
}

impl UIImage {
    pub fn new(image : Rc<RgbaImage>) -> UIImage {
        UIImage { image, position : IVec2::zero() } 
    }
}

impl UIControl for UIImage {
    fn draw(&self, buffer: &mut RgbImage) {
        ImageOps::overlay_rgba(buffer, &self.image, &self.position);
    }

    fn get_size(&self) -> IVec2 {
        IVec2::new(self.image.width() as isize, self.image.height() as isize)
    }
    
    fn set_position(&mut self, position: IVec2) {
        self.position = position;
    }
}
use image::*;

use crate::engine::math::*;

mod ui_text;
mod ui_image;

pub use ui_text::*;
pub use ui_image::*;

pub enum Pivot {
    Center,
    LeftBottom
}

pub trait UIControl {
    fn draw(&self, buffer : &mut RgbImage);
}
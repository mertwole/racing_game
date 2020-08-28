use image::*;

use crate::engine::common::IVec2;

mod ui_text;
mod ui_image;
mod ui_selector;
mod ui_void;

pub use ui_text::*;
pub use ui_image::*;
pub use ui_selector::*;
pub use ui_void::*;

pub trait UIControl {
    fn draw(&self, buffer : &mut RgbImage);
    fn set_position(&mut self, position : IVec2);
    fn get_position(&self) -> IVec2;
    fn get_size(&self) -> IVec2;
}
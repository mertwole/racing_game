use image::RgbImage;

use super::math::IVec2;

mod ui_controls;
pub use ui_controls::{UIImage, UIText, Pivot};
use ui_controls::UIControl;

mod font;
pub use font::Font;

pub struct UIPage{
    controls : Vec<Box<dyn UIControl>>,
    resolution : IVec2
}

impl UIPage {
    pub fn new(resolution : IVec2) -> UIPage{
        UIPage { controls : Vec::new(), resolution }
    }

    pub fn add_control(&mut self, mut control : Box<dyn UIControl>, pivot : Pivot, position : IVec2) {
        let cintrol_size = control.get_size();
        let position = match pivot {
            Pivot::Center => { &position - &(&cintrol_size / 2) },
            Pivot::LeftBottom => { position }
        };

        control.set_position(position);

        self.controls.push(control);
    }
    
    pub fn draw(&self, buffer : &mut RgbImage) {
        for control in &self.controls{ control.as_ref().draw(buffer); }
    }
}
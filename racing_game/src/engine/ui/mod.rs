use std::rc::Rc;

use image::RgbImage;

use super::math::IVec2;

pub mod ui_controls;
use ui_controls::*;

mod font;
pub use font::Font;

pub struct UIPage{
    controls : Vec<Box<UIControl>>,
    font : Rc<Font>
}

impl UIPage {
    pub fn new(resolution : IVec2, font : Rc<Font>) -> UIPage{
        UIPage { font, controls : Vec::new() }
    }

    pub fn add_control(&mut self, control : Box<UIControl>) {
        self.controls.push(control);
    }
    
    pub fn draw(&self, buffer : &mut RgbImage) {
        for control in &self.controls{ control.as_ref().draw(buffer); }
    }
}
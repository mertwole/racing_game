use image::{RgbImage, Rgb};

use super::common::{IVec2, IAABB};

mod ui_controls;
pub use ui_controls::{UIImage, UIText, Pivot};
use ui_controls::UIControl;

pub mod font;

pub struct UIPage{
    controls : Vec<Box<dyn UIControl>>,
    resolution : IVec2,
    background_color : Option<Rgb<u8>>
}

impl UIPage {
    pub fn new(resolution : IVec2, background_color : Option<Rgb<u8>>) -> UIPage{
        UIPage { controls : Vec::new(), resolution, background_color }
    }

    pub fn get_control_aabb(&self, control : &dyn UIControl, pivot : Pivot, position : IVec2) -> IAABB {
        let control_size = control.get_size();
        let position = match pivot {
            Pivot::Center => { &position - &(&control_size / 2) },
            Pivot::LeftBottom => { position }
        };

        IAABB::new(position.clone(), &position + &control_size)
    }

    pub fn add_control(&mut self, mut control : Box<dyn UIControl>, pivot : Pivot, position : IVec2) {
        let control_size = control.get_size();
        let position = match pivot {
            Pivot::Center => { &position - &(&control_size / 2) },
            Pivot::LeftBottom => { position }
        };

        control.set_position(position);

        self.controls.push(control);
    }
    
    pub fn clear_controls(&mut self) {
        self.controls = Vec::new();
    }

    pub fn draw(&self, buffer : &mut RgbImage) {
        match self.background_color {
            Some(color) => { 
                for x in 0..buffer.width() {
                    for y in 0..buffer.height() {
                        buffer.put_pixel(x, y, color);
                    }
                } 
            }
            _ => { } 
        }
        for control in &self.controls{ control.as_ref().draw(buffer); }
    }
}
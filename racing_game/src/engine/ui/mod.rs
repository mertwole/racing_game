use image::{RgbImage, Rgb};

use super::common::{IVec2};

mod ui_controls;
pub use ui_controls::{UIImage, UIText};
use ui_controls::UIControl;

pub mod font;

pub mod selector_menu;

pub enum Pivot {
    Center,
    LeftBottom,
    RightTop,
    RightBottom,
    LeftTop
}

pub enum Binding {
    LeftBottom,
    RightBottom,
    LeftTop,
    RightTop,
    Center
}

pub struct ControlProperties {
    pub pivot : Pivot,
    pub binding : Binding,
    pub position : IVec2
}

#[readonly::make]
pub struct UIPage{
    controls : Vec<Box<dyn UIControl>>,
    pub resolution : IVec2,
    background_color : Option<Rgb<u8>>
}

impl UIPage {
    pub fn new(resolution : IVec2, background_color : Option<Rgb<u8>>) -> UIPage{
        UIPage { controls : Vec::new(), resolution, background_color }
    }

    pub fn add_control(&mut self, mut control : Box<dyn UIControl>, properties : &ControlProperties) {
        let control_size = control.get_size();
        let mut position = properties.position;

        position = &position - &match properties.pivot {
            Pivot::Center => { &control_size / 2 },
            Pivot::LeftBottom => { IVec2::zero() }
            Pivot::RightTop => { control_size }
            Pivot::RightBottom => { IVec2::new(control_size.x, 0) }
            Pivot::LeftTop => { IVec2::new(0, control_size.y) }
        };
        
        position = &position + &match properties.binding {
            Binding::Center => { &self.resolution / 2 },
            Binding::LeftBottom => { IVec2::zero() },
            Binding::LeftTop => { IVec2::new(0, self.resolution.y) },
            Binding::RightBottom => { IVec2::new(self.resolution.x, 0) },
            Binding::RightTop => { self.resolution },
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

pub struct ModalPage {
    page : UIPage,
    position : IVec2,
    size : IVec2,
    background_color : Option<Rgb<u8>>
}

impl ModalPage {
    pub fn new(position : IVec2, size : IVec2, background_color : Option<Rgb<u8>>) -> ModalPage{
        let page = UIPage::new(size.clone(), None);
        ModalPage { page, position, size, background_color }
    }

    pub fn add_control(&mut self, mut control : Box<dyn UIControl>, mut properties : ControlProperties) {
        properties.position = &properties.position + &self.position;
        self.page.add_control(control, &properties);
    }

    pub fn draw(&self, buffer : &mut RgbImage) {
        match self.background_color {
            Some(color) => { 
                for x in self.position.x..self.position.x + self.size.x {
                    for y in self.position.y..self.position.y + self.size.y {
                        buffer.put_pixel(x as u32, y as u32, color);
                    }
                } 
            }
            _ => { } 
        }
        self.page.draw(buffer);
    }
}
use image::{RgbImage, Rgb};

use super::common::{IVec2, Vec2};

mod ui_controls;
pub use ui_controls::*;

pub mod font;

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

impl ControlProperties {
    pub fn new(position : IVec2, pivot : Pivot, binding : Binding) -> ControlProperties {
        ControlProperties { position, pivot, binding }
    }

    fn get_position(&self, control_size : IVec2, resolution : IVec2) -> IVec2 {
        let mut position = self.position;

        position = &position - &match self.pivot {
            Pivot::Center => { &control_size / 2 },
            Pivot::LeftBottom => { IVec2::zero() }
            Pivot::RightTop => { control_size }
            Pivot::RightBottom => { IVec2::new(control_size.x, 0) }
            Pivot::LeftTop => { IVec2::new(0, control_size.y) }
        };
        
        position = &position + &match self.binding {
            Binding::Center => { &resolution / 2 },
            Binding::LeftBottom => { IVec2::zero() },
            Binding::LeftTop => { IVec2::new(0, resolution.y) },
            Binding::RightBottom => { IVec2::new(resolution.x, 0) },
            Binding::RightTop => { resolution },
        };

        position
    }
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
        let position = properties.get_position(control.get_size(), self.resolution);
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

#[derive(PartialEq)]
pub enum ModalAnim{
    Unfold(f32),
    Fold(f32),
    Void
}

#[readonly::make]
pub struct ModalPage {
    page : UIPage,
    position : IVec2,
    size : IVec2,
    curr_size : Vec2,
    background_color : Option<Rgb<u8>>,
    render_controls : bool,
    pub anim_state : ModalAnim
}

impl ModalPage {
    pub fn new(position : IVec2, size : IVec2, background_color : Option<Rgb<u8>>) -> ModalPage{
        let page = UIPage::new(size.clone(), None);
        ModalPage { page, position, size, curr_size : Vec2::new(0.0, size.y as f32), background_color, anim_state : ModalAnim::Void, render_controls : false }
    }

    pub fn update(&mut self, delta_time : f32) {
        match self.anim_state {
            ModalAnim::Fold(anim_speed) => {
                self.curr_size.x = self.curr_size.x - anim_speed * delta_time;
                if self.curr_size.x <= 0.0 {
                    self.curr_size.x = 0.0;
                    self.anim_state = ModalAnim::Void;
                }
            }
            ModalAnim::Unfold(anim_speed) => {
                self.curr_size.x = self.curr_size.x + anim_speed * delta_time;
                if self.curr_size.x >= self.size.x as f32 {
                    self.curr_size.x = self.size.x as f32;
                    self.anim_state = ModalAnim::Void;
                    self.render_controls = true;
                }
            }
            ModalAnim::Void => { }
        }
    }   

    pub fn add_control(&mut self, control : Box<dyn UIControl>, mut properties : ControlProperties) {
        properties.position = &properties.position + &self.position;
        self.page.add_control(control, &properties);
    }

    pub fn clear_controls(&mut self) { 
        self.page.clear_controls();
    }

    pub fn get_control_mut(&mut self, id : usize) -> &mut dyn UIControl {
        self.page.controls[id].as_mut()
    }

    pub fn get_control(&mut self, id : usize) -> &dyn UIControl {
        self.page.controls[id].as_ref()
    }

    pub fn start_anim_unfold(&mut self, anim_speed : f32) {
        self.anim_state = ModalAnim::Unfold(anim_speed);
    }

    pub fn start_anim_fold(&mut self, anim_speed : f32) {
        self.anim_state = ModalAnim::Fold(anim_speed);
        self.render_controls = false;
    }   

    pub fn draw(&self, buffer : &mut RgbImage) {
        match self.background_color {
            Some(color) => {
                for x in self.position.x..self.position.x + self.curr_size.x as isize {
                    for y in self.position.y..self.position.y + self.curr_size.y as isize {
                        buffer.put_pixel(x as u32, y as u32, color);
                    }
                }
            }
            _ => { }
        }

        if self.render_controls { self.page.draw(buffer); }
    }
}
use image::{RgbImage};

use crate::engine::common::{IVec2};
use super::UIControl;

pub struct UIVoid {
    position : IVec2
}

impl UIVoid {
    pub fn new() -> UIVoid { UIVoid { position : IVec2::zero() } }
}

impl UIControl for UIVoid {
    fn draw(&self, buffer: &mut RgbImage) { }

    fn get_size(&self) -> IVec2 { IVec2::zero() }
    
    fn set_position(&mut self, position: IVec2) { self.position = position; }

    fn get_position(&self) -> IVec2 { self.position } 
}
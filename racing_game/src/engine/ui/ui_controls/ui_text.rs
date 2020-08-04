use std::rc::Rc;

use image::RgbImage;

use crate::engine::common::{IVec2, ImageOps};
use super::{UIControl, super::font::Font};

pub struct UIText {
    font : Rc<Font>,
    text : String,
    position : IVec2
}

impl UIText {
    pub fn new(font : Rc<Font>, text : String) -> UIText {
        UIText { text, position : IVec2::zero(), font }
    }
}

impl UIControl for UIText {
    fn draw(&self, buffer : &mut RgbImage) {
        let mut draw_offset = self.position.clone();

        for symbol in self.text.chars() {
            let symbol_buffer = self.font.get_symbol(symbol);
            ImageOps::overlay_rgba(buffer, symbol_buffer, &draw_offset);
            draw_offset.x += symbol_buffer.width() as isize;
        }
    }

    fn get_size(&self) -> IVec2 {
        let width : u32 = self.text.chars().map(|c| self.font.get_symbol(c).width()).sum();
        let height : u32 = self.text.chars().map(|c| self.font.get_symbol(c).height()).max().unwrap();
        IVec2::new(width as isize, height as isize)
    }
    
    fn set_position(&mut self, position: IVec2) {
        self.position = position;
    }

    fn get_position(&self) -> IVec2 {
        self.position
    }   
}
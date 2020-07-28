use std::rc::Rc;

use crate::engine::math::{IVec2, Math};

use image::{RgbImage, Rgb};

use super::{UIControl, Pivot, super::Font};

pub struct UIText {
    font : Rc<Font>,
    text : String,
    position : IVec2
}

impl UIText {
    pub fn new(font : Rc<Font>, text : String, position : IVec2, pivot : Pivot) -> UIText {
        let text_width : u32 = text.chars().map(|c| font.get_symbol(c).width()).sum();
        let text_height : u32 = text.chars().map(|c| font.get_symbol(c).height()).max().unwrap();
        let text_size = IVec2::new(text_width as isize, text_height as isize);

        let mut position = position;

        match pivot {
            Pivot::Center => { position = &position - &(&text_size / 2) },
            Pivot::LeftBottom => { }
        }

        UIText { text, position, font }
    }
}

impl UIControl for UIText {
    fn draw(&self, buffer : &mut RgbImage) {
        let mut draw_offset = self.position.clone();

        for symbol in self.text.chars() {
            let symbol_buffer = self.font.get_symbol(symbol);
            for x in Math::max(0, -draw_offset.x)..Math::min(symbol_buffer.width() as isize, buffer.width() as isize - draw_offset.x) {
                for y in Math::max(0, -draw_offset.y)..Math::min(symbol_buffer.height() as isize, buffer.height() as isize - draw_offset.y){
                    let symbol_pixel = symbol_buffer.get_pixel(x as u32, y as u32);
                    if symbol_pixel[3] == 0 { continue; }
                    buffer.put_pixel((draw_offset.x + x) as u32, (draw_offset.y + y) as u32, Rgb([symbol_pixel[0], symbol_pixel[1], symbol_pixel[2]]));
                }
            }
            draw_offset.x += symbol_buffer.width() as isize;
        }
    }
}
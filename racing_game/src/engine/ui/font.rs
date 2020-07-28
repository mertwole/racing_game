use std::collections::HashMap;

use image::RgbaImage;

use crate::engine::math::IVec2;

pub struct Font {
    symbols : HashMap<char, RgbaImage>
}

impl Font {
    pub fn new(texture : RgbaImage, symbol_size : IVec2, symbol_sequence : String) -> Font {
        let symbols_in_row = texture.width() as isize / symbol_size.x;

        let mut symbols = HashMap::<char, RgbaImage>::new();

        let mut symbol_id = 0;
        for symbol in symbol_sequence.chars() {
            let column = symbol_id % symbols_in_row;
            let row = symbol_id / symbols_in_row;
            let read_x = (column * symbol_size.x) as u32;
            let read_y = (row * symbol_size.y) as u32;

            let mut symbol_buffer = RgbaImage::new(symbol_size.x as u32, symbol_size.y as u32);

            for x in 0..symbol_size.x as u32 {
                for y in 0..symbol_size.y as u32 {
                    let pixel = texture.get_pixel(x + read_x, y + read_y);
                    symbol_buffer.put_pixel(x, y, *pixel);
                }
            }
            symbols.insert(symbol, symbol_buffer);

            symbol_id += 1;
        }

        Font { symbols }
    }

    pub fn get_symbol(&self, symbol : char) -> &RgbaImage {
        &self.symbols.get(&symbol).unwrap()
    }
}
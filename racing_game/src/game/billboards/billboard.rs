use image::*;

use std::slice;
use std::mem;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct SpriteDescr {
    pos_x : u32,
    pos_y : u32,

    width : u32,
    height : u32
}

pub struct Billboard{
    lods : Vec<RgbaImage>
}

impl Billboard{
    pub fn new(spritesheet : RgbaImage, meta_file : File) -> Billboard {
        let mut reader = BufReader::new(meta_file);
        let mut file_data : Vec<u8> = Vec::new();
        reader.read_to_end(&mut file_data).unwrap();
        let sprites_data_raw = file_data.as_ptr() as *const SpriteDescr;
        let sprites_data_count = file_data.len() / mem::size_of::<SpriteDescr>();
        let sprites_data = unsafe { slice::from_raw_parts(sprites_data_raw, sprites_data_count) };

        let mut lods : Vec<RgbaImage> = Vec::with_capacity(sprites_data.len());
        for sprite_data in sprites_data {
            let mut lod = RgbaImage::new(sprite_data.width, sprite_data.height);
            
            for x in 0..sprite_data.width {
                for y in 0..sprite_data.height {
                    lod.put_pixel(x, y, *spritesheet.get_pixel(x + sprite_data.pos_x, y + sprite_data.pos_y));
                }
            }

            lods.push(lod);
        }

        Billboard { lods } 
    }

    pub fn render(&self) {

    }
}
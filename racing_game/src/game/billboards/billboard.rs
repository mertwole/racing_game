use image::*;

use crate::game::math::*;

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

struct Lod{
    image : RgbaImage,
    scale : f32
}

pub struct Billboard{
    lods : Vec<Lod>
}

impl Billboard{
    pub fn new(spritesheet : RgbaImage, meta_file : File) -> Billboard {
        let mut reader = BufReader::new(meta_file);
        let mut file_data : Vec<u8> = Vec::new();
        reader.read_to_end(&mut file_data).unwrap();
        let sprites_data_raw = file_data.as_ptr() as *const SpriteDescr;
        let sprites_data_count = file_data.len() / mem::size_of::<SpriteDescr>();
        let sprites_data = unsafe { slice::from_raw_parts(sprites_data_raw, sprites_data_count) };

        let mut lods : Vec<Lod> = Vec::with_capacity(sprites_data.len());
        for sprite_data in sprites_data {
            let mut lod = Lod { image : RgbaImage::new(sprite_data.width, sprite_data.height), scale : 1.0 };
            
            if lods.len() != 0 {
                lod.scale = (lod.image.width() as f32) / (lods[0].image.width() as f32);
            }

            for x in 0..sprite_data.width {
                for y in 0..sprite_data.height {
                    lod.image.put_pixel(x, y, *spritesheet.get_pixel(x + sprite_data.pos_x, y + sprite_data.pos_y));
                }
            }

            lods.push(lod);
        }

        Billboard { lods } 
    }

    fn get_lod_id(&self, scale : f32) -> u32 {
        let mut closest_lod = 0;
        for i in 0..self.lods.len() {
            if self.lods[i].scale > scale { 
                closest_lod += 1; 
            } else { 
                if i == 0 { return 0; }

                let to_curr_lod = self.lods[i - 1].scale - scale;
                let to_next_lod = scale - self.lods[i].scale;

                return if to_next_lod > to_curr_lod { closest_lod - 1 } else { closest_lod }
            }
        }

        return closest_lod;
    }

    pub fn render(&self, pos_x : i32, pos_y : i32, scale : f32, buffer : &mut RgbImage) {
        let lod = &self.lods[self.get_lod_id(scale) as usize];

        let left_bottom_x = pos_x - lod.image.width() as i32 / 2;
        for x in Math::max(0, -left_bottom_x)..Math::min(lod.image.width() as i32, buffer.width() as i32 - left_bottom_x) {
            for y in Math::max(0, -pos_y)..Math::min(lod.image.height() as i32, buffer.height() as i32 - pos_y) {
                let pixel = lod.image.get_pixel(x as u32, (lod.image.height() as i32 - y - 1) as u32);
                if pixel[3] == 0 { continue; }
                buffer.put_pixel((x + left_bottom_x) as u32, (y + pos_y) as u32, Rgb([pixel[0], pixel[1], pixel[2]]))
            }
        }
    }
}
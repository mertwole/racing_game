extern crate image;
use image::*;

use std::env;
use std::slice;
use std::mem;
use std::fs::File;
use std::io::prelude::*;

struct SpriteDescr {
    pos_x : u32,
    pos_y : u32,

    width : u32,
    height : u32
}

fn main() {
    // Skip first because it is executable path.
    let filenames : Vec<String> = env::args().skip(1).collect();

    let mut images : Vec<RgbaImage> = Vec::with_capacity(filenames.len());
    for filename in filenames { images.push(image::open(filename).unwrap().into_rgba()); }

    let spritesheet_height = images.iter().max_by(|x, y| x.height().cmp(&y.height())).unwrap().height();
    let spritesheet_width : u32 = images.iter().map(|x| x.width()).sum();

    let mut spritesheet = RgbaImage::new(spritesheet_width, spritesheet_height);
    let mut sprite_descriptions : Vec<SpriteDescr> = Vec::with_capacity(images.len());
    let mut curr_x = 0;
    for image in images {
        sprite_descriptions.push(SpriteDescr { pos_x : curr_x, pos_y : 0, width : image.width(), height : image.height() });

        for x in 0..image.width() {
            for y in 0..image.height() {
                spritesheet.put_pixel(x + curr_x, y, *image.get_pixel(x, y));
            }
        }

        curr_x += image.width();
    }

    let meta_raw = sprite_descriptions.as_ptr() as *const u8;
    let meta_raw_slice : &[u8] = unsafe { slice::from_raw_parts(meta_raw, sprite_descriptions.len() * mem::size_of::<SpriteDescr>()) };
    let mut meta_file = File::create("output.meta").unwrap();
    meta_file.write_all(meta_raw_slice).unwrap();

    spritesheet.save_with_format("output.png", ImageFormat::Png).unwrap();
}

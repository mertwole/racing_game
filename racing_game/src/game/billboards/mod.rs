use crate::image::{ RgbImage, RgbaImage, Rgb};
use super::road::RoadYData;

mod billboard;

pub use billboard::*;

struct Billboards {

}

impl Billboards {
    pub fn new() -> Billboards {
        Billboards { }
    }

    pub fn render(&self, y_data : &Vec<RoadYData>, buffer : &mut RgbImage) {

    }
}


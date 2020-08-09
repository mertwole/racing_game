use std::rc::Rc;

use image::RgbaImage;

use super::*;

pub struct Shop {
    logo : Rc<RgbaImage>
}

impl Service for Shop {
    fn get_logo(&self) -> Rc<RgbaImage> {
        self.logo.clone()
    }

    fn get_ref_type(&self) -> ServiceType { ServiceType::Shop }
    fn get_type() -> ServiceType { ServiceType::Shop }
}
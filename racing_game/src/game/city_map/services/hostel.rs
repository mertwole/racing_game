use std::rc::Rc;

use image::RgbaImage;

use super::*;

pub struct Hostel {
    logo : Rc<RgbaImage>
}

impl Service for Hostel {
    fn get_logo(&self) -> Rc<RgbaImage> {
        self.logo.clone()
    }

    fn get_ref_type(&self) -> ServiceType { ServiceType::Hostel }
    fn get_type() -> ServiceType { ServiceType::Hostel }
}
use std::rc::Rc;

use image::RgbaImage;

use super::*;

pub struct RepairStation {
    logo : Rc<RgbaImage>
}

impl Service for RepairStation {
    fn get_logo(&self) -> Rc<RgbaImage> {
        self.logo.clone()
    }

    fn get_ref_type(&self) -> ServiceType { ServiceType::RepairStation }
    fn get_type() -> ServiceType { ServiceType::RepairStation }
}
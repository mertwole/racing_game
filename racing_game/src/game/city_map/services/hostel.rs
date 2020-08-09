use std::rc::Rc;

use rand::{Rng, rngs::StdRng};
use image::RgbaImage;

use super::*;
use crate::game::Time;

pub struct RoomRemoveOption {
    pub time : Time,
    pub cost : f32
}

pub struct Hostel {
    logo : Rc<RgbaImage>,
    pub options : Vec<RoomRemoveOption>
}

impl Hostel {
    pub fn generate(logo : RgbaImage, rng : &mut StdRng) -> Hostel {
        let mut options = Vec::new();
        options.push(RoomRemoveOption { time : Time::new(1, 30), cost : 10.0 });
        options.push(RoomRemoveOption { time : Time::new(1, 00), cost : 15.0 });
        options.push(RoomRemoveOption { time : Time::new(2, 50), cost : 20.0 });

        Hostel { logo : Rc::from(logo), options }
    }

    pub fn rest(&mut self, option_id : u32, player : &mut Player) {
        player.money -= self.options[option_id as usize].cost;
    }
}

impl Service for Hostel {
    fn get_logo(&self) -> Rc<RgbaImage> {
        self.logo.clone()
    }

    fn get_ref_type(&self) -> ServiceType { ServiceType::Hostel }
    fn get_type() -> ServiceType { ServiceType::Hostel }
}
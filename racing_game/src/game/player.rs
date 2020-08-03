use crate::image::{RgbaImage, RgbImage};
use crate::engine::common::{IVec2, ImageOps};
use crate::game::{Percent};

pub struct Player {
    pub money : u32,
    pub hunger : Percent,
    pub thirst : Percent,
    pub tireness : Percent,
    
    pub oil_level : Percent,
    pub gas_level : Percent,
    pub car_damage : Percent,
}

impl Player {
    pub fn new() -> Player {
        Player {
            money : 0,
            hunger : Percent(0.0),
            thirst : Percent(0.0),
            tireness : Percent(0.0),
            oil_level : Percent(0.0),
            gas_level : Percent(0.0),
            car_damage : Percent(0.0)
        }
    }
}
use crate::image::{RgbaImage, RgbImage};
use crate::engine::common::{IVec2, ImageOps};
use crate::game::{Percent, Money};

pub struct Player {
    pub money : Money,
    pub hunger : Percent,
    pub thirst : Percent,
    pub tireness : Percent
}

impl Player {
    pub fn new() -> Player {
        Player {
            money : Money(0),
            hunger : Percent(0.0),
            thirst : Percent(0.0),
            tireness : Percent(0.0)
        }
    }
}
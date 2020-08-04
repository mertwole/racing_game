use std::rc::Rc;

use rand::{Rng, rngs::StdRng};
use image::{RgbaImage};

use crate::game::{Percent, player::Player};

pub struct GasStation {
    pub logo : Rc<RgbaImage>,
    pub gas_cost : u32
}   

impl GasStation {
    pub fn generate(logo : RgbaImage, rng : &mut StdRng) -> GasStation {
        let gas_cost = rng.gen_range(5, 15);
        GasStation { logo : Rc::from(logo), gas_cost }
    }

    pub fn buy_gas(&mut self, amount : u32, player : &mut Player) {
        player.money -= 1;
        player.gas_level = Percent(100.0);
    }
}
use std::rc::Rc;

use rand::{Rng, rngs::StdRng};
use image::{RgbaImage};

use crate::game::{Percent, player::Player};

pub struct GasStation {
    pub logo : Rc<RgbaImage>,
    pub gas_cost : f32
}   

impl GasStation {
    pub fn generate(logo : RgbaImage, rng : &mut StdRng) -> GasStation {
        let gas_cost = rng.gen_range(5.0, 15.0);
        GasStation { logo : Rc::from(logo), gas_cost }
    }

    pub fn get_cost(&self, amount : u32) -> f32 {
        return self.gas_cost * amount as f32;
    }

    pub fn buy_gas(&mut self, amount : u32, player : &mut Player) {
        player.money -= self.gas_cost * amount as f32;
        player.gas_level += amount;

        println!("money : {} gas : {}", player.money, player.gas_level);
    }
}
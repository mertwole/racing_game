use std::rc::Rc;

use rand::{Rng, rngs::StdRng};
use image::{RgbaImage};

use super::*;
use crate::game::{Percent, player::Player};

#[readonly::make]
#[derive(Clone)]
pub struct GasStation {
    pub logo : Rc<RgbaImage>,
    pub gas_cost : f32,
    pub discount : Percent
}   

impl GasStation {
    pub fn generate(logo : RgbaImage, rng : &mut StdRng) -> GasStation {
        let gas_cost = rng.gen_range(5.0, 15.0);
        GasStation { logo : Rc::from(logo), gas_cost, discount : Percent(0.0) }
    }

    pub fn get_max_gas_amount(&self, money : f32) -> u32 {
        (money / self.gas_cost).floor() as u32
    }

    pub fn get_cost(&self, amount : u32) -> f32 {
        return self.gas_cost * amount as f32;
    }

    pub fn buy_gas(&mut self, amount : u32, player : &mut Player) {
        player.money -= self.gas_cost * amount as f32;
        player.gas_level += amount as f32;
        self.discount.0 += amount as f32 * 0.1;
        if self.discount.0 > 50.0 { self.discount.0 = 50.0; }

        println!("money : {} gas : {} discount : {}", player.money, player.gas_level, self.discount.0);
    }
}

impl Service for GasStation { 
    fn get_logo(&self) -> Rc<RgbaImage> {
        self.logo.clone()
    }

    fn get_ref_type(&self) -> ServiceType { ServiceType::GasStation }
    fn get_type() -> ServiceType { ServiceType::GasStation }
}
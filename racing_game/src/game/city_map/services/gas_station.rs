use std::rc::Rc;

use rand::{Rng, rngs::StdRng};
use image::{RgbaImage};

pub struct GasStation {
    pub logo : Rc<RgbaImage>,
    pub gas_cost : u32
}   

impl GasStation {
    pub fn generate(logo : RgbaImage, rng : &mut StdRng) -> GasStation {
        let gas_cost = rng.gen_range(5, 15);
        GasStation { logo : Rc::from(logo) , gas_cost }
    }

    pub fn buy_gas(&mut self, amount : u32, money : &mut u32) -> bool {
        let cost = self.gas_cost * amount;
        if cost > *money { return false; }
        *money -= cost;
        true
    }
}
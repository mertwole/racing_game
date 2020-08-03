use image::{RgbaImage};

pub struct GasStation {
    logo : RgbaImage,
    gas_cost : u32
}   

impl GasStation {
    pub fn new(logo : RgbaImage) -> GasStation {
        GasStation { logo, gas_cost : 10 }
    }

    pub fn buy_gas(&mut self, amount : u32, money : &mut u32) -> bool {
        let cost = self.gas_cost * amount;
        if cost > *money { return false; }
        *money -= cost;
        true
    }
}
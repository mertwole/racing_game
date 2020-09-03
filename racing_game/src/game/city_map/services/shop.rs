use std::rc::Rc;

use image::RgbaImage;

use super::*;
use crate::game::Player;

pub enum ProductType {
    Water(f32),
    Soda(f32),
    Food(f32)
}

pub struct ShopProduct {
    product_type : ProductType,
    cost : f32
}

pub struct Shop {
    logo : Rc<RgbaImage>,
    pub assortment : Vec<ShopProduct>
}

impl Shop {
    pub fn generate(logo : RgbaImage, rng : &mut StdRng) -> Shop {
        let assortment = vec![
            ShopProduct { product_type : ProductType::Water(0.5), cost : 1.0 },
            ShopProduct { product_type : ProductType::Soda(1.0), cost : 2.0 },
            ShopProduct { product_type : ProductType::Food(1.0), cost : 3.0 }
        ];

        Shop { logo : Rc::from(logo), assortment }
    }

    pub fn buy_product(&self, id : usize, player : &mut Player) {
        let product = &self.assortment[id];

        match product.product_type {
            ProductType::Water(size) => { player.thirst.sub(Percent(size * 20.0)); }
            ProductType::Soda(size) => { player.thirst.sub(Percent(size * 10.0)); player.hunger.sub(Percent(size * 5.0)); }
            ProductType::Food(size) => { player.hunger.sub(Percent(size * 20.0)); }
        }

        player.money -= product.cost;
    }
}

impl Service for Shop {
    fn get_logo(&self) -> Rc<RgbaImage> {
        self.logo.clone()
    }

    fn get_ref_type(&self) -> ServiceType { ServiceType::Shop }
    fn get_type() -> ServiceType { ServiceType::Shop }
}
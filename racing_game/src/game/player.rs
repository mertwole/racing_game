use crate::game::{Percent};

#[derive(Clone)]
pub struct Player {
    pub money : f32,
    pub hunger : Percent,
    pub thirst : Percent,
    pub tireness : Percent,
    
    pub oil_level : u32,
    pub max_oil_level : u32,

    pub gas_level : u32,
    pub max_gas_level : u32,

    pub car_damage : Percent,
}

impl Player {
    pub fn new() -> Player {
        Player {
            money : 100.0,
            hunger : Percent(0.0),
            thirst : Percent(0.0),
            tireness : Percent(0.0),

            oil_level : 0,
            max_oil_level : 100,

            gas_level : 0,
            max_gas_level : 100,

            car_damage : Percent(0.0)
        }
    }
}
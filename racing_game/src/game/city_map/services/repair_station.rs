use std::rc::Rc;
use std::collections::HashMap;

use image::RgbaImage;

use super::*;
use crate::game::Percent;
use crate::game::ride::car::*;

pub struct RepairStation {
    logo : Rc<RgbaImage>,
    diagnosis_cost : f32,
    repair_costs : HashMap<CarSystem, f32>
}   

impl RepairStation {
    pub fn generate(logo : RgbaImage, rng : &mut StdRng) -> RepairStation {
        let diagnosis_cost = 10.0;
        let mut repair_costs = HashMap::<CarSystem, f32>::new();
        repair_costs.insert(CarSystem::Wheels, 5.0);
        repair_costs.insert(CarSystem::Transmission, 5.0);
        repair_costs.insert(CarSystem::Chase, 5.0);
        repair_costs.insert(CarSystem::Engine, 5.0);
        repair_costs.insert(CarSystem::Brake, 5.0);
        repair_costs.insert(CarSystem::Starter, 5.0);

        RepairStation { logo : Rc::from(logo), diagnosis_cost, repair_costs }
    }

    pub fn fix(&self, car_system : CarSystem, to_fix : Percent, player : &mut Player, car : &mut Car) {
        let repair_cost = self.repair_costs.get(&car_system).unwrap() * to_fix.to_norm();
        player.money -= repair_cost;
        car.fix_system(car_system, to_fix);
    }

    pub fn get_diagnosis_cost(&self) -> f32 {
        self.diagnosis_cost
    }
}

impl Service for RepairStation {
    fn get_logo(&self) -> Rc<RgbaImage> {
        self.logo.clone()
    }

    fn get_ref_type(&self) -> ServiceType { ServiceType::RepairStation }
    fn get_type() -> ServiceType { ServiceType::RepairStation }
}
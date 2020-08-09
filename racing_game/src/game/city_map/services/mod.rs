use std::collections::HashMap;
use std::rc::Rc;

use image::RgbaImage;
use rand::{rngs::StdRng};

use crate::{Game, game::player::Player};

mod gas_station;
mod hostel;
mod repair_station;
mod shop;

pub use gas_station::*;
pub use hostel::*;
pub use repair_station::*;
pub use shop::*;

pub enum ServiceAction{
    BuyGas(u32)
}

pub trait Service {
    fn get_logo(&self) -> Rc<RgbaImage>;

    fn get_type() -> ServiceType where Self : Sized;
    fn get_ref_type(&self) -> ServiceType;
}

#[derive(Clone, Copy)]
pub struct ServiceId(pub usize);

#[derive(PartialEq, Eq, Hash)]
pub enum ServiceType {
    GasStation,
    Hostel,
    RepairStation,
    Shop
}

pub struct CityServicesSubset {
    service_ids : HashMap<ServiceType, Vec<ServiceId>>
}

impl CityServicesSubset {
    pub fn get_of_type<T>(&self) -> Vec<ServiceId> where T : Sized + 'static + Service {
        self.service_ids.get(&T::get_type()).unwrap().clone()
    }   
}

pub struct Services {
    services : HashMap<ServiceType, Vec<Box<dyn Service>>>
}

impl Services {
    pub fn generate(rng : &mut StdRng) -> Services {
        let mut gas_stations = Vec::new();
        let gs0_logo = Game::load_image_rgba("logo0.png");
        let gas_station0 = GasStation::generate(gs0_logo, rng);
        gas_stations.push(Box::<dyn Service>::from(Box::from(gas_station0)));
        let gs1_logo = Game::load_image_rgba("logo1.png");
        let gas_station1 = GasStation::generate(gs1_logo, rng);
        gas_stations.push(Box::<dyn Service>::from(Box::from(gas_station1)));

        let mut services = HashMap::new();
        services.insert(ServiceType::GasStation, gas_stations);
        services.insert(ServiceType::Hostel, Vec::new());
        services.insert(ServiceType::RepairStation, Vec::new());
        services.insert(ServiceType::Shop, Vec::new());

        Services { services }
    }

    pub fn generate_subsets(&self, city_count : usize, rng : &mut StdRng) -> Vec<CityServicesSubset> {
        let mut subsets : Vec<CityServicesSubset> = Vec::with_capacity(city_count);

        for _i in 0..city_count {
            let mut service_ids = HashMap::new();
            service_ids.insert(ServiceType::GasStation, vec![ServiceId(0), ServiceId(1)]);
            service_ids.insert(ServiceType::Hostel, Vec::new());
            service_ids.insert(ServiceType::RepairStation, Vec::new());
            service_ids.insert(ServiceType::Shop, Vec::new());

            subsets.push(CityServicesSubset { service_ids });
        }

        subsets
    }

    pub fn process_action(&mut self, id : ServiceId, action : ServiceAction, player : &mut Player) {
        match action {
            ServiceAction::BuyGas(amount) => { self.get_service_mut::<GasStation>(id).buy_gas(amount, player); }
        }
    }  

    fn get_service_mut<T>(&mut self, id : ServiceId) -> &mut T where T : Sized + 'static + Service {
        unsafe {
            &mut *(self.services.get_mut(&T::get_type()).unwrap()[id.0].as_mut() as *mut dyn Service as *mut T)
        }
    }

    pub fn get_service<T>(&self, id : ServiceId) -> &T where T : Sized + 'static + Service {
        unsafe {
            &*(self.services.get(&T::get_type()).unwrap()[id.0].as_ref() as *const dyn Service as *const T)
        }
    }
}
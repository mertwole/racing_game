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
    BuyGas(u32),
    RestInHostel(u32)
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
        for i in 0..7 {
            let gs_logo = Game::load_image_rgba(&*format!("logos/gas_stations/logo{}.png", i));
            let gs = GasStation::generate(gs_logo, rng);
            gas_stations.push(Box::<dyn Service>::from(Box::from(gs)));
        }

        let mut hostels = Vec::new();
        for i in 0..7 {
            let h_logo = Game::load_image_rgba(&*format!("logos/hostels/logo{}.png", i));
            let h = Hostel::generate(h_logo, rng);
            hostels.push(Box::<dyn Service>::from(Box::from(h)));
        }

        let mut services = HashMap::new();
        services.insert(ServiceType::GasStation, gas_stations);
        services.insert(ServiceType::Hostel, hostels);
        services.insert(ServiceType::RepairStation, Vec::new());
        services.insert(ServiceType::Shop, Vec::new());

        Services { services }
    }

    pub fn generate_subsets(&self, city_count : usize, rng : &mut StdRng) -> Vec<CityServicesSubset> {
        let mut subsets : Vec<CityServicesSubset> = Vec::with_capacity(city_count);

        for _i in 0..city_count {
            let mut service_ids = HashMap::new();
            service_ids.insert(ServiceType::GasStation, vec![ServiceId(0), ServiceId(1), ServiceId(2), ServiceId(3), ServiceId(4), ServiceId(5), ServiceId(6)]);
            service_ids.insert(ServiceType::Hostel, vec![ServiceId(0), ServiceId(1), ServiceId(2), ServiceId(3), ServiceId(4), ServiceId(5), ServiceId(6)]);
            service_ids.insert(ServiceType::RepairStation, Vec::new());
            service_ids.insert(ServiceType::Shop, Vec::new());

            subsets.push(CityServicesSubset { service_ids });
        }

        subsets
    }

    pub fn process_action(&mut self, id : ServiceId, action : ServiceAction, player : &mut Player) {
        match action {
            ServiceAction::BuyGas(amount) => { self.get_service_mut::<GasStation>(id).buy_gas(amount, player); }
            ServiceAction::RestInHostel(option_id) => { self.get_service_mut::<Hostel>(id).rest(option_id, player); }
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
use std::collections::HashMap;
use rand::Rng;
use std::rc::Rc;

use image::RgbaImage;
use rand::{rngs::StdRng};

use crate::Game;
use crate::game::Percent;
use crate::game::player::Player;
use crate::game::ride::car::*;

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
    RestInHostel(u32),
    FixCarSystem(CarSystem, Percent),
    BuyProduct(usize)
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

pub struct ServicesSubsetProperties {
    pub gas_station_count : usize,
    pub hostel_count : usize,
    pub repair_station_count : usize,
    pub shop_count : usize
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

        let mut repair_stations = Vec::new();
        for i in 0..7 {
            let rs_logo = Game::load_image_rgba(&*format!("logos/hostels/logo{}.png", i));
            let rs = RepairStation::generate(rs_logo, rng);
            repair_stations.push(Box::<dyn Service>::from(Box::from(rs)));
        }

        let mut shops = Vec::new();
        for i in 0..7 {
            let sh_logo = Game::load_image_rgba(&*format!("logos/hostels/logo{}.png", i));
            let sh = Shop::generate(sh_logo, rng);
            shops.push(Box::<dyn Service>::from(Box::from(sh)));
        }

        let mut services = HashMap::new();
        services.insert(ServiceType::GasStation, gas_stations);
        services.insert(ServiceType::Hostel, hostels);
        services.insert(ServiceType::RepairStation, repair_stations);
        services.insert(ServiceType::Shop, shops);

        Services { services }
    }

    fn generate_subset_concrete_service(&self, subset : &mut CityServicesSubset, service_type : ServiceType, count : usize, rng : &mut StdRng) {
        let mut ids : Vec<ServiceId> = Vec::new();
        let services_of_type = self.services.get(&service_type).unwrap();
        for _i in 0..count {
            'outer : loop {
                let id = rng.gen_range(0, services_of_type.len());
                for existing_id in &ids {
                    if existing_id.0 == id { continue 'outer; }
                }
                ids.push(ServiceId { 0 : id });
                break;
            }
        }

        subset.service_ids.insert(service_type, ids);
    }

    pub fn generate_subset(&self, properties : ServicesSubsetProperties, rng : &mut StdRng) -> CityServicesSubset {
        let mut subset = CityServicesSubset { service_ids : HashMap::new() };

        self.generate_subset_concrete_service(&mut subset, ServiceType::GasStation,     properties.gas_station_count,       rng);
        self.generate_subset_concrete_service(&mut subset, ServiceType::Hostel,         properties.hostel_count,            rng);
        self.generate_subset_concrete_service(&mut subset, ServiceType::RepairStation,  properties.repair_station_count,    rng);
        self.generate_subset_concrete_service(&mut subset, ServiceType::Shop,           properties.shop_count,              rng);

        subset
    }

    pub fn process_action(&mut self, id : ServiceId, action : ServiceAction, player : &mut Player, car : &mut Car) {
        match action {
            ServiceAction::BuyGas(amount) => { self.get_service_mut::<GasStation>(id).buy_gas(amount, player); }
            ServiceAction::RestInHostel(option_id) => { self.get_service_mut::<Hostel>(id).rest(option_id, player); }
            ServiceAction::FixCarSystem(system, percent) => { self.get_service_mut::<RepairStation>(id).fix(system, percent, player, car); }
            ServiceAction::BuyProduct(product_id) => { self.get_service_mut::<Shop>(id).buy_product(product_id, player); }
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
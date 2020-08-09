use std::rc::Rc;
use std::any::TypeId;

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
}

#[derive(Clone, Copy)]
pub struct ServiceId(pub usize);

#[derive(Clone)]
pub struct CityServicesSubset {
    pub gas_station_ids : Vec<ServiceId>,
    pub hostel_ids : Vec<ServiceId>,
    pub repair_station_ids : Vec<ServiceId>,
    pub shop_ids : Vec<ServiceId>
}

impl CityServicesSubset {
    pub fn get_of_type<T>(&self) -> Vec<ServiceId> where T : Sized + 'static + Service {
        let gas_station_type_id : TypeId = TypeId::of::<GasStation>();
        let hostel_type_id : TypeId = TypeId::of::<Hostel>();
        let repair_station_type_id : TypeId = TypeId::of::<RepairStation>();
        let shop_type_id : TypeId = TypeId::of::<Shop>();

        match TypeId::of::<T>() {
            gas_station_type_id => { self.gas_station_ids.clone() }
            hostel_type_id => { self.hostel_ids.clone() }
            repair_station_type_id => { self.repair_station_ids.clone() }
            shop_type_id => { self.shop_ids.clone() }
            _ => { panic!("Incorrect service type!"); }
        }
    }   
}

pub struct Services {
    pub gas_stations : Vec<GasStation>,
    pub hostels : Vec<Hostel>,
    pub repair_stations : Vec<RepairStation>,
    pub shops : Vec<Shop>
}

impl Services {
    pub fn generate(rng : &mut StdRng) -> Services {
        let mut gas_stations = Vec::new();
        let mut hostels = Vec::new();
        let mut repair_stations = Vec::new();
        let mut shops = Vec::new();

        let gs0_logo = Game::load_image_rgba("logo0.png");
        let gas_station0 = GasStation::generate(gs0_logo, rng);
        gas_stations.push(gas_station0);
        let gs1_logo = Game::load_image_rgba("logo1.png");
        let gas_station1 = GasStation::generate(gs1_logo, rng);
        gas_stations.push(gas_station1);

        Services { gas_stations, hostels, repair_stations, shops }
    }

    pub fn generate_subsets(&self, city_count : usize, rng : &mut StdRng) -> Vec<CityServicesSubset> {
        let mut subsets : Vec<CityServicesSubset> = Vec::with_capacity(city_count);

        for _i in 0..city_count {
            subsets.push(CityServicesSubset { 
                gas_station_ids : vec![ServiceId(0), ServiceId(1)], 
                hostel_ids : Vec::new(), 
                repair_station_ids : Vec::new(), 
                shop_ids : Vec::new() 
            });
        }

        subsets
    }

    pub fn process_action(&mut self, id : ServiceId, action : ServiceAction, player : &mut Player) {
        match action {
            ServiceAction::BuyGas(amount) => { self.gas_stations[id.0].buy_gas(amount, player); }
        }
    }  
    
    pub fn get_service<T>(&self, id : ServiceId) -> &T where T : Sized + 'static + Service {
        let gas_station_type_id : TypeId = TypeId::of::<GasStation>();
        let hostel_type_id : TypeId = TypeId::of::<Hostel>();
        let repair_station_type_id : TypeId = TypeId::of::<RepairStation>();
        let shop_type_id : TypeId = TypeId::of::<Shop>();

        unsafe { 
            &*match TypeId::of::<T>() {
                gas_station_type_id => { &self.gas_stations[id.0] as *const GasStation as *const T}
                hostel_type_id => { &self.hostels[id.0] as *const Hostel as *const T}
                repair_station_type_id => { &self.repair_stations[id.0] as *const RepairStation as *const T}
                shop_type_id => { &self.shops[id.0] as *const Shop as *const T}
                _ => { panic!("Incorrect service type!"); }
            } 
        }
    }
}
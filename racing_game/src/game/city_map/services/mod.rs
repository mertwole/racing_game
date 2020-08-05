use std::rc::Rc;

use rand::{RngCore, rngs::StdRng};

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
    BuyGas(u32, ServiceId)
}

#[derive(Clone, Copy)]
pub struct ServiceId(usize);

pub struct CityServicesSubset {
    gas_station_ids : Vec<ServiceId>,
    hostel_ids : Vec<ServiceId>,
    repair_station_ids : Vec<ServiceId>,
    shop_ids : Vec<ServiceId>
}

pub struct Services {
    gas_stations : Vec<GasStation>,
    hostels : Vec<Hostel>,
    repair_stations : Vec<RepairStation>,
    shops : Vec<Shop>
}

pub struct ServiceReferences<'a> {
    pub gas_stations : Vec<(ServiceId, &'a GasStation)>,
    pub hostels : Vec<(ServiceId, &'a Hostel)>,
    pub repair_stations : Vec<(ServiceId, &'a RepairStation)>,
    pub shops : Vec<(ServiceId, &'a Shop)>
}

impl Services {
    pub fn generate(rng : &mut StdRng) -> Services {
        let mut gas_stations = Vec::new();
        let mut hostels = Vec::new();
        let mut repair_stations = Vec::new();
        let mut shops = Vec::new();

        let gs_logo = Game::load_image_rgba("logo.png");
        let gas_station = GasStation::generate(gs_logo, rng);
        gas_stations.push(gas_station);

        Services { gas_stations, hostels, repair_stations, shops }
    }

    pub fn generate_subsets(&self, city_count : usize, rng : &mut StdRng) -> Vec<CityServicesSubset> {
        let mut subsets : Vec<CityServicesSubset> = Vec::with_capacity(city_count);

        for _i in 0..city_count {
            subsets.push(CityServicesSubset { 
                gas_station_ids : vec![ServiceId(0)], 
                hostel_ids : Vec::new(), 
                repair_station_ids : Vec::new(), 
                shop_ids : Vec::new() 
            });
        }

        subsets
    }

    pub fn get_subset_services(&self, subset : &CityServicesSubset) -> ServiceReferences {
        let mut gas_stations : Vec<(ServiceId, &GasStation)> = Vec::with_capacity(subset.gas_station_ids.len());

        for &gas_station_id in &subset.gas_station_ids {
            unsafe {
                gas_stations.push((gas_station_id, &self.gas_stations[gas_station_id.0]));
            }
        }

        ServiceReferences {
            gas_stations,
            hostels : Vec::new(),
            repair_stations : Vec::new(),
            shops : Vec::new()
        }
    }

    pub fn process_action(&mut self, action : ServiceAction, player : &mut Player) {
        match action {
            ServiceAction::BuyGas(amount, id) => { self.gas_stations[id.0].buy_gas(amount, player); }
        }
    }   
}
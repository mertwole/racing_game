use rand::{RngCore, rngs::StdRng};

use crate::Game;

mod gas_station;
mod hostel;
mod repair_station;
mod shop;

pub use gas_station::*;
pub use hostel::*;
pub use repair_station::*;
pub use shop::*;

pub struct CityServicesSubset {
    gas_station_ids : Vec<usize>,
    hostel_ids : Vec<usize>,
    repair_station_ids : Vec<usize>,
    shop_ids : Vec<usize>
}

pub struct Services {
    gas_stations : Vec<GasStation>,
    hostels : Vec<Hostel>,
    repair_stations : Vec<RepairStation>,
    shops : Vec<Shop>
}

pub struct ServiceReferences<'a> {
    pub gas_stations : Vec<&'a GasStation>,
    pub hostels : Vec<&'a Hostel>,
    pub repair_stations : Vec<&'a RepairStation>,
    pub shops : Vec<&'a Shop>
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
                gas_station_ids : vec![0], 
                hostel_ids : Vec::new(), 
                repair_station_ids : Vec::new(), 
                shop_ids : Vec::new() 
            });
        }

        subsets
    }

    pub fn get_subset_services(&self, subset : &CityServicesSubset) -> ServiceReferences {
        let mut gas_stations : Vec<&GasStation> = Vec::with_capacity(subset.gas_station_ids.len());

        for &gas_station_id in &subset.gas_station_ids {
            gas_stations.push(&self.gas_stations[gas_station_id]);
        }

        ServiceReferences {
            gas_stations,
            hostels : Vec::new(),
            repair_stations : Vec::new(),
            shops : Vec::new()
        }
    }
}